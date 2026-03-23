use axum::{
    extract::{Path, State},
    middleware::from_fn_with_state,
    routing::{get, post, put},
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
    model::{CreateUserRequest, UpdateUserRequest, UserResponse, UsersResponse},
    service::UsersService,
};

pub fn router(state: AppState) -> Router {
    let protected_state = state.clone();

    Router::new()
        .route("/api/v1/users", get(list_users).post(create_user))
        .route("/api/v1/users/:id", put(update_user))
        .layer(from_fn_with_state(protected_state, auth_middleware))
        .with_state(state)
}

async fn list_users(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
) -> AppResult<Json<ApiResponse<UsersResponse>>> {
    let service = UsersService::new(state.db.clone());
    let response = service.list_users(&claims).await?;
    Ok(Json(ApiResponse::success(response)))
}

async fn create_user(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
    Json(request): Json<CreateUserRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let service = UsersService::new(state.db.clone());
    let response = service.create_user(&claims, request).await?;
    Ok(Json(ApiResponse::success(response)))
}

async fn update_user(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
    Path(user_id): Path<String>,
    Json(request): Json<UpdateUserRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let service = UsersService::new(state.db.clone());
    let response = service.update_user(&claims, &user_id, request).await?;
    Ok(Json(ApiResponse::success(response)))
}
