use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{
    app::state::AppState,
    common::{auth::extractor::AuthUser, response::ApiResponse},
};

use super::{
    model::{CurrentSessionResponse, LoginRequest, LoginResponse},
    service::AuthService,
};

pub fn auth_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/api/v1/auth/login", axum::routing::post(login))
        .route("/api/v1/auth/profile", axum::routing::get(get_profile))
        .route("/api/v1/auth/session", axum::routing::get(get_session))
        .route("/api/v1/auth/permissions", axum::routing::get(get_permissions))
        .route("/api/v1/auth/menus", axum::routing::get(get_menus))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let auth_service = AuthService::new(state.config.clone());
    let response = auth_service.login(&state.db, request).await?;
    Ok(ApiResponse::success(response))
}

pub async fn get_profile(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let auth_service = AuthService::new(state.config.clone());
    let profile = auth_service
        .get_current_user_profile(&state.db, auth_user.id)
        .await?;
    Ok(ApiResponse::success(profile))
}

pub async fn get_session(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    // Get user profile
    let auth_service = AuthService::new(state.config.clone());
    let profile = auth_service
        .get_current_user_profile(&state.db, auth_user.id)
        .await?;

    // Get user menu tree
    let menu_tree = super::repo::get_user_menu_tree(&state.db, auth_user.id).await?;

    let session_response = CurrentSessionResponse {
        user: profile,
        permissions: auth_user.permissions,
        roles: auth_user.roles,
        menu_tree,
    };

    Ok(ApiResponse::success(session_response))
}

pub async fn get_permissions(
    State(state): State<AppState>,
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
    let menu_tree = super::repo::get_user_menu_tree(&state.db, auth_user.id).await?;
    Ok(ApiResponse::success(serde_json::json!({
        "menus": menu_tree
    })))
}