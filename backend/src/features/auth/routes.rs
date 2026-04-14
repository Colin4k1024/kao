use axum::{
    extract::{Json, State},
    response::IntoResponse,
    http::HeaderMap,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    AppState,
    common::{auth::extractor::AuthUser, error::AppError, response::ApiResponse},
};

use super::{
    model::{CurrentSessionResponse, LoginRequest},
    service::{AuthService, OnlineUserInfo},
};

pub fn auth_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/login", axum::routing::post(login))
        .route("/register", axum::routing::post(register))
        .route("/logout", axum::routing::post(logout))
        .route("/profile", axum::routing::get(get_profile))
        .route("/session", axum::routing::get(get_session))
        .route("/permissions", axum::routing::get(get_permissions))
        .route("/change-password", axum::routing::post(change_password))
        // Online user management routes (admin only in production)
        .route("/online-users/cleanup", axum::routing::post(cleanup_expired_sessions))
        .route("/online-users/count", axum::routing::get(get_online_user_count))
}

/// Parse user agent string to extract browser and OS info
fn parse_user_agent(user_agent: Option<&str>) -> (String, String) {
    let ua = user_agent.unwrap_or("unknown");

    let browser = if ua.contains("Firefox") {
        "Firefox"
    } else if ua.contains("Chrome") && !ua.contains("Edg") {
        "Chrome"
    } else if ua.contains("Edg") {
        "Edge"
    } else if ua.contains("Safari") && !ua.contains("Chrome") {
        "Safari"
    } else if ua.contains("Opera") || ua.contains("OPR") {
        "Opera"
    } else {
        "Other"
    }.to_string();

    let os = if ua.contains("Windows") {
        "Windows"
    } else if ua.contains("Mac OS") || ua.contains("MacOS") {
        "macOS"
    } else if ua.contains("Linux") {
        "Linux"
    } else if ua.contains("Android") {
        "Android"
    } else if ua.contains("iPhone") || ua.contains("iPad") {
        "iOS"
    } else {
        "Other"
    }.to_string();

    (browser, os)
}

/// Extract client IP address from request headers
fn extract_client_ip(headers: &HeaderMap) -> String {
    // Check X-Forwarded-For header (may contain multiple IPs)
    if let Some(forwarded) = headers.get("x-forwarded-for") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(ip) = forwarded_str.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }

    // Check X-Real-IP header
    if let Some(real_ip) = headers.get("x-real-ip") {
        if let Ok(ip_str) = real_ip.to_str() {
            return ip_str.trim().to_string();
        }
    }

    "unknown".to_string()
}

/// POST /api/v1/login - User login
#[utoipa::path(
    post,
    path = "/api/v1/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = ApiResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let auth_service = AuthService::new(Arc::new(state.settings.clone()));
    let response = auth_service.login(&state.pool, request).await?;

    // Extract client info for online user tracking
    let ip_address = extract_client_ip(&headers);
    let user_agent = headers.get("user-agent").and_then(|h| h.to_str().ok());
    let (browser, os) = parse_user_agent(user_agent);

    // Generate session ID and create online user record
    let session_id = Uuid::new_v4().to_string();
    let online_user_info = OnlineUserInfo {
        session_id: session_id.clone(),
        user_id: response.userInfo.id,
        username: response.userInfo.username.clone(),
        ip_address,
        browser,
        os,
    };

    // Create online user record (non-blocking - log error but don't fail login)
    if let Err(e) = auth_service.create_online_user(&state.pool, online_user_info).await {
        tracing::warn!("Failed to create online user record: {}", e);
    }

    Ok(ApiResponse::success(serde_json::json!({
        "access_token": response.token,
        "refresh_token": "",
        "token_type": "Bearer",
        "expires_in": 86400,
        "user": response.userInfo
    })))
}

/// POST /api/v1/register - User registration
#[utoipa::path(
    post,
    path = "/api/v1/register",
    tag = "auth",
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Registration successful", body = ApiResponse),
        (status = 400, description = "Validation error")
    )
)]
pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let username = request
        .get("username")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation { field: "username".to_string(), message: "Username is required".to_string() })?
        .to_string();

    let password = request
        .get("password")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation { field: "password".to_string(), message: "Password is required".to_string() })?
        .to_string();

    let email = request
        .get("email")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let display_name = request
        .get("displayName")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| username.clone());

    let auth_service = AuthService::new(Arc::new(state.settings.clone()));
    auth_service.register(&state.pool, username, password, email, display_name).await?;

    Ok(ApiResponse::success(json!({
        "message": "Registration successful"
    })))
}

/// GET /api/v1/profile - Get current user profile
#[utoipa::path(
    get,
    path = "/api/v1/profile",
    tag = "auth",
    security (
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Profile retrieved successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_profile(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let auth_service = AuthService::new(Arc::new(state.settings.clone()));
    let profile = auth_service
        .get_current_user_profile(&state.pool, auth_user.id)
        .await?;
    Ok(ApiResponse::success(profile))
}

/// GET /api/v1/session - Get current user session
#[utoipa::path(
    get,
    path = "/api/v1/session",
    tag = "auth",
    security (
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Session retrieved successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_session(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    // Get user profile
    let auth_service = AuthService::new(Arc::new(state.settings.clone()));
    let profile = auth_service
        .get_current_user_profile(&state.pool, auth_user.id)
        .await?;

    // Get user menu tree
    let menu_tree = super::repo::get_user_menu_tree(&state.pool, auth_user.id).await?;

    let session_response = CurrentSessionResponse {
        user: profile,
        permissions: auth_user.permissions,
        roles: auth_user.roles,
        menu_tree,
    };

    Ok(ApiResponse::success(session_response))
}

/// GET /api/v1/permissions - Get current user permissions
#[utoipa::path(
    get,
    path = "/api/v1/permissions",
    tag = "auth",
    security (
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Permissions retrieved successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_permissions(
    State(_state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    Ok(ApiResponse::success(serde_json::json!({
        "permissions": auth_user.permissions
    })))
}

/// POST /api/v1/logout - User logout
#[utoipa::path(
    post,
    path = "/api/v1/logout",
    tag = "auth",
    security (
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Logout successful", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let auth_service = AuthService::new(Arc::new(state.settings.clone()));

    // Delete online user record for this user
    if let Err(e) = auth_service.delete_online_user_by_user(&state.pool, auth_user.id).await {
        tracing::warn!("Failed to delete online user record for user {}: {}", auth_user.id, e);
    }

    tracing::info!("User {} logged out", auth_user.id);
    Ok(ApiResponse::success(json!({
        "message": "Logout successful"
    })))
}

pub async fn get_menus(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_tree = super::repo::get_user_menu_tree(&state.pool, auth_user.id).await?;
    Ok(ApiResponse::success(serde_json::json!({
        "menus": menu_tree
    })))
}

/// POST /api/v1/change-password - Change user password
#[utoipa::path(
    post,
    path = "/api/v1/change-password",
    tag = "auth",
    security (
        ("bearer_auth" = [])
    ),
    request_body = serde_json::Value,
    responses(
        (status = 200, description = "Password changed successfully", body = ApiResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn change_password(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<serde_json::Value>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let old_password = request
        .get("oldPassword")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation { field: "oldPassword".to_string(), message: "Old password is required".to_string() })?
        .to_string();

    let new_password = request
        .get("newPassword")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AppError::Validation { field: "newPassword".to_string(), message: "New password is required".to_string() })?
        .to_string();

    let auth_service = AuthService::new(Arc::new(state.settings.clone()));
    auth_service
        .change_password(&state.pool, auth_user.id, old_password, new_password)
        .await?;

    Ok(ApiResponse::success(json!({
        "message": "Password changed successfully"
    })))
}

/// POST /api/v1/online-users/cleanup - Cleanup expired online user sessions
#[utoipa::path(
    post,
    path = "/api/v1/online-users/cleanup",
    tag = "auth",
    security (
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Expired sessions cleaned up", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn cleanup_expired_sessions(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let auth_service = AuthService::new(Arc::new(state.settings.clone()));
    let deleted_count = auth_service.cleanup_expired_sessions(&state.pool).await?;

    tracing::info!("Cleaned up {} expired session(s)", deleted_count);

    Ok(ApiResponse::success(json!({
        "message": "Expired sessions cleaned up",
        "deleted_count": deleted_count
    })))
}

/// GET /api/v1/online-users/count - Get count of active online users
#[utoipa::path(
    get,
    path = "/api/v1/online-users/count",
    tag = "auth",
    security (
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Online user count retrieved", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_online_user_count(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let auth_service = AuthService::new(Arc::new(state.settings.clone()));
    let count = auth_service.get_online_user_count(&state.pool).await?;

    Ok(ApiResponse::success(json!({
        "count": count
    })))
}
