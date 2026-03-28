use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::common::{db::get_pool, auth::extractor::AuthUser, error::AppError, response::ApiResponse};

use super::{
    model::{CreateDataRequest, UpdateDataRequest},
    service::DataService,
};

pub fn data_routes() -> axum::Router<()> {
    axum::Router::new()
        .route("/api/system/dictionary/data", axum::routing::get(list_data))
        .route("/api/system/dictionary/data", axum::routing::post(create_data))
        .route("/api/system/dictionary/data/{id}", axum::routing::get(get_data))
        .route("/api/system/dictionary/data/{id}", axum::routing::put(update_data))
        .route("/api/system/dictionary/data/{id}", axum::routing::delete(delete_data))
        .route("/api/system/dictionary/data/type/{dict_type}", axum::routing::get(list_data_by_type))
}

pub async fn list_data(
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
    let data = service.list_data_by_type(db, "").await?;
    Ok(ApiResponse::success(data))
}

pub async fn list_data_by_type(
    _auth_user: AuthUser,
    Path(dict_type): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
    let data = service.list_data_by_type(db, &dict_type).await?;
    Ok(ApiResponse::success(data))
}

pub async fn get_data(
    _auth_user: AuthUser,
    Path(data_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
    match service.get_data_by_id(db, data_id).await? {
        Some(d) => Ok(ApiResponse::success(d)),
        None => Ok(ApiResponse::error(404, "Data not found".to_string())),
    }
}

pub async fn create_data(
    _auth_user: AuthUser,
    Json(request): Json<CreateDataRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
    let d = service.create_data(db, request).await?;
    Ok(ApiResponse::success(d))
}

pub async fn update_data(
    _auth_user: AuthUser,
    Path(data_id): Path<Uuid>,
    Json(request): Json<UpdateDataRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
    let d = service.update_data(db, data_id, request).await?;
    Ok(ApiResponse::success(d))
}

pub async fn delete_data(
    _auth_user: AuthUser,
    Path(data_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
    service.delete_data(db, data_id).await?;
    Ok(ApiResponse::success_no_data())
}
