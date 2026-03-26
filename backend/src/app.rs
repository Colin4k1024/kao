use axum::{
    routing::{get, post},
    Router,
    http::{Method, header},
};
use tower_http::cors::{CorsLayer, Any};
use sqlx::PgPool;
use crate::config::Settings;
use axum::{extract::State, Json};
use crate::features::monitoring::monitoring_router;
use crate::features::dictionary::r#type::routes::type_routes;
use crate::features::dictionary::data::routes::data_routes;
use crate::features::config::routes::config_routes;
use crate::features::notice::routes::notice_routes;

pub async fn create_app(pool: PgPool, settings: Settings) -> Router {
    let state = AppState { pool, settings };
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
    
    Router::new()
        .nest("/system/monitor", monitoring_router())
        .nest("/api/system/dictionary", type_routes())
        .nest("/api/system/dictionary", data_routes())
        .nest("/api/system/config", config_routes())
        .nest("/api/system/notice", notice_routes())
        .route("/api-docs", get(redirect_to_swagger))
        .route("/api-docs/openapi.yaml", get(openapi_spec))
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/api/auth/logout", post(logout))
        .layer(cors)
        .with_state(state)
}

/// Redirect to Swagger UI
async fn redirect_to_swagger() -> axum::http::StatusCode {
    axum::http::StatusCode::PERMANENT_REDIRECT
}

/// Return OpenAPI specification
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

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub settings: Settings,
}

async fn health_check() -> &'static str {
    "OK"
}

async fn login(
    State(_state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> Json<serde_json::Value> {
    let username = req.get("username").and_then(|v| v.as_str()).unwrap_or("");
    let password = req.get("password").and_then(|v| v.as_str()).unwrap_or("");

    if username == "admin" && password == "admin123" {
        let token = format!("token_{}", uuid::Uuid::new_v4());
        Json(serde_json::json!({
            "code": 200,
            "message": "登录成功",
            "data": {
                "access_token": token,
                "refresh_token": token,
                "token_type": "Bearer",
                "expires_in": 3600,
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
    } else {
        Json(serde_json::json!({
            "code": 401,
            "message": "用户名或密码错误",
            "data": serde_json::Value::Null
        }))
    }
}

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

async fn logout() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "退出成功",
        "data": serde_json::Value::Null
    }))
}
