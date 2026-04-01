use axum::{
  extract::{Path, State},
  response::IntoResponse,
  Json,
};
use uuid::Uuid;

use crate::{AppState, common::{auth::extractor::AuthUser, error::AppError, response::ApiResponse}};

use super::{
  model::{CreateNoticeRequest, UpdateNoticeRequest},
  service::NoticeService,
};

pub fn notice_routes() -> axum::Router<crate::AppState> {
  axum::Router::new()
    .route("/notice", axum::routing::get(list_notices))
    .route("/notice", axum::routing::post(create_notice))
    .route("/notice/:id", axum::routing::get(get_notice))
    .route("/notice/:id", axum::routing::put(update_notice))
    .route("/notice/:id", axum::routing::delete(delete_notice))
    .route("/notice/:id/view", axum::routing::post(increment_view))
}

pub async fn list_notices(
  State(state): State<AppState>,
  _auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let notices = service.list_notices(&state.pool, None, None).await?;
  Ok(ApiResponse::success(serde_json::json!({
      "items": notices,
      "total": notices.len()
  })))
}

pub async fn get_notice(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(notice_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  match service.get_notice_by_id(&state.pool, notice_id).await? {
    Some(n) => Ok(ApiResponse::success(n)),
    None => Ok(ApiResponse::error(404, "Notice not found".to_string())),
  }
}

pub async fn create_notice(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Json(request): Json<CreateNoticeRequest>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let n = service.create_notice(&state.pool, request).await?;
  Ok(ApiResponse::success(n))
}

pub async fn update_notice(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(notice_id): Path<Uuid>,
  Json(request): Json<UpdateNoticeRequest>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let n = service.update_notice(&state.pool, notice_id, request).await?;
  Ok(ApiResponse::success(n))
}

pub async fn delete_notice(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(notice_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  service.delete_notice(&state.pool, notice_id).await?;
  Ok(ApiResponse::success_no_data())
}

pub async fn increment_view(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(notice_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let view_count = service.increment_view_count(&state.pool, notice_id).await?;
  Ok(ApiResponse::success(view_count))
}
