use axum::{
    extract::{Json, Path, State},
    http::HeaderMap,
    response::IntoResponse,
};

use crate::{
    AppState,
    common::{auth::extractor::AuthUser, response::ApiResponse},
};

use super::{
    model::CreateMenuRequest,
    service::MenuService,
};

pub fn menu_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/menus", axum::routing::get(get_menus))
        .route("/menus", axum::routing::post(create_menu))
        .route("/menus/:id", axum::routing::get(get_menu))
        .route("/menus/:id", axum::routing::put(update_menu))
        .route("/menus/:id", axum::routing::delete(delete_menu))
}

/// GET /api/v1/menus - Get menu tree
#[utoipa::path(
    get,
    path = "/api/v1/menus",
    tag = "menus",
    security (
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Menu tree retrieved successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_menus(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    _headers: HeaderMap,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    let menus = menu_service.get_menu_tree(&state.pool).await?;
    Ok(ApiResponse::success(menus))
}

/// GET /api/v1/menus/{id} - Get menu by ID
#[utoipa::path(
    get,
    path = "/api/v1/menus/{id}",
    tag = "menus",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = uuid::Uuid, Path, description = "Menu ID")
    ),
    responses(
        (status = 200, description = "Menu found", body = ApiResponse),
        (status = 404, description = "Menu not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_menu(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(menu_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    match menu_service.get_menu_by_id(&state.pool, menu_id).await? {
        Some(menu) => Ok(ApiResponse::success(menu)),
        None => Ok(ApiResponse::error(404, "Menu not found".to_string())),
    }
}

/// POST /api/v1/menus - Create new menu
#[utoipa::path(
    post,
    path = "/api/v1/menus",
    tag = "menus",
    security (
        ("bearer_auth" = [])
    ),
    request_body = CreateMenuRequest,
    responses(
        (status = 200, description = "Menu created successfully", body = ApiResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn create_menu(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(request): Json<CreateMenuRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    let menu = menu_service.create_menu(&state.pool, request).await?;
    Ok(ApiResponse::success(menu))
}

/// PUT /api/v1/menus/{id} - Update menu
#[utoipa::path(
    put,
    path = "/api/v1/menus/{id}",
    tag = "menus",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = uuid::Uuid, Path, description = "Menu ID")
    ),
    request_body = CreateMenuRequest,
    responses(
        (status = 200, description = "Menu updated successfully", body = ApiResponse),
        (status = 404, description = "Menu not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn update_menu(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(menu_id): Path<uuid::Uuid>,
    Json(request): Json<CreateMenuRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    let menu = menu_service.update_menu(&state.pool, menu_id, request).await?;
    Ok(ApiResponse::success(menu))
}

/// DELETE /api/v1/menus/{id} - Delete menu
#[utoipa::path(
    delete,
    path = "/api/v1/menus/{id}",
    tag = "menus",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = uuid::Uuid, Path, description = "Menu ID")
    ),
    responses(
        (status = 200, description = "Menu deleted successfully", body = ApiResponse),
        (status = 404, description = "Menu not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn delete_menu(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(menu_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    menu_service.delete_menu(&state.pool, menu_id).await?;
    Ok(ApiResponse::success_no_data())
}
