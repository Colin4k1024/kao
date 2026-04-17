use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

/// List of paths that are considered Swagger/OpenAPI documentation endpoints
const SWAGGER_PATHS: &[&str] = &[
    "/swagger-ui",
    "/api-docs",
    "/rapidoc",
    "/redoc",
];

/// Check if a path is a Swagger documentation path
fn is_swagger_path(path: &str) -> bool {
    SWAGGER_PATHS.iter().any(|p| path.starts_with(p))
}

/// Swagger protection middleware
/// In production, Swagger endpoints should be protected or disabled
pub async fn swagger_protection_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let path = request.uri().path().to_string();

    // Check if this is a swagger path
    if is_swagger_path(&path) {
        // Check environment - in production, block access to Swagger
        // This is a simple check; in production you might want to use a proper config
        let environment = std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());

        if environment == "production" {
            // Block access to Swagger in production
            let response = serde_json::json!({
                "code": 403,
                "message": "API documentation is not available in production"
            });

            return Ok(Response::builder()
                .status(StatusCode::FORBIDDEN)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(response.to_string()))
                .unwrap());
        }
    }

    Ok(next.run(request).await)
}