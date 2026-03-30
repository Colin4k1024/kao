use axum::{
  extract::{Json, Path, State},
  response::IntoResponse,
};
use uuid::Uuid;

use crate::{
  AppState,

  common::{auth::extractor::AuthUser, response::ApiResponse},
};

use super::{
  model::{CreateDepartmentRequest, UpdateDepartmentRequest},
  service::DepartmentService,
};

pub fn department_routes() -> axum::Router<AppState> {
  axum::Router::new()
    .route("/departments", axum::routing::get(list_departments))
    .route("/departments", axum::routing::post(create_department))
    .route("/departments/:id", axum::routing::get(get_department))
    .route("/departments/:id", axum::routing::put(update_department))
    .route("/departments/:id", axum::routing::delete(delete_department))
}

pub async fn list_departments(
  State(state): State<AppState>,
  _auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let dept_service = DepartmentService::new();
  let departments = dept_service.get_department_tree(&state.pool).await?;
  Ok(ApiResponse::success(departments))
}

pub async fn get_department(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(dept_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let dept_service = DepartmentService::new();
  match dept_service.get_department_by_id(&state.pool, dept_id).await? {
    Some(dept) => Ok(ApiResponse::success(dept)),
    None => Ok(ApiResponse::error(404, "Department not found".to_string())),
  }
}

pub async fn create_department(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Json(request): Json<CreateDepartmentRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let dept_service = DepartmentService::new();
  let dept = dept_service.create_department(&state.pool, request).await?;
  Ok(ApiResponse::success(dept))
}

pub async fn update_department(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(dept_id): Path<Uuid>,
  Json(request): Json<UpdateDepartmentRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let dept_service = DepartmentService::new();
  let dept = dept_service.update_department(&state.pool, dept_id, request).await?;
  Ok(ApiResponse::success(dept))
}

pub async fn delete_department(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(dept_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let dept_service = DepartmentService::new();
  dept_service.delete_department(&state.pool, dept_id).await?;
  Ok(ApiResponse::success_no_data())
}