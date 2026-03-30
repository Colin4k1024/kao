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

/// GET /api/v1/departments - Get department tree
#[utoipa::path(
    get,
    path = "/api/v1/departments",
    tag = "departments",
    security (
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Department tree retrieved successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn list_departments(
  State(state): State<AppState>,
  _auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let dept_service = DepartmentService::new();
  let departments = dept_service.get_department_tree(&state.pool).await?;
  Ok(ApiResponse::success(departments))
}

/// GET /api/v1/departments/{id} - Get department by ID
#[utoipa::path(
    get,
    path = "/api/v1/departments/{id}",
    tag = "departments",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Department ID")
    ),
    responses(
        (status = 200, description = "Department found", body = ApiResponse),
        (status = 404, description = "Department not found"),
        (status = 401, description = "Not authenticated")
    )
)]
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

/// POST /api/v1/departments - Create new department
#[utoipa::path(
    post,
    path = "/api/v1/departments",
    tag = "departments",
    security (
        ("bearer_auth" = [])
    ),
    request_body = CreateDepartmentRequest,
    responses(
        (status = 200, description = "Department created successfully", body = ApiResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn create_department(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Json(request): Json<CreateDepartmentRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let dept_service = DepartmentService::new();
  let dept = dept_service.create_department(&state.pool, request).await?;
  Ok(ApiResponse::success(dept))
}

/// PUT /api/v1/departments/{id} - Update department
#[utoipa::path(
    put,
    path = "/api/v1/departments/{id}",
    tag = "departments",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Department ID")
    ),
    request_body = UpdateDepartmentRequest,
    responses(
        (status = 200, description = "Department updated successfully", body = ApiResponse),
        (status = 404, description = "Department not found"),
        (status = 401, description = "Not authenticated")
    )
)]
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

/// DELETE /api/v1/departments/{id} - Delete department
#[utoipa::path(
    delete,
    path = "/api/v1/departments/{id}",
    tag = "departments",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Department ID")
    ),
    responses(
        (status = 200, description = "Department deleted successfully", body = ApiResponse),
        (status = 404, description = "Department not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn delete_department(
  State(state): State<AppState>,
  _auth_user: AuthUser,
  Path(dept_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
  let dept_service = DepartmentService::new();
  dept_service.delete_department(&state.pool, dept_id).await?;
  Ok(ApiResponse::success_no_data())
}
