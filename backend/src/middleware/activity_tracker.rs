// Activity tracker middleware for tracking user online status
// Updates last_access_time in sys_online_user table for authenticated requests

use axum::{
    extract::Request,
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::common::auth::jwt::validate_jwt;

/// Paths that don't require activity tracking (public endpoints)
const PUBLIC_PATHS: &[&str] = &[
    "/health",
    "/metrics",
    "/api/v1/auth/login",
    "/api/v1/auth/register",
];

/// Check if the path is a public endpoint that doesn't require authentication
fn is_public_path(path: &str) -> bool {
    PUBLIC_PATHS.iter().any(|p| path.starts_with(p))
}

/// Extract the Bearer token from the Authorization header
fn extract_token(auth_header: &str) -> Option<&str> {
    if auth_header.starts_with("Bearer ") {
        Some(auth_header.trim_start_matches("Bearer ").trim())
    } else {
        None
    }
}

/// Update last_access_time for a user's online session
/// Only updates if the last access was more than ACTIVITY_UPDATE_INTERVAL_SECS ago
async fn update_activity(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Only update if last access was more than 5 minutes ago
    let result = sqlx::query(
        r#"
        UPDATE sys_online_user
        SET last_access_time = NOW()
        WHERE user_id = $1
          AND (last_access_time IS NULL OR last_access_time < NOW() - INTERVAL '5 minutes')
        "#,
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() > 0 {
        tracing::debug!("Updated activity for user {}", user_id);
    }

    Ok(())
}

/// Activity tracking middleware
/// Skips public paths and only updates activity for authenticated users
/// Does not block the request - errors are logged but not propagated
pub async fn track_activity(
    request: Request,
    next: Next,
) -> Response {
    let path = request.uri().path().to_string();

    // Skip public paths
    if is_public_path(&path) {
        return next.run(request).await;
    }

    // Extract Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(extract_token);

    // If no auth token, skip activity tracking
    let Some(token) = auth_header else {
        return next.run(request).await;
    };

    // Get app state from request extensions
    let state = request
        .extensions()
        .get::<crate::app::AppState>()
        .cloned();

    let Some(state) = state else {
        // No app state available, skip activity tracking
        return next.run(request).await;
    };

    // Validate JWT and extract user_id
    let user_id = match validate_jwt(token, &state.settings.jwt.secret) {
        Ok(claims) => match Uuid::parse_str(&claims.sub) {
            Ok(uuid) => uuid,
            Err(_) => {
                tracing::warn!("Invalid user ID in JWT token");
                return next.run(request).await;
            }
        },
        Err(_) => {
            // Invalid token - skip activity tracking (don't block the request)
            // The auth extractor will handle rejecting invalid tokens for protected routes
            return next.run(request).await;
        }
    };

    // Clone pool for async task
    let pool = state.pool.clone();

    // Spawn a task to update activity (non-blocking)
    tokio::spawn(async move {
        if let Err(e) = update_activity(&pool, user_id).await {
            tracing::warn!("Failed to update activity for user {}: {}", user_id, e);
        }
    });

    // Continue with the request without waiting for activity update
    next.run(request).await
}
