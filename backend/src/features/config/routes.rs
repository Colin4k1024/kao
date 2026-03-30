use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};

use crate::common::{auth::extractor::AuthUser, db::get_pool, error::AppError, response::ApiResponse};

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
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    let configs = service.list_configs(db, None, None).await?;
    Ok(ApiResponse::success(configs))
}

pub async fn get_config(
    _auth_user: AuthUser,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    match service.get_config_by_key(db, &key).await? {
        Some(c) => Ok(ApiResponse::success(c)),
        None => Ok(ApiResponse::error(404, "Config not found".to_string())),
    }
}

pub async fn create_config(
    _auth_user: AuthUser,
    Json(request): Json<CreateConfigRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    let c = service.create_config(db, request).await?;
    Ok(ApiResponse::success(c))
}

pub async fn update_config(
    _auth_user: AuthUser,
    Path(key): Path<String>,
    Json(request): Json<UpdateConfigRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    let c = service.update_config(db, &key, request).await?;
    Ok(ApiResponse::success(c))
}

pub async fn delete_config(
    _auth_user: AuthUser,
    Path(key): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = ConfigService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    service.delete_config(db, &key).await?;
    Ok(ApiResponse::success_no_data())
}
