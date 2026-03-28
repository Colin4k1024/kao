use axum::{extract::State, Json};
use bcrypt::{verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use crate::app::AppState;

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
                    // 生成JWT token
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
                    Json(serde_json::json!({
                        "code": 401,
                        "message": "密码错误",
                        "data": serde_json::Value::Null
                    }))
                }
                Err(_) => {
                    Json(serde_json::json!({
                        "code": 500,
                        "message": "密码验证失败",
                        "data": serde_json::Value::Null
                    }))
                }
            }
        }
        Ok(None) => {
            Json(serde_json::json!({
                "code": 401,
                "message": "用户不存在",
                "data": serde_json::Value::Null
            }))
        }
        Err(e) => {
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

pub async fn get_current_user() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "code": 200,
        "message": "获取用户信息成功",
        "data": {
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
    }))
}
