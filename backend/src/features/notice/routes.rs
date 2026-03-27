use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
  Json,
};
use uuid::Uuid;

use crate::common::{auth::extractor::AuthUser, db::{get_pool, check_health}, error::AppError, response::ApiResponse};
use crate::AppState;

use super::{
  model::{CreateNoticeRequest, UpdateNoticeRequest, NoticeResponse},
  service::NoticeService,
};

pub fn notice_routes() -> axum::Router<()> {
  axum::Router::new()
    .route("/api/system/notice", axum::routing::get(list_notices))
    .route("/api/system/notice", axum::routing::post(create_notice))
    .route("/api/system/notice/{id}", axum::routing::get(get_notice))
    .route("/api/system/notice/{id}", axum::routing::put(update_notice))
    .route("/api/system/notice/{id}", axum::routing::delete(delete_notice))
    .route("/api/system/notice/{id}/view", axum::routing::post(increment_view))
}

pub async fn list_notices(
  _auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let db = get_pool()
    .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
  let notices = service.list_notices(&db, None, None).await?;
  Ok(ApiResponse::success(notices))
}

pub async fn get_notice(
  _auth_user: AuthUser,
  Path(notice_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let db = get_pool()
    .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
  match service.get_notice_by_id(&db, notice_id).await? {
    Some(n) => Ok(ApiResponse::success(n)),
    None => Ok(ApiResponse::error(404, "Notice not found".to_string())),
  }
}

pub async fn create_notice(
  _auth_user: AuthUser,
  Json(request): Json<CreateNoticeRequest>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let db = get_pool()
    .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
  let n = service.create_notice(&db, request).await?;
  Ok(ApiResponse::success(n))
}

pub async fn update_notice(
  _auth_user: AuthUser,
  Path(notice_id): Path<Uuid>,
  Json(request): Json<UpdateNoticeRequest>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let db = get_pool()
    .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
  let n = service.update_notice(&db, notice_id, request).await?;
  Ok(ApiResponse::success(n))
}

pub async fn delete_notice(
  _auth_user: AuthUser,
  Path(notice_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let db = get_pool()
    .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
  service.delete_notice(&db, notice_id).await?;
  Ok(ApiResponse::success_no_data())
}

pub async fn increment_view(
  _auth_user: AuthUser,
  Path(notice_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let db = get_pool()
    .ok_or_else(|| AppError::Internal("Database pool not initialized".to_string()))?;
  let view_count = service.increment_view_count(&db, notice_id).await?;
  Ok(ApiResponse::success(view_count))
}
