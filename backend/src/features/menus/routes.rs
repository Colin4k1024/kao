use axum::{
    extract::{State},
    middleware::from_fn_with_state,
    routing::get,
    Json, Router,
};

use crate::{
    app::AppState,
    common::{
        auth::{extractor::CurrentClaims, middleware::auth_middleware},
        error::AppResult,
        response::ApiResponse,
    },
};

use super::{
    model::MenuTreeResponse,
    service::MenusService,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/auth/menus", get(current_menus))
        .layer(from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state)
}

async fn current_menus(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
) -> AppResult<Json<ApiResponse<MenuTreeResponse>>> {
    let service = MenusService::new(state.db.clone());
    let response = service.list_menu_tree_by_user_id(&claims.sub).await?;
    Ok(Json(ApiResponse::success(response)))
}
