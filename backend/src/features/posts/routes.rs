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
    model::{CreatePostRequest, UpdatePostRequest},
    service::PostService,
};

pub fn post_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/posts", axum::routing::get(list_posts))
        .route("/posts", axum::routing::post(create_post))
        .route("/posts/:id", axum::routing::get(get_post))
        .route("/posts/:id", axum::routing::put(update_post))
        .route("/posts/:id", axum::routing::delete(delete_post))
}

pub async fn list_posts(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    _headers: HeaderMap,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let post_service = PostService::new();
    let posts = post_service.list_posts(&state.pool).await?;
    Ok(ApiResponse::success(posts))
}

pub async fn get_post(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(post_id): Path<i64>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let post_service = PostService::new();
    match post_service.get_post_by_id(&state.pool, post_id).await? {
        Some(post) => Ok(ApiResponse::success(post)),
        None => Ok(ApiResponse::error(404, "Post not found".to_string())),
    }
}

pub async fn create_post(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(request): Json<CreatePostRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let post_service = PostService::new();
    let post = post_service
        .create_post(&state.pool, request, None)
        .await?;
    Ok(ApiResponse::success(post))
}

pub async fn update_post(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(post_id): Path<i64>,
    Json(request): Json<UpdatePostRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let post_service = PostService::new();
    let post = post_service
        .update_post(&state.pool, post_id, request)
        .await?;
    Ok(ApiResponse::success(post))
}

pub async fn delete_post(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(post_id): Path<i64>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let post_service = PostService::new();
    post_service.delete_post(&state.pool, post_id).await?;
    Ok(ApiResponse::success_no_data())
}
