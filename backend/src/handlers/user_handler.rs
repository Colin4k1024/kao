use crate::error::AppResult;
use crate::models::auth::Claims;
use crate::models::user::UserResponse;
use crate::services::user_service::UserService;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;

pub fn user_routes(user_service: Arc<UserService>) -> Router {
    Router::new()
        .route("/api/v1/users/me", get(get_current_user))
        .with_state(user_service)
}

async fn get_current_user(
    Extension(claims): Extension<Claims>,
    State(user_service): State<Arc<UserService>>,
) -> AppResult<impl IntoResponse> {
    let user = user_service.find_by_id(&claims.sub).await?;
    Ok((StatusCode::OK, Json(UserResponse::from(user))))
}
