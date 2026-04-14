use crate::common::response::ApiResponse;
pub mod routes;
use axum::{
    extract::State,
    response::Response,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LoginLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub status: i32,
    pub message: Option<String>,
    pub login_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginLogListResponse {
    pub list: Vec<LoginLog>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoginLogRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub status: i32,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginLogQueryParams {
    #[serde(default)]
    pub user_id: Option<Uuid>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub status: Option<i32>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub page: i64,
    #[serde(default)]
    pub page_size: i64,
}

pub struct LoginLogService {
    pub pool: PgPool,
}

impl LoginLogService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_login_log(&self, log: CreateLoginLogRequest) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO sys_login_log (
                id, user_id, username, ip_address, user_agent, status, message, create_time
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
        )
        .bind(log.id)
        .bind(log.user_id)
        .bind(&log.username)
        .bind(&log.ip_address)
        .bind(&log.user_agent)
        .bind(log.status)
        .bind(&log.message)
        .bind(chrono::Utc::now().to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_login_logs(
        &self,
        params: LoginLogQueryParams,
    ) -> Result<LoginLogListResponse, sqlx::Error> {
        let page = params.page.max(1);
        let page_size = params.page_size.clamp(1, 100);
        let offset = (page - 1) * page_size;

        // Simplified query without dynamic WHERE clause (needs refactoring for full filter support)
        let logs: Vec<LoginLog> = sqlx::query_as(
            r#"SELECT id, user_id, username, ip_address, browser as user_agent, status, msg as message,
               login_time::text as login_time
               FROM sys_login_log ORDER BY login_time DESC LIMIT $1 OFFSET $2"#
        )
        .bind(page_size)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        // Count query (simplified)
        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM sys_login_log")
            .fetch_one(&self.pool)
            .await?;

        Ok(LoginLogListResponse {
            list: logs,
            total,
            page,
            page_size,
        })
    }

    pub async fn get_login_log_by_id(&self, id: Uuid) -> Result<Option<LoginLog>, sqlx::Error> {
        let log = sqlx::query_as(
            r#"
            SELECT id, user_id, username, ip_address, browser as user_agent, status, msg as message,
               login_time::text as login_time
            FROM sys_login_log
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(log)
    }
}

pub struct LoginLogController;

impl LoginLogController {
    pub async fn create_login_log_handler(
        State(state): State<AppState>,
        Json(req): Json<CreateLoginLogRequest>,
    ) -> Response {
        let service = LoginLogService::new(state.pool.clone());

        match service.create_login_log(req).await {
            Ok(_) => {
                ApiResponse::<()>::success_no_data()
            }
            Err(e) => {
                log::error!("Failed to create login log: {}", e);
                ApiResponse::error(500, format!("Failed to create login log: {}", e))
            }
        }
    }

    pub async fn get_login_logs_handler(
        State(state): State<AppState>,
        query: axum::extract::Query<LoginLogQueryParams>,
    ) -> Response {
        let service = LoginLogService::new(state.pool.clone());

        match service.get_login_logs(query.0).await {
            Ok(response) => {
                ApiResponse::success(response)
            }
            Err(e) => {
                log::error!("Failed to get login logs: {}", e);
                ApiResponse::error(500, format!("Failed to get login logs: {}", e))
            }
        }
    }

    pub async fn get_login_log_handler(
        State(state): State<AppState>,
        axum::extract::Path(id): axum::extract::Path<Uuid>,
    ) -> Response {
        let service = LoginLogService::new(state.pool.clone());

        match service.get_login_log_by_id(id).await {
            Ok(Some(log)) => {
                ApiResponse::success(log)
            }
            Ok(None) => {
                ApiResponse::error(404, "Login log not found".to_string())
            }
            Err(e) => {
                log::error!("Failed to get login log: {}", e);
                ApiResponse::error(500, format!("Failed to get login log: {}", e))
            }
        }
    }
}
