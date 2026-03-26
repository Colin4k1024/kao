use crate::common::response::ApiResponse;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub status: i32,
    pub message: Option<String>,
    pub login_time: String,
    pub create_time: String,
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
        sqlx::query!(
            r#"
            INSERT INTO sys_login_log (
                id, user_id, username, ip_address, user_agent, status, message, create_time
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            log.id,
            log.user_id,
            log.username,
            log.ip_address,
            log.user_agent,
            log.status,
            log.message,
            chrono::Utc::now().to_rfc3339()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_login_logs(
        &self,
        params: LoginLogQueryParams,
    ) -> Result<LoginLogListResponse, sqlx::Error> {
        let page = params.page.max(1);
        let page_size = params.page_size.max(1).min(100);
        let offset = (page - 1) * page_size;

        // Build query with filters
        let mut query_string = String::from(
            r#"
            SELECT id, user_id, username, ip_address, user_agent, status, message, create_time
            FROM sys_login_log
            WHERE 1=1
            "#,
        );
        let mut params_vec: Vec<&str> = Vec::new();

        if let Some(user_id) = params.user_id {
            params_vec.push(user_id.to_string().as_str());
            query_string.push_str(" AND user_id = $");
            query_string.push_str(&(params_vec.len()).to_string());
        }

        if let Some(username) = params.username {
            params_vec.push(format!("%{}%", username).as_str());
            query_string.push_str(" AND username LIKE $");
            query_string.push_str(&(params_vec.len()).to_string());
        }

        if let Some(status) = params.status {
            params_vec.push(&status.to_string());
            query_string.push_str(" AND status = $");
            query_string.push_str(&(params_vec.len()).to_string());
        }

        if let Some(start_time) = params.start_time {
            params_vec.push(start_time.as_str());
            query_string.push_str(" AND create_time >= $");
            query_string.push_str(&(params_vec.len()).to_string());
        }

        if let Some(end_time) = params.end_time {
            params_vec.push(end_time.as_str());
            query_string.push_str(" AND create_time <= $");
            query_string.push_str(&(params_vec.len()).to_string());
        }

        query_string.push_str(" ORDER BY create_time DESC LIMIT $");
        query_string.push_str(&page_size.to_string());
        query_string.push_str(" OFFSET $");
        query_string.push_str(&offset.to_string());

        // Get total count
        let mut count_query = String::from(
            r#"
            SELECT COUNT(*) FROM sys_login_log
            WHERE 1=1
            "#,
        );
        let mut count_params: Vec<&str> = Vec::new();

        if params.user_id.is_some() {
            count_params.push(params.user_id.unwrap().to_string().as_str());
            count_query.push_str(" AND user_id = $");
            count_query.push_str(&(count_params.len()).to_string());
        }

        if params.username.is_some() {
            count_params.push(format!("%{}%", params.username.unwrap()).as_str());
            count_query.push_str(" AND username LIKE $");
            count_query.push_str(&(count_params.len()).to_string());
        }

        if params.status.is_some() {
            count_params.push(&params.status.unwrap().to_string());
            count_query.push_str(" AND status = $");
            count_query.push_str(&(count_params.len()).to_string());
        }

        if params.start_time.is_some() {
            count_params.push(params.start_time.unwrap().as_str());
            count_query.push_str(" AND create_time >= $");
            count_query.push_str(&(count_params.len()).to_string());
        }

        if params.end_time.is_some() {
            count_params.push(params.end_time.unwrap().as_str());
            count_query.push_str(" AND create_time <= $");
            count_query.push_str(&(count_params.len()).to_string());
        }

        let total: i64 = sqlx::query_scalar(&count_query)
            .fetch_one(&self.pool)
            .await?;

        let logs: Vec<LoginLog> = sqlx::query_as(&query_string)
            .fetch_all(&self.pool)
            .await?;

        Ok(LoginLogListResponse {
            list: logs,
            total,
            page,
            page_size,
        })
    }

    pub async fn get_login_log_by_id(&self, id: Uuid) -> Option<LoginLog> {
        let log: Option<LoginLog> = sqlx::query_as(
            r#"
            SELECT id, user_id, username, ip_address, user_agent, status, message, create_time
            FROM sys_login_log
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok();

        log
    }
}

pub struct LoginLogController;

impl LoginLogController {
    pub async fn create_login_log_handler(
        State(pool): State<PgPool>,
        Json(req): Json<CreateLoginLogRequest>,
    ) -> Response {
        let service = LoginLogService::new(pool);

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
        State(pool): State<PgPool>,
        query: axum::extract::Query<LoginLogQueryParams>,
    ) -> Response {
        let service = LoginLogService::new(pool);

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
        State(pool): State<PgPool>,
        axum::extract::Path(id): axum::extract::Path<Uuid>,
    ) -> Response {
        let service = LoginLogService::new(pool);

        match service.get_login_log_by_id(id).await {
            Some(log) => {
                ApiResponse::success(log)
            }
            None => {
                ApiResponse::error(404, "Login log not found".to_string())
            }
        }
    }
}
