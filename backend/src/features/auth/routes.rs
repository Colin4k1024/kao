use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde_json::json;
use std::sync::Arc;

use crate::{
    AppState,
    common::{auth::extractor::AuthUser, error::AppError, response::ApiResponse},
};

use super::{
    model::{CurrentSessionResponse, LoginRequest},
    service::AuthService,
};

pub fn auth_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/api/v1/auth/login", axum::routing::post(login))
        .route("/api/v1/auth/register", axum::routing::post(register))
        .route("/api/v1/auth/profile", axum::routing::get(get_profile))
        .route("/api/v1/auth/session", axum::routing::get(get_session))
        .route("/api/v1/auth/permissions", axum::routing::get(get_permissions))
        .route("/api/v1/auth/menus", axum::routing::get(get_menus))
        .route("/api/v1/auth/change-password", axum::routing::post(change_password))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let auth_service = AuthService::new(Arc::new(state.settings.clone()));
    let response = auth_service.login(&state.pool, request).await?;
    Ok(ApiResponse::success(response))
}

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

pub async fn get_permissions(
    State(_state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    Ok(ApiResponse::success(serde_json::json!({
        "permissions": auth_user.permissions
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
