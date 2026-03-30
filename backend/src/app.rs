use axum::{extract::State, routing::get, Json, Router, response::{IntoResponse, Response}, http::HeaderMap as AxumHeaderMap, http::HeaderName, http::HeaderValue};
use sqlx::PgPool;
use serde::Serialize;
use crate::config::Settings;
use crate::features::monitoring::routes::monitoring_router;
use crate::features::monitoring::metrics as monitoring_metrics;
use crate::middleware::logger::request_logger;
use crate::middleware::cors::init_cors;
use crate::features::auth::routes::auth_routes;
use crate::features::users::routes::user_routes;
use crate::features::departments::routes::department_routes;
use crate::features::roles::routes::role_routes;
use crate::features::menus::routes::menu_routes;
use crate::features::config::routes::config_routes;
use crate::features::notice::routes::notice_routes;
use crate::features::dictionary::r#type::routes::type_routes;
use crate::features::dictionary::data::routes::data_routes;

pub fn create_app(pool: PgPool, settings: Settings) -> Router {
    let state = AppState { pool, settings };

    // System routes under /api/system (config, notice, dictionary)
    let system_router = Router::new()
        .nest("/", config_routes())
        .nest("/", notice_routes())
        .nest("/", type_routes())
        .nest("/", data_routes());

    Router::new()
        // Health check endpoint
        .route("/health", get(health_check))
        // Prometheus metrics endpoint at root for easy scraping
        .route("/metrics", get(monitoring_metrics::get_metrics))
        // Auth routes at /api/v1 (login, register, profile, session)
        .nest("/api/v1", auth_routes())
        // API v1 routes (users, departments, roles, menus)
        .nest("/api/v1", user_routes())
        .nest("/api/v1", department_routes())
        .nest("/api/v1", role_routes())
        .nest("/api/v1", menu_routes())
        // System management routes
        .nest("/api/system", system_router)
        // Monitoring routes under /api/monitoring
        .nest("/api/monitoring", monitoring_router())
        // Apply CORS middleware
        .layer(init_cors())
        // Apply request logging middleware to all routes
        .layer(axum::middleware::from_fn(request_logger))
        .with_state(state)
}

/// Redirect to Swagger UI
#[allow(dead_code)]
async fn redirect_to_swagger() -> axum::http::StatusCode {
    axum::http::StatusCode::PERMANENT_REDIRECT
}

/// Return OpenAPI specification
#[allow(dead_code)]
async fn openapi_spec() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Kao Admin Management System API",
            "description": "Enterprise admin management system API documentation",
            "contact": {
                "name": "Kao Team",
                "url": "https://github.com/kao-admin/kao",
                "email": "team@kao-admin.com"
            },
            "version": "1.0.0"
        },
        "servers": [
            {
                "url": "http://localhost:8080",
                "description": "Development server"
            },
            {
                "url": "https://api.kao-admin.com",
                "description": "Production server"
            }
        ],
        "paths": {
            "/api/auth/login": {
                "post": {
                    "summary": "User login",
                    "description": "Authenticate user and return JWT token",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "username": {"type": "string"},
                                        "password": {"type": "string"}
                                    },
                                    "required": ["username", "password"]
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Login successful",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ApiResponse"
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Invalid credentials"
                        }
                    }
                }
            },
            "/api/auth/refresh": {
                "post": {
                    "summary": "Refresh access token",
                    "description": "Generate new access token from refresh token",
                    "requestBody": {
                        "required": true,
                        "content": {
                            "application/json": {
                                "schema": {
                                    "type": "object",
                                    "properties": {
                                        "refresh_token": {"type": "string"}
                                    },
                                    "required": ["refresh_token"]
                                }
                            }
                        }
                    },
                    "responses": {
                        "200": {
                            "description": "Token refreshed successfully",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "$ref": "#/components/schemas/ApiResponse"
                                    }
                                }
                            }
                        },
                        "401": {
                            "description": "Invalid refresh token"
                        }
                    }
                }
            },
            "/api/auth/logout": {
                "post": {
                    "summary": "User logout",
                    "description": "Logout current user",
                    "security": [{"Bearer": []}],
                    "responses": {
                        "200": {
                            "description": "Logout successful"
                        },
                        "401": {
                            "description": "Not authenticated"
                        }
                    }
                }
            },
            "/api/auth/session": {
                "get": {
                    "summary": "Get current user session",
                    "description": "Get current user profile and session info",
                    "security": [{"Bearer": []}],
                    "responses": {
                        "200": {
                            "description": "Session retrieved successfully"
                        },
                        "401": {
                            "description": "Not authenticated"
                        }
                    }
                }
            }
        },
        "components": {
            "securitySchemes": {
                "Bearer": {
                    "type": "http",
                    "scheme": "bearer",
                    "bearerFormat": "JWT"
                }
            },
            "schemas": {
                "ApiResponse": {
                    "type": "object",
                    "properties": {
                        "code": {"type": "integer"},
                        "message": {"type": "string"},
                        "data": {"type": "object", "nullable": true}
                    },
                    "required": ["code", "message"]
                }
            }
        }
    }))
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

// Login handler moved to backend/src/api/auth/handlers.rs
// Authentication is handled via the auth service with bcrypt + JWT

#[allow(dead_code)]
async fn register(
    State(_state): State<AppState>,
    Json(_req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "注册功能开发中",
        "data": serde_json::Value::Null
    }))
}

#[allow(dead_code)]
async fn logout() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "退出成功",
        "data": serde_json::Value::Null
    }))
}

#[allow(dead_code)]
async fn refresh(
    State(_state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let refresh_token = req.get("refresh_token").and_then(|v| v.as_str());
    
    match refresh_token {
        Some(_token) => {
            // In production, use token validation without DB lookup
            Json(serde_json::json!({
                "code": 200,
                "message": "Token refreshed successfully",
                "data": {
                    "access_token": format!("new_token_{}", uuid::Uuid::new_v4()),
                    "token_type": "Bearer",
                    "expires_in": 3600
                }
            }))
        }
        None => Json(serde_json::json!({
            "code": 400,
            "message": "Refresh token is required",
            "data": serde_json::Value::Null
        }))
    }
}

#[allow(dead_code)]
async fn get_session() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "Session retrieved successfully",
        "data": {
            "user": {
                "id": "00000000-0000-0000-0000-000000000001",
                "username": "admin",
                "nickname": "管理员",
                "email": "admin@example.com",
                "phone": "13800138000",
                "avatar": serde_json::Value::Null,
                "status": 1,
                "roles": ["admin"],
                "permissions": ["*"]
            }
        }
    }))
}
