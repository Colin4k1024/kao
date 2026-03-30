use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::common::{db::get_pool, auth::extractor::AuthUser, error::AppError, response::ApiResponse};

use super::{
    model::{CreateTypeRequest, UpdateTypeRequest},
    service::TypeService,
};

pub fn type_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/dictionary/types", axum::routing::get(list_types))
        .route("/dictionary/types", axum::routing::post(create_type))
        .route("/dictionary/types/{id}", axum::routing::get(get_type))
        .route("/dictionary/types/{id}", axum::routing::put(update_type))
        .route("/dictionary/types/{id}", axum::routing::delete(delete_type))
}

pub async fn list_types(
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    let types = service.list_types(db).await?;
    Ok(ApiResponse::success(types))
}

pub async fn get_type(
    _auth_user: AuthUser,
    Path(type_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    match service.get_type_by_id(db, type_id).await? {
        Some(t) => Ok(ApiResponse::success(t)),
        None => Ok(ApiResponse::error(404, "Type not found".to_string())),
    }
}

pub async fn create_type(
    _auth_user: AuthUser,
    Json(request): Json<CreateTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    let t = service.create_type(db, request).await?;
    Ok(ApiResponse::success(t))
}

pub async fn update_type(
    _auth_user: AuthUser,
    Path(type_id): Path<Uuid>,
    Json(request): Json<UpdateTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    let t = service.update_type(db, type_id, request).await?;
    Ok(ApiResponse::success(t))
}

pub async fn delete_type(
    _auth_user: AuthUser,
    Path(type_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    let db = get_pool()
        .ok_or_else(|| AppError::Internal(Some("Database pool not initialized".to_string())))?;
    service.delete_type(db, type_id).await?;
    Ok(ApiResponse::success_no_data())
}
