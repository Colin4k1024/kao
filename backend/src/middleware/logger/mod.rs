// Logger middleware for request logging with request tracking

use axum::{
    body::Body,
    extract::Request,
    http::{HeaderName, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use std::task::{Context, Poll};
use tower_http::classify::ServerErrorsFailureClass;
use uuid::Uuid;

/// Header name for request tracking
pub const X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

/// Generate a new request ID
pub fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

/// Extract request ID from headers or generate a new one
pub fn get_or_create_request_id(request: &Request) -> String {
    let headers = request.headers();
    if let Some(existing_id) = headers.get(X_REQUEST_ID.as_str()) {
        if let Ok(id_str) = existing_id.to_str() {
            return id_str.to_string();
        }
    }
    generate_request_id()
}

/// Middleware for logging HTTP requests with request tracking
pub async fn request_logger(request: Request, next: Next) -> Response {
    let request_id = get_or_create_request_id(&request);
    let method = request.method().clone();
    let uri = request.uri().clone();

    // Log incoming request
    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %uri.path(),
        "Incoming request"
    );

    // Add request ID to response headers
    let mut response = next.run(request).await;

    // Add X-Request-ID header to response
    response.headers_mut().insert(
        X_REQUEST_ID.as_str(),
        HeaderValue::from_str(&request_id).unwrap_or_else(|_| HeaderValue::from_static("unknown")),
    );

    // Log response
    let status = response.status();
    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %uri.path(),
        status = %status.as_u16(),
        "Request completed"
    );

    response
}

/// Response extension for adding request ID to responses
pub fn add_request_id_to_response(response: Response, request_id: &str) -> Response {
    let mut resp = response;
    resp.headers_mut().insert(
        X_REQUEST_ID.as_str(),
        HeaderValue::from_str(request_id).unwrap_or_else(|_| HeaderValue::from_static("unknown")),
    );
    resp
}
