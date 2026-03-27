use axum::{
    extract::{Json, Path, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde_json::json;

use crate::{
    AppState,
    common::{auth::extractor::AuthUser, middleware::caching::CacheControl, response::ApiResponse},
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
    headers: HeaderMap,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let menu_service = MenuService::new();
    let menus = menu_service.get_menu_tree(&state.pool).await?;
    
    // Check If-None-Match for conditional requests
    let if_none_match = headers.get("if-none-match");
    
    // Generate ETag
    let body = serde_json::to_string(&menus)?;
    let etag = format!("\"{}\"", hex::encode(md5::compute(&body).0));
    let etag_str = etag.as_str();
    
    // Check if client has cached version
    if let Some(header_value) = if_none_match {
        if header_value.to_str().map(|h| h.contains(etag_str)).unwrap_or(false) {
            return Ok((
                StatusCode::NOT_MODIFIED,
                [(
                    "Cache-Control",
                    "max-age=900, immutable",
                )],
                "",
            ));
        }
    }
    
    let mut response = ApiResponse::success(menus);
    response.headers_mut().insert(
        HeaderName::from_static("etag"),
        HeaderValue::from_str(&etag).expect("Valid etag"),
    );
    Ok(response)
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