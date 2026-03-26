use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: u32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "Success".to_string(),
            data: Some(data),
        }
    }

    pub fn success_with_msg(message: &str, data: T) -> Self {
        Self {
            code: 200,
            message: message.to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: u32, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let code = self.code;
        let status = match code {
            200..=299 => StatusCode::OK,
            400 => StatusCode::BAD_REQUEST,
            401 => StatusCode::UNAUTHORIZED,
            403 => StatusCode::FORBIDDEN,
            404 => StatusCode::NOT_FOUND,
            500..=599 => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::OK,
        };

        (status, Json(self)).into_response()
    }
}

pub type AppError = String;

impl From<AppError> for ApiResponse<()> {
    fn from(err: AppError) -> Self {
        ApiResponse {
            code: 500,
            message: err,
            data: None,
        }
    }
}
