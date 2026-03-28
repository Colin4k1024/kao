use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

/// Structured error response data
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponseData {
    pub error_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_after: Option<u64>,
}

impl ErrorResponseData {
    pub fn new(error_type: impl Into<String>) -> Self {
        ErrorResponseData {
            error_type: error_type.into(),
            field: None,
            details: None,
            retry_after: None,
        }
    }

    pub fn with_field(mut self, field: impl Into<String>) -> Self {
        self.field = Some(field.into());
        self
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }

    pub fn with_retry_after(mut self, seconds: u64) -> Self {
        self.retry_after = Some(seconds);
        self
    }
}

#[derive(Debug)]
pub enum AppError {
    // Database errors with source
    Database(String),
    // Authentication errors
    Authentication(String),
    // Authorization/permission errors
    Authorization(String),
    // Validation errors with field name
    Validation { field: String, message: String },
    // Resource not found
    NotFound(String),
    // Rate limiting with retry-after
    RateLimit { retry_after: u64 },
    // Internal/server errors
    Internal(Option<String>),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Authentication(msg) => write!(f, "Authentication error: {}", msg),
            AppError::Authorization(msg) => write!(f, "Authorization error: {}", msg),
            AppError::Validation { field, message } => write!(f, "Validation error on {}: {}", field, message),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::RateLimit { retry_after } => write!(f, "Rate limit exceeded, retry after {} seconds", retry_after),
            AppError::Internal(msg) => write!(f, "Internal error: {}", msg.as_ref().unwrap_or(&"Unknown error".to_string())),
        }
    }
}

impl std::error::Error for AppError {}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_response) = match &self {
            AppError::Authentication(msg) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponseData::new("AuthenticationError").with_details(msg),
            ),
            AppError::Authorization(msg) => (
                StatusCode::FORBIDDEN,
                ErrorResponseData::new("AuthorizationError").with_details(msg),
            ),
            AppError::Validation { field, message } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                ErrorResponseData::new("ValidationError")
                    .with_field(field)
                    .with_details(message),
            ),
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorResponseData::new("NotFoundError").with_details(msg),
            ),
            AppError::RateLimit { retry_after } => (
                StatusCode::TOO_MANY_REQUESTS,
                ErrorResponseData::new("RateLimitError").with_retry_after(*retry_after),
            ),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponseData::new("DatabaseError").with_details("A database error occurred"),
            ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponseData::new("InternalError").with_details(msg.as_ref().unwrap_or(&"An internal error occurred".to_string())),
            ),
        };

        let body = serde_json::to_string(&json!({
            "code": status.as_u16(),
            "message": self.to_string(),
            "data": error_response
        }))
        .unwrap_or_else(|_| r#"{"code":500,"message":"Failed to serialize error"}"#.to_string());

        let mut headers = axum::http::HeaderMap::new();
        headers.insert(
            axum::http::header::CONTENT_TYPE,
            axum::http::HeaderValue::from_static("application/json"),
        );

        // Add Retry-After header for rate limit errors
        if let AppError::RateLimit { retry_after } = &self {
            headers.insert(
                axum::http::header::RETRY_AFTER,
                axum::http::HeaderValue::from_str(&retry_after.to_string())
                    .unwrap_or_else(|_| axum::http::HeaderValue::from_static("900"))
            );
        }

        (status, headers, body).into_response()
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        AppError::Authentication(format!("JWT error: {}", err))
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::Internal(Some(err.to_string()))
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Internal(Some(err.to_string()))
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::Validation {
            field: "uuid".to_string(),
            message: format!("Invalid UUID format: {}", err),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Internal(Some(format!("JSON serialization error: {}", err)))
    }
}
