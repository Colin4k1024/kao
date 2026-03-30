use axum::{
  extract::{Json, Path, Query, State},
  response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
  AppState,

  common::{auth::extractor::AuthUser, response::ApiResponse},
};

use super::{
  model::{CreateUserRequest, UpdateUserRequest},
  service::UserService,
};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
  pub page: Option<i64>,
  pub page_size: Option<i64>,
  pub dept_id: Option<Uuid>,
}

pub fn user_routes() -> axum::Router<AppState> {
  axum::Router::new()
    .route("/users", axum::routing::get(list_users))
    .route("/users", axum::routing::post(create_user))
    .route("/users/{id}", axum::routing::get(get_user))
    .route("/users/{id}", axum::routing::put(update_user))
    .route("/users/{id}", axum::routing::delete(delete_user))
}

pub async fn list_users(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Query(query): Query<ListQuery>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let user_service = UserService::new();
  let page = query.page.unwrap_or(1);
  let page_size = query.page_size.unwrap_or(20);
  
  let (users, total) = user_service.list_users(&state.pool, page, page_size, query.dept_id).await?;
  
  Ok(ApiResponse::success(serde_json::json!({
    "items": users,
    "total": total
  })))
}

pub async fn get_user(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let user_service = UserService::new();
  match user_service.get_user_by_id(&state.pool, user_id).await? {
    Some(user) => Ok(ApiResponse::success(user)),
    None => Ok(ApiResponse::error(404, "User not found".to_string())),
  }
}

pub async fn create_user(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let user_service = UserService::new();
  let user = user_service.create_user(&state.pool, request).await?;
  Ok(ApiResponse::success(user))
}

pub async fn update_user(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(user_id): Path<Uuid>,
  Json(request): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let user_service = UserService::new();
  let user = user_service.update_user(&state.pool, user_id, request).await?;
  Ok(ApiResponse::success(user))
}

pub async fn delete_user(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let user_service = UserService::new();
  user_service.delete_user(&state.pool, user_id).await?;
  Ok(ApiResponse::success_no_data())
}