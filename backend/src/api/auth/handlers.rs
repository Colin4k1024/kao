use axum::{extract::State, Json};
use bcrypt::{verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use crate::app::AppState;
use crate::common::auth::jwt::{generate_jwt, Claims};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub code: u32,
    pub message: String,
    pub data: Option<LoginData>,
}

#[derive(Debug, Serialize)]
pub struct LoginData {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub status: i32,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    let username = req.username.clone();

    // 查询用户，包括密码哈希用于bcrypt验证
    let user_result = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, Option<String>, Option<String>, i32)>(
        "SELECT id, username, password, nickname, email, phone, avatar, status FROM sys_user WHERE username = $1 AND deleted_at IS NULL"
    )
    .bind(&req.username)
    .fetch_optional(&state.pool)
    .await;

    match user_result {
        Ok(Some(user)) => {
            // 使用bcrypt验证密码
            match verify(&req.password, &user.2) {
                Ok(true) => {
                    // 生成真正的JWT token
                    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "change-me-in-production".to_string());
                    let now = chrono::Utc::now();
                    let exp = now + chrono::Duration::hours(24);
                    let claims = Claims {
                        sub: user.0.clone(),
                        username: user.1.clone(),
                        exp: exp.timestamp() as usize,
                        iat: now.timestamp() as usize,
                        permissions: vec!["*".to_string()],
                        dept_id: None,
                        roles: vec!["admin".to_string()],
                        token_version: None,
                    };
                    let token = generate_jwt(claims, &jwt_secret)
                        .unwrap_or_else(|_| format!("token_{}", uuid::Uuid::new_v4()));

                    // Log successful login with structured logging
                    tracing::info!(
                        username = %username,
                        user_id = %user.0,
                        action = "login",
                        success = true,
                        "Login successful"
                    );

                    Json(serde_json::json!({
                        "code": 200,
                        "message": "登录成功",
                        "data": {
                            "access_token": token,
                            "refresh_token": token,
                            "token_type": "Bearer",
                            "expires_in": 3600,
                            "user": {
                                "id": user.0,
                                "username": user.1,
                                "nickname": user.3,
                                "email": user.4,
                                "phone": user.5,
                                "avatar": user.6,
                                "status": user.7,
                                "roles": ["admin"],
                                "permissions": ["*"]
                            }
                        }
                    }))
                }
                Ok(false) => {
                    // Log failed login attempt
                    tracing::warn!(
                        username = %username,
                        action = "login",
                        success = false,
                        reason = "invalid_password",
                        "Login failed - invalid password"
                    );

                    Json(serde_json::json!({
                        "code": 401,
                        "message": "密码错误",
                        "data": serde_json::Value::Null
                    }))
                }
                Err(e) => {
                    // Log authentication error
                    tracing::error!(
                        username = %username,
                        action = "login",
                        success = false,
                        error = %e,
                        "Authentication error during password verification"
                    );

                    Json(serde_json::json!({
                        "code": 500,
                        "message": "密码验证失败",
                        "data": serde_json::Value::Null
                    }))
                }
            }
        }
        Ok(None) => {
            // Log user not found
            tracing::warn!(
                username = %username,
                action = "login",
                success = false,
                reason = "user_not_found",
                "Login failed - user not found"
            );

            Json(serde_json::json!({
                "code": 401,
                "message": "用户不存在",
                "data": serde_json::Value::Null
            }))
        }
        Err(e) => {
            // Log database error
            tracing::error!(
                username = %username,
                action = "login",
                success = false,
                error = %e,
                "Login failed - database error"
            );

            Json(serde_json::json!({
                "code": 500,
                "message": format!("数据库错误: {}", e),
                "data": serde_json::Value::Null
            }))
        }
    }
}

pub async fn register(Json(_req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "注册功能开发中",
        "data": serde_json::Value::Null
    }))
}

pub async fn logout() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "退出成功",
        "data": serde_json::Value::Null
    }))
}

pub async fn refresh(Json(_req): Json<serde_json::Value>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "刷新功能开发中",
        "data": serde_json::Value::Null
    }))
}

pub async fn get_current_user(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Json<serde_json::Value> {
    // Extract token from Authorization header
    let token = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "));

    if let Some(token) = token {
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "change-me-in-production".to_string());
        if let Ok(claims) = crate::common::auth::jwt::validate_jwt(token, &jwt_secret) {
            // Query user from database by ID
            let user_result = sqlx::query_as::<_, (String, String, Option<String>, Option<String>, Option<String>, Option<String>, i32)>(
                "SELECT id, username, nickname, email, phone, avatar, status FROM sys_user WHERE id = $1 AND deleted_at IS NULL"
            )
            .bind(&claims.sub)
            .fetch_optional(&state.pool)
            .await;

            if let Ok(Some(user)) = user_result {
                return Json(serde_json::json!({
                    "code": 200,
                    "message": "获取用户信息成功",
                    "data": {
                        "id": user.0,
                        "username": user.1,
                        "nickname": user.2,
                        "email": user.3,
                        "phone": user.4,
                        "avatar": user.5,
                        "status": user.6,
                        "roles": claims.roles,
                        "permissions": claims.permissions
                    }
                }));
            }
        }
    }

    Json(serde_json::json!({
        "code": 401,
        "message": "未授权",
        "data": serde_json::Value::Null
    }))
}
