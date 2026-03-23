use crate::app::state::AppState;
use crate::common::{
    auth::{extractor::CurrentClaims, middleware::auth_middleware},
    response::ApiResponse,
};
use crate::features::{auth, departments, menus, roles, users};
use axum::{middleware::from_fn_with_state, routing::get, Json, Router};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub async fn create_app(
    config: crate::common::config::AppConfig,
) -> Result<Router, crate::common::error::AppError> {
    let state = AppState::bootstrap(config).await?;
    Ok(build_router(state))
}

pub fn build_router(state: AppState) -> Router {
    let protected_state = state.clone();

    let public_router = Router::new().route("/api/v1/health", get(health));
    let protected_router = Router::new()
        .route("/api/v1/protected/ping", get(protected_ping))
        .layer(from_fn_with_state(protected_state, auth_middleware));

    Router::new()
        .merge(public_router)
        .merge(auth::router(state.clone()))
        .merge(users::router(state.clone()))
        .merge(roles::router(state.clone()))
        .merge(departments::router(state.clone()))
        .merge(menus::router(state.clone()))
        .merge(protected_router)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

async fn health() -> Json<ApiResponse<HealthResponse>> {
    Json(ApiResponse::success(HealthResponse {
        status: "ok",
        service: "ai-coding-backend",
    }))
}

async fn protected_ping(CurrentClaims(claims): CurrentClaims) -> Json<ApiResponse<ProtectedPing>> {
    Json(ApiResponse::success(ProtectedPing {
        subject: claims.sub,
        department_id: claims.dept_id,
        roles: claims.roles,
        permissions: claims.permissions,
    }))
}

#[derive(Debug, serde::Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, serde::Serialize)]
struct ProtectedPing {
    subject: String,
    department_id: Option<String>,
    roles: Vec<String>,
    permissions: Vec<String>,
}
