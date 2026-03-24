use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde_json::json;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

use crate::{
    app::state::AppState,
    common::response::ApiResponse,
    features::{
        auth::routes::auth_routes,
        departments::routes::department_routes,
        menus::routes::menu_routes,
        roles::routes::role_routes,
        users::routes::user_routes,
    },
};

pub fn create_router(state: AppState) -> Router {
    // Add CORS layer
    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::any())
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any());

    Router::new()
        .merge(auth_routes())       // Add auth routes
        .merge(menu_routes())       // Add menu routes
        .merge(role_routes())       // Add role routes
        .merge(user_routes())       // Add user routes
        .merge(department_routes()) // Add department routes
        .route("/api/v1/health", get(health_handler))
        .layer(cors_layer)
        .with_state(state)
}

async fn health_handler(State(_state): State<AppState>) -> impl IntoResponse {
    ApiResponse::success(json!({
        "status": "ok",
        "message": "Server is running"
    }))
}