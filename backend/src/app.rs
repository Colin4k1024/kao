use axum::{
    routing::{get, post},
    Router,
    http::{Method, header},
};
use tower_http::cors::{CorsLayer, Any};
use sqlx::PgPool;
use crate::config::Settings;
use axum::{extract::State, Json};

pub async fn create_app(pool: PgPool, settings: Settings) -> Router {
    let state = AppState { pool, settings };
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
    
    Router::new()
        .route("/health", get(health_check))
        .route("/api/auth/login", post(login))
        .route("/api/auth/register", post(register))
        .route("/api/auth/logout", post(logout))
        .layer(cors)
        .with_state(state)
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
