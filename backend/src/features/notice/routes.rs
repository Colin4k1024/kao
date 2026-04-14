use axum::{
  extract::{Path, Query, State},
  response::IntoResponse,
  Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{AppState, common::{auth::extractor::AuthUser, error::AppError, response::ApiResponse}};

use super::{
  model::{CreateNoticeRequest, UpdateNoticeRequest},
  service::NoticeService,
};

#[derive(Debug, Deserialize)]
pub struct ListQueryParams {
  pub page: Option<usize>,
  pub page_size: Option<usize>,
  pub notice_title: Option<String>,
  pub notice_type: Option<String>,
  pub notice_status: Option<String>,
}

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
  Query(params): Query<ListQueryParams>,
) -> Result<impl IntoResponse, AppError> {
  let service = NoticeService::new();
  let page = params.page.unwrap_or(1);
  let page_size = params.page_size.unwrap_or(10);
  let notices = service.list_notices(&state.pool, params.notice_type.as_deref(), params.notice_status.as_deref()).await?;
  
  // Simple pagination
  let total = notices.len();
  let start = (page - 1) * page_size;
  let end = start + page_size;
  let items: Vec<_> = notices.into_iter().skip(start).take(page_size).collect();
  
  Ok(ApiResponse::success(serde_json::json!({
      "items": items,
      "total": total,
      "page": page,
      "pageSize": page_size
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
