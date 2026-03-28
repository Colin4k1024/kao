//! Input validation middleware using the validator crate.
//!
//! This module provides request validation for auth endpoints using
//! the validator derive macros for declarative validation rules.

use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Response type for validation errors
#[derive(Debug, Serialize)]
pub struct ValidationErrorResponse {
    pub code: u32,
    pub message: String,
    pub errors: Vec<FieldError>,
}

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

/// Validates a request body against a validator struct.
///
/// Returns a 400 Bad Request response with validation errors if validation fails.
/// This middleware should be applied to routes that expect validated request bodies.
///
/// Type bounds:
/// - Validate: from validator crate for validation derive macros
/// - for<'de> Deserialize<'de>: from serde for JSON deserialization
pub async fn validate_request<T>(request: Request, next: Next) -> Result<Response, StatusCode>
where
    T: Validate + for<'de> Deserialize<'de> + Send + 'static,
{
    // Clone the request to read the body
    let (parts, body) = request.into_parts();

    // Parse the body as JSON
    let bytes = axum::body::to_bytes(body, 10 * 1024 * 1024) // 10MB limit
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let value: serde_json::Value = serde_json::from_slice(&bytes)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Deserialize into the target type
    let data: T = serde_json::from_value(value)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Validate
    if let Err(validation_errors) = data.validate() {
        let errors: Vec<FieldError> = validation_errors
            .field_errors()
            .iter()
            .flat_map(|(field, errors)| {
                errors.iter().map(move |e| FieldError {
                    field: field.to_string(),
                    message: e.message.clone().map(|m| m.to_string()).unwrap_or_else(|| {
                        format!("{} is invalid", field)
                    }),
                })
            })
            .collect();

        let response = ValidationErrorResponse {
            code: 400,
            message: "Validation failed".to_string(),
            errors,
        };

        let json = serde_json::to_string(&response).unwrap_or_else(|_| r#"{"code":400,"message":"Validation failed"}"#.to_string());

        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("Content-Type", "application/json")
            .body(Body::from(json))
            .unwrap());
    }

    // Reconstruct the request with the parsed body
    let request = Request::from_parts(parts, Body::from(bytes));

    Ok(next.run(request).await)
}
