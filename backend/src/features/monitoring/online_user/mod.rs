use crate::common::response::ApiResponse;
pub mod routes;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OnlineUser {
    pub session_id: String,
    pub user_id: Uuid,
    pub username: String,
    pub dept_name: Option<String>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub login_time: String,
    pub last_activity_time: String,
    pub expire_time: String,
    pub status: i32, // 0: active, 1: force logged out
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnlineUsersResponse {
    pub list: Vec<OnlineUser>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceLogoutRequest {
    pub session_id: String,
    pub user_id: Uuid,
    pub reason: Option<String>,
}

pub struct OnlineUserService {
    pub pool: PgPool,
    // In production, this would also track sessions in Redis
}

impl OnlineUserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_online_users(&self) -> Result<OnlineUsersResponse, sqlx::Error> {
        let now = chrono::Utc::now();

        let users: Vec<OnlineUser> = sqlx::query_as(
            r#"
            SELECT 
                session_id,
                user_id,
                username,
                dept_name,
                ip_address,
                user_agent,
                login_time,
                last_activity_time,
                expire_time,
                status
            FROM sys_online_user
            WHERE expire_time > $1
            ORDER BY last_activity_time DESC
            "#,
        )
        .bind(now.to_rfc3339())
        .fetch_all(&self.pool)
        .await?;

        let total = users.len() as i64;

        Ok(OnlineUsersResponse { list: users, total })
    }

    pub async fn force_logout(&self, request: ForceLogoutRequest) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE sys_online_user
            SET status = $1, remark = $2, update_time = $3
            WHERE session_id = $4 AND user_id = $5
            "#,
        )
        .bind(1i32)
        .bind(request.reason.unwrap_or_else(|| "Force logout by admin".to_string()))
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(&request.session_id)
        .bind(&request.user_id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }

    pub async fn update_last_activity(&self, session_id: &str) -> Result<bool, sqlx::Error> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE sys_online_user
            SET last_activity_time = $1, update_time = $2
            WHERE session_id = $3
            "#,
        )
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(session_id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        Ok(rows_affected > 0)
    }
}

pub struct OnlineUserController;

impl OnlineUserController {
    pub async fn get_online_users_handler(State(state): State<AppState>) -> Response {
        let service = OnlineUserService::new(state.pool.clone());

        match service.get_online_users().await {
            Ok(response) => {
                ApiResponse::success(response)
            }
            Err(e) => {
                log::error!("Failed to get online users: {}", e);
                ApiResponse::error(500, format!("Failed to get online users: {}", e))
            }
        }
    }

    pub async fn force_logout_handler(
        State(state): State<AppState>,
        Json(req): Json<ForceLogoutRequest>,
    ) -> Response {
        let service = OnlineUserService::new(state.pool.clone());

        match service.force_logout(req).await {
            Ok(true) => {
                ApiResponse::<()>::success_no_data()
            }
            Ok(false) => {
                ApiResponse::error(404, "Session not found".to_string())
            }
            Err(e) => {
                log::error!("Failed to force logout: {}", e);
                ApiResponse::error(500, format!("Failed to force logout: {}", e))
            }
        }
    }
}
