use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Database(#[from] sqlx::Error),

    #[error("认证失败: {0}")]
    Unauthorized(String),

    #[error("权限不足: {0}")]
    Forbidden(String),

    #[error("资源不存在: {0}")]
    NotFound(String),

    #[error("请求参数错误: {0}")]
    BadRequest(String),

    #[error("服务器内部错误: {0}")]
    Internal(String),

    #[error("密码错误")]
    InvalidPassword,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Unauthorized(ref e) => (StatusCode::UNAUTHORIZED, e.to_string()),
            AppError::Forbidden(ref e) => (StatusCode::FORBIDDEN, e.to_string()),
            AppError::NotFound(ref e) => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::BadRequest(ref e) => (StatusCode::BAD_REQUEST, e.to_string()),
            AppError::Internal(ref e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::InvalidPassword => (StatusCode::UNAUTHORIZED, "Invalid password".to_string()),
        };

        let body = Json(json!({
            "error": error_message,
            "status": status.as_u16(),
        }));

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
