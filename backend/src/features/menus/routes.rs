use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{
    app::state::AppState,
    common::{auth::extractor::AuthUser, response::ApiResponse},
};

use super::{
    model::{CreateMenuRequest, MenuTreeItem},
    service::MenuService,
};

pub fn menu_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/api/v1/menus", axum::routing::get(get_menus))
        .route("/api/v1/menus", axum::routing::post(create_menu))
        .route("/api/v1/menus/{id}", axum::routing::get(get_menu))
        .route("/api/v1/menus/{id}", axum::routing::put(update_menu))
        .route("/api/v1/menus/{id}", axum::routing::delete(delete_menu))
}

pub async fn get_menus(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    let menus = menu_service.get_menu_tree(&state.db).await?;
    Ok(ApiResponse::success(menus))
}

pub async fn get_menu(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(menu_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    match menu_service.get_menu_by_id(&state.db, menu_id).await? {
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
    let menu = menu_service.create_menu(&state.db, request).await?;
    Ok(ApiResponse::success(menu))
}

pub async fn update_menu(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(menu_id): Path<uuid::Uuid>,
    Json(request): Json<CreateMenuRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    let menu = menu_service.update_menu(&state.db, menu_id, request).await?;
    Ok(ApiResponse::success(menu))
}

pub async fn delete_menu(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(menu_id): Path<uuid::Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    menu_service.delete_menu(&state.db, menu_id).await?;
    Ok(ApiResponse::success_no_data())
}