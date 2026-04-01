use axum::{
    extract::{Json, Path, State},
    http::HeaderMap,
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    common::{auth::extractor::AuthUser, response::ApiResponse},
};

use super::{
    model::{CreateRoleRequest, UpdateRoleRequest},
    service::RoleService,
};

pub fn role_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/roles", axum::routing::get(list_roles))
        .route("/roles", axum::routing::post(create_role))
        .route("/roles/:id", axum::routing::get(get_role))
        .route("/roles/:id", axum::routing::put(update_role))
        .route("/roles/:id", axum::routing::delete(delete_role))
}

/// GET /api/v1/roles - List all roles
#[utoipa::path(
    get,
    path = "/api/v1/roles",
    tag = "roles",
    security (
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List roles successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn list_roles(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    _headers: HeaderMap,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    let roles = role_service.list_roles_cached(&state.pool, &state.cache).await?;

    // Add cache headers for client-side caching
    let response = ApiResponse::success(serde_json::json!({
        "items": roles,
        "total": roles.len()
    }));
    let mut axum_response = response.into_response();
    let headers = axum_response.headers_mut();
    headers.insert(
        axum::http::HeaderName::from_static("cache-control"),
        axum::http::HeaderValue::from_static("public, max-age=300"),
    );
    Ok(axum_response)
}

/// GET /api/v1/roles/{id} - Get role by ID
#[utoipa::path(
    get,
    path = "/api/v1/roles/{id}",
    tag = "roles",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role found", body = ApiResponse),
        (status = 404, description = "Role not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_role(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    match role_service.get_role_by_id(&state.pool, role_id).await? {
        Some(role) => Ok(ApiResponse::success(role)),
        None => Ok(ApiResponse::error(404, "Role not found".to_string())),
    }
}

/// POST /api/v1/roles - Create new role
#[utoipa::path(
    post,
    path = "/api/v1/roles",
    tag = "roles",
    security (
        ("bearer_auth" = [])
    ),
    request_body = CreateRoleRequest,
    responses(
        (status = 200, description = "Role created successfully", body = ApiResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn create_role(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(request): Json<CreateRoleRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    let role = role_service.create_role(&state.pool, &state.cache, request).await?;
    Ok(ApiResponse::success(role))
}

/// PUT /api/v1/roles/{id} - Update role
#[utoipa::path(
    put,
    path = "/api/v1/roles/{id}",
    tag = "roles",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    request_body = UpdateRoleRequest,
    responses(
        (status = 200, description = "Role updated successfully", body = ApiResponse),
        (status = 404, description = "Role not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn update_role(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(role_id): Path<Uuid>,
    Json(request): Json<UpdateRoleRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    let role = role_service.update_role(&state.pool, &state.cache, role_id, request).await?;
    Ok(ApiResponse::success(role))
}

/// DELETE /api/v1/roles/{id} - Delete role
#[utoipa::path(
    delete,
    path = "/api/v1/roles/{id}",
    tag = "roles",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = Uuid, Path, description = "Role ID")
    ),
    responses(
        (status = 200, description = "Role deleted successfully", body = ApiResponse),
        (status = 404, description = "Role not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn delete_role(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    role_service.delete_role(&state.pool, &state.cache, role_id).await?;
    Ok(ApiResponse::success_no_data())
}
