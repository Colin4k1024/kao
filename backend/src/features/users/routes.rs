use axum::{
  extract::{Json, Path, Query, State},
  response::IntoResponse,
  http::{StatusCode, HeaderMap},
  http::header::ACCESS_CONTROL_ALLOW_ORIGIN,
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
    .route("/users", axum::routing::options(options_handler))
    .route("/users/:id", axum::routing::get(get_user))
    .route("/users/:id", axum::routing::put(update_user))
    .route("/users/:id", axum::routing::delete(delete_user))
    .route("/users/:id", axum::routing::options(options_handler))
}

/// OPTIONS handler for CORS preflight
async fn options_handler() -> Result<impl IntoResponse, StatusCode> {
  let mut headers = HeaderMap::new();
  headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
  headers.insert(
    axum::http::header::ACCESS_CONTROL_ALLOW_METHODS,
    "GET, POST, PUT, DELETE, PATCH".parse().unwrap(),
  );
  headers.insert(
    axum::http::header::ACCESS_CONTROL_ALLOW_HEADERS,
    "Content-Type, Authorization".parse().unwrap(),
  );
  headers.insert(
    axum::http::header::ACCESS_CONTROL_MAX_AGE,
    "3600".parse().unwrap(),
  );
  
  Ok((StatusCode::NO_CONTENT, headers, ()))
}

/// GET /api/v1/users - List all users
#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "users",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("dept_id" = Option<Uuid>, Query, description = "Department ID to filter by")
    ),
    responses(
        (status = 200, description = "List users successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
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

/// GET /api/v1/users/{id} - Get user by ID
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "users",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User found", body = ApiResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Not authenticated")
    )
)]
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

/// POST /api/v1/users - Create new user
#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "users",
    security (
        ("bearer_auth" = [])
    ),
    request_body = CreateUserRequest,
    responses(
        (status = 200, description = "User created successfully", body = ApiResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn create_user(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Json(request): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let user_service = UserService::new();
  let user = user_service.create_user(&state.pool, request).await?;
  Ok(ApiResponse::success(user))
}

/// PUT /api/v1/users/{id} - Update user
#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "users",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated successfully", body = ApiResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Not authenticated")
    )
)]
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

/// DELETE /api/v1/users/{id} - Delete user
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "users",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User deleted successfully", body = ApiResponse),
        (status = 404, description = "User not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn delete_user(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let user_service = UserService::new();
  user_service.delete_user(&state.pool, user_id).await?;
  Ok(ApiResponse::success_no_data())
}
