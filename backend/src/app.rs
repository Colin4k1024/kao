use axum::{extract::State, routing::{get, post, put, delete}, Router, response::{IntoResponse, Response}, http::HeaderMap as AxumHeaderMap, http::HeaderName, http::HeaderValue, http::Method};
use sqlx::PgPool;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_cron_scheduler::JobScheduler;
use crate::config::Settings;
use crate::features::monitoring::routes::monitoring_router;
use crate::features::monitoring::metrics as monitoring_metrics;
use crate::middleware::logger::request_logger;
use crate::middleware::cors::init_cors;
use crate::middleware::activity_tracker::track_activity;
use crate::middleware::auth::auth_middleware;
use crate::middleware::rate_limit::login_rate_limit_middleware;
use crate::middleware::security_headers::security_headers_middleware;
use crate::middleware::swagger_protection::swagger_protection_middleware;
use crate::common::middleware::openapi::setup_openapi_middleware;
use crate::common::cache::redis::RedisCache;
use crate::features::auth::routes::auth_routes;
use crate::features::users::routes::user_routes;
use crate::features::departments::routes::department_routes;
use crate::features::roles::routes::role_routes;
use crate::features::menus::routes::menu_routes;
use crate::features::config::routes::config_routes;
use crate::features::notice::routes::notice_routes;
use crate::features::dictionary::r#type::routes::type_routes;
use crate::features::dictionary::data::routes::data_routes;
use crate::features::job;
use crate::features::posts::routes::post_routes;

/// Handle CORS preflight requests directly
async fn cors_preflight_handler(req: axum::extract::Request, next: axum::middleware::Next) -> Result<impl IntoResponse, axum::http::StatusCode> {
    if req.method() == Method::OPTIONS {
        // Check if this is a preflight request
        if req.headers().contains_key("access-control-request-method") {
            let mut response = axum::http::StatusCode::NO_CONTENT.into_response();
            // Add CORS headers to preflight response
            response.headers_mut().insert(
                HeaderName::from_static("access-control-allow-origin"),
                HeaderValue::from_static("http://localhost:3000"),
            );
            response.headers_mut().insert(
                HeaderName::from_static("access-control-allow-methods"),
                HeaderValue::from_static("GET, POST, PUT, DELETE, PATCH, OPTIONS"),
            );
            response.headers_mut().insert(
                HeaderName::from_static("access-control-allow-headers"),
                HeaderValue::from_static("Content-Type, Authorization"),
            );
            response.headers_mut().insert(
                HeaderName::from_static("access-control-allow-credentials"),
                HeaderValue::from_static("true"),
            );
            response.headers_mut().insert(
                HeaderName::from_static("access-control-max-age"),
                HeaderValue::from_static("3600"),
            );
            return Ok(response);
        }
    }
    
    // For non-OPTIONS requests, continue to next middleware
    Ok(next.run(req).await)
}

pub fn create_app(
    pool: PgPool,
    settings: Settings,
    cache: RedisCache,
    running_jobs: Arc<RwLock<std::collections::HashSet<i64>>>,
    scheduler: Arc<JobScheduler>,
) -> Router {
    let state = AppState {
        pool,
        settings,
        cache,
        running_jobs,
        scheduler,
    };

    // System routes under /api/system (config, notice, dictionary)
    let system_router = Router::new()
        .nest("/", config_routes())
        .nest("/", notice_routes())
        .nest("/", type_routes())
        .nest("/", data_routes());

    // Build base router without middleware
    let base_router = Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // Prometheus metrics endpoint at root for easy scraping
        .route("/metrics", get(monitoring_metrics::get_metrics))
        // Auth routes at /api/v1 (login, register, profile, session)
        // Apply rate limiting to auth routes for login protection
        .nest("/api/v1", auth_routes().layer(axum::middleware::from_fn(login_rate_limit_middleware())))
        // API v1 routes (users, departments, roles, menus, posts)
        .nest("/api/v1", user_routes())
        .nest("/api/v1", department_routes())
        .nest("/api/v1", role_routes())
        .nest("/api/v1", menu_routes())
        .nest("/api/v1", post_routes())
        // System management routes
        .nest("/api/system", system_router)
        // Job routes - registered directly to avoid nested router issue
        .route("/api/jobs", get(job::routes::list_jobs))
        .route("/api/jobs", post(job::routes::create_job))
        .route("/api/jobs/:id", get(job::routes::get_job))
        .route("/api/jobs/:id", put(job::routes::update_job))
        .route("/api/jobs/:id", delete(job::routes::delete_job))
        .route("/api/jobs/:id/schedule", put(job::routes::schedule_job))
        .route("/api/jobs/:id/unschedule", put(job::routes::unschedule_job))
        .route("/api/jobs/:id/run", post(job::routes::run_job))
        .route("/api/jobs/logs", get(job::routes::list_job_logs))
        .route("/api/jobs/logs/clear", delete(job::routes::clear_job_logs))
        .route("/api/jobs/logs/:id", get(job::routes::get_job_log))
        // Monitoring routes under /api/monitoring
        .nest("/api/monitoring", monitoring_router())
        .with_state(state.clone());

    let app = Router::new()
        // First: Handle CORS preflight OPTIONS requests directly
        .layer(axum::middleware::from_fn(cors_preflight_handler))
        // Apply CORS middleware (handles actual CORS headers)
        .layer(init_cors())
        // Apply request logging middleware to all routes
        .layer(axum::middleware::from_fn(request_logger))
        // Apply authentication middleware for JWT validation
        .layer(axum::middleware::from_fn_with_state(state.clone(), auth_middleware))
        // Apply activity tracking middleware for online user updates
        .layer(axum::middleware::from_fn(track_activity))
        // Apply security headers middleware
        // .layer(axum::middleware::from_fn(security_headers_middleware))
        // Apply Swagger protection middleware
        // .layer(axum::middleware::from_fn(swagger_protection_middleware))
        .nest("/", base_router);

    // Apply OpenAPI/Swagger UI middleware (after state is set)
    setup_openapi_middleware(app)
}

/// Health check response structure
#[derive(Serialize)]
struct HealthResponse {
    status: String,
    checks: HealthChecks,
    timestamp: String,
}

#[derive(Serialize)]
struct HealthChecks {
    database: String,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub settings: Settings,
    pub cache: RedisCache,
    /// Running job IDs for the scheduler
    pub running_jobs: Arc<RwLock<std::collections::HashSet<i64>>>,
    /// The job scheduler for cron-based job execution
    pub scheduler: Arc<JobScheduler>,
}

/// Health check endpoint with dependency status
async fn health_check(State(state): State<AppState>) -> Response {
    let request_id = uuid::Uuid::new_v4().to_string();
    let mut db_status = "ok".to_string();
    let mut overall_status = "healthy".to_string();

    // Check database connection
    match sqlx::query("SELECT 1").execute(&state.pool).await {
        Ok(_) => {
            db_status = "ok".to_string();
        }
        Err(e) => {
            tracing::error!("Health check database error: {}", e);
            db_status = "error".to_string();
            overall_status = "unhealthy".to_string();
        }
    }

    let response = HealthResponse {
        status: overall_status.clone(),
        checks: HealthChecks {
            database: db_status,
        },
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let body = serde_json::to_string(&response).unwrap_or_else(|_| r#"{"status":"error"}"#.to_string());

    let status_code = if overall_status == "healthy" {
        axum::http::StatusCode::OK
    } else {
        axum::http::StatusCode::SERVICE_UNAVAILABLE
    };

    let mut headers = AxumHeaderMap::new();
    headers.insert(
        HeaderName::from_static("content-type"),
        HeaderValue::from_static("application/json"),
    );
    headers.insert(
        HeaderName::from_static("x-request-id"),
        HeaderValue::from_str(&request_id).unwrap_or_else(|_| HeaderValue::from_static("unknown")),
    );

    (status_code, headers, body).into_response()
}
