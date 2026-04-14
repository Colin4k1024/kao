use axum::{
    extract::{Request, State},
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};

use crate::{app::AppState, common::error::AppError};

/// List of paths that don't require authentication
const PUBLIC_PATHS: &[&str] = &[
    "/health",
    "/metrics",
    "/api/v1/login",
    "/api/v1/register",
    "/swagger-ui",
    "/docs",
    "/redoc",
    "/rapidoc",
];

/// Check if a path is public and doesn't require authentication
fn is_public_path(path: &str) -> bool {
    PUBLIC_PATHS.iter().any(|p| path.starts_with(p))
}

/// Authentication middleware
/// Validates JWT tokens for protected routes and allows public paths through
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let path = req.uri().path().to_string();

    // Skip authentication for public paths
    if is_public_path(&path) {
        return Ok(next.run(req).await);
    }

    // Extract authorization header
    let auth_header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Authentication("No authorization header".to_string()))?;

    // Extract bearer token
    let token = if auth_header.starts_with("Bearer ") {
        auth_header.trim_start_matches("Bearer ").trim()
    } else {
        return Err(AppError::Authentication(
            "Invalid authorization header format".to_string(),
        ));
    };

    // Validate JWT token using settings from state
    let claims = crate::common::auth::jwt::validate_jwt(token, &state.settings.jwt.secret)?;

    // Insert claims into request extensions for use in handlers
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
