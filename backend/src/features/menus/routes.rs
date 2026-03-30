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
        .route("/menus/{id}", axum::routing::get(get_menu))
        .route("/menus/{id}", axum::routing::put(update_menu))
        .route("/menus/{id}", axum::routing::delete(delete_menu))
}

pub async fn get_menus(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    _headers: HeaderMap,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    let menus = menu_service.get_menu_tree(&state.pool).await?;
    Ok(ApiResponse::success(menus))
}

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

pub async fn create_menu(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(request): Json<CreateMenuRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    let menu = menu_service.create_menu(&state.pool, request).await?;
    Ok(ApiResponse::success(menu))
}

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

pub async fn delete_menu(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(menu_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    menu_service.delete_menu(&state.pool, menu_id).await?;
    Ok(ApiResponse::success_no_data())
}