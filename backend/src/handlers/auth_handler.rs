use crate::error::AppResult;
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::models::user::{CreateUserRequest, UserResponse};
use crate::services::auth_service;
use crate::services::user_service::UserService;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{post, Router},
    Json, Router,
};
use std::sync::Arc;

pub fn auth_routes(user_service: Arc<UserService>) -> Router {
    Router::new()
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/register", post(register))
        .with_state(user_service)
}

async fn login(
    State(user_service): State<Arc<UserService>>,
    Json(req): Json<LoginRequest>,
) -> AppResult<impl IntoResponse> {
    let user = user_service.verify_password(&req.email, &req.password).await?;
    let token = auth_service::generate_token(&user.id, &user.email)?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            access_token: token,
            user: UserResponse::from(user),
        }),
    ))
}

async fn register(
    State(user_service): State<Arc<UserService>>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<impl IntoResponse> {
    let user = user_service.create(req).await?;
    let token = auth_service::generate_token(&user.id, &user.email)?;

    Ok((
        StatusCode::CREATED,
        Json(LoginResponse {
            access_token: token,
            user,
        }),
    ))
}
