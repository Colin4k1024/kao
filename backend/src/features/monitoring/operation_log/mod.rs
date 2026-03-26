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
pub struct OperationLog {
    pub id: Uuid,
    pub user_id: Uuid,
    pub username: String,
    pub module: String,
    pub action_type: String,
    pub method: String,
    pub path: String,
    pub request_method: String,
    pub request_params: Option<String>,
    pub response_code: i32,
    pub response_message: Option<String>,
    pub execution_time: i64,
    pub create_time: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationLogListResponse {
    pub list: Vec<OperationLog>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOperationLogRequest {
    pub user_id: Uuid,
    pub username: String,
    pub module: String,
    pub action_type: String,
    pub method: String,
    pub path: String,
    pub request_method: String,
    pub request_params: Option<String>,
    pub response_code: i32,
    pub response_message: Option<String>,
    pub execution_time: i64,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationLogQueryParams {
    #[serde(default)]
    pub user_id: Option<Uuid>,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub module: Option<String>,
    #[serde(default)]
    pub action_type: Option<String>,
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

pub struct OperationLogService {
    pub pool: PgPool,
}

impl OperationLogService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_operation_log(&self, log: CreateOperationLogRequest) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO sys_oper_log (
                id, user_id, username, module, action_type, method, path,
                request_method, request_params, response_code, response_message,
                execution_time, ip_address, user_agent, status, create_time
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
            "#,
            log.id,
            log.user_id,
            log.username,
            log.module,
            log.action_type,
            log.method,
            log.path,
            log.request_method,
            log.request_params,
            log.response_code,
            log.response_message,
            log.execution_time,
            log.ip_address,
            log.user_agent,
            log.status,
            chrono::Utc::now().to_rfc3339()
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_operation_logs(
        &self,
        params: OperationLogQueryParams,
    ) -> Result<OperationLogListResponse, sqlx::Error> {
        let page = params.page.max(1);
        let page_size = params.page_size.max(1).min(100);
        let offset = (page - 1) * page_size;

        // Build query with filters
        let mut query_string = String::from(
            r#"
            SELECT 
                id, user_id, username, module, action_type, method, path,
                request_method, request_params, response_code, response_message,
                execution_time, ip_address, user_agent, status, create_time
            FROM sys_oper_log
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

        if let Some(module) = params.module {
            params_vec.push(format!("%{}%", module).as_str());
            query_string.push_str(" AND module LIKE $");
            query_string.push_str(&(params_vec.len()).to_string());
        }

        if let Some(action_type) = params.action_type {
            params_vec.push(action_type.as_str());
            query_string.push_str(" AND action_type = $");
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
            SELECT COUNT(*) FROM sys_oper_log
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

        if params.module.is_some() {
            count_params.push(format!("%{}%", params.module.unwrap()).as_str());
            count_query.push_str(" AND module LIKE $");
            count_query.push_str(&(count_params.len()).to_string());
        }

        if params.action_type.is_some() {
            count_params.push(params.action_type.unwrap().as_str());
            count_query.push_str(" AND action_type = $");
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

        let logs: Vec<OperationLog> = sqlx::query_as(&query_string)
            .fetch_all(&self.pool)
            .await?;

        Ok(OperationLogListResponse {
            list: logs,
            total,
            page,
            page_size,
        })
    }

    pub async fn get_operation_log_by_id(&self, id: Uuid) -> Option<OperationLog> {
        let log: Option<OperationLog> = sqlx::query_as(
            r#"
            SELECT 
                id, user_id, username, module, action_type, method, path,
                request_method, request_params, response_code, response_message,
                execution_time, ip_address, user_agent, status, create_time
            FROM sys_oper_log
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .ok();

        log
    }

    pub async fn delete_operation_log(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let affected = sqlx::query!("DELETE FROM sys_oper_log WHERE id = $1", id)
            .execute(&self.pool)
            .await?
            .rows_affected();

        Ok(affected > 0)
    }
}

pub struct OperationLogController;

impl OperationLogController {
    pub async fn create_operation_log_handler(
        State(pool): State<PgPool>,
        Json(req): Json<CreateOperationLogRequest>,
    ) -> Response {
        let service = OperationLogService::new(pool);

        match service.create_operation_log(req).await {
            Ok(_) => {
                ApiResponse::<()>::success_no_data()
            }
            Err(e) => {
                log::error!("Failed to create operation log: {}", e);
                ApiResponse::error(500, format!("Failed to create operation log: {}", e))
            }
        }
    }

    pub async fn get_operation_logs_handler(
        State(pool): State<PgPool>,
        query: axum::extract::Query<OperationLogQueryParams>,
    ) -> Response {
        let service = OperationLogService::new(pool);

        match service.get_operation_logs(query.0).await {
            Ok(response) => {
                ApiResponse::success(response)
            }
            Err(e) => {
                log::error!("Failed to get operation logs: {}", e);
                ApiResponse::error(500, format!("Failed to get operation logs: {}", e))
            }
        }
    }

    pub async fn get_operation_log_handler(
        State(pool): State<PgPool>,
        axum::extract::Path(id): axum::extract::Path<Uuid>,
    ) -> Response {
        let service = OperationLogService::new(pool);

        match service.get_operation_log_by_id(id).await {
            Some(log) => {
                ApiResponse::success(log)
            }
            None => {
                ApiResponse::error(404, "Operation log not found".to_string())
            }
        }
    }

    pub async fn delete_operation_log_handler(
        State(pool): State<PgPool>,
        axum::extract::Path(id): axum::extract::Path<Uuid>,
    ) -> Response {
        let service = OperationLogService::new(pool);

        match service.delete_operation_log(id).await {
            Ok(true) => {
                ApiResponse::<()>::success_no_data()
            }
            Ok(false) => {
                ApiResponse::error(404, "Operation log not found".to_string())
            }
            Err(e) => {
                log::error!("Failed to delete operation log: {}", e);
                ApiResponse::error(500, format!("Failed to delete operation log: {}", e))
            }
        }
    }
}
