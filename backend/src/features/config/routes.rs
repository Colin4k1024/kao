use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use crate::{AppState, common::{auth::extractor::AuthUser, error::AppError, response::ApiResponse}};

use super::{
    model::{CreateConfigRequest, UpdateConfigRequest},
    service::ConfigService,
};

pub fn config_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/config", axum::routing::get(list_configs))
        .route("/config", axum::routing::post(create_config))
        .route("/config/{key}", axum::routing::get(get_config))
        .route("/config/{key}", axum::routing::put(update_config))
        .route("/config/{key}", axum::routing::delete(delete_config))
}

pub async fn list_configs(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    let configs = service.list_configs(&state.pool, None, None).await?;
    Ok(ApiResponse::success(serde_json::json!({
        "list": configs,
        "total": configs.len()
    })))
}

pub async fn get_config(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    match service.get_config_by_key(&state.pool, &key).await? {
        Some(c) => Ok(ApiResponse::success(c)),
        None => Ok(ApiResponse::error(404, "Config not found".to_string())),
    }
}

pub async fn create_config(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(request): Json<CreateConfigRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    let c = service.create_config(&state.pool, request).await?;
    Ok(ApiResponse::success(c))
}

pub async fn update_config(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(key): Path<String>,
    Json(request): Json<UpdateConfigRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    let c = service.update_config(&state.pool, &key, request).await?;
    Ok(ApiResponse::success(c))
}

pub async fn delete_config(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    service.delete_config(&state.pool, &key).await?;
    Ok(ApiResponse::success_no_data())
}
