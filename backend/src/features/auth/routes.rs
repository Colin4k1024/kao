use axum::{
    extract::{State},
    middleware::from_fn_with_state,
    routing::{get, post},
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
    model::{LoginRequest, LoginResponse, PermissionsResponse, ProfileResponse},
    service::AuthService,
};

pub fn router(state: AppState) -> Router {
    let protected_state = state.clone();

    Router::new()
        .route("/api/v1/auth/login", post(login))
        .merge(
            Router::new()
                .route("/api/v1/auth/profile", get(profile))
                .route("/api/v1/auth/permissions", get(permissions))
                .layer(from_fn_with_state(protected_state, auth_middleware)),
        )
        .with_state(state)
}

async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> AppResult<Json<ApiResponse<LoginResponse>>> {
    let service = AuthService::new(state.db.clone());
    let response = service.login(&state.config, request).await?;
    Ok(Json(ApiResponse::success(response)))
}

async fn profile(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
) -> AppResult<Json<ApiResponse<ProfileResponse>>> {
    let service = AuthService::new(state.db.clone());
    let response = service.profile(&claims.sub).await?;
    Ok(Json(ApiResponse::success(response)))
}

async fn permissions(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
) -> AppResult<Json<ApiResponse<PermissionsResponse>>> {
    let service = AuthService::new(state.db.clone());
    let response = service.permissions(&claims.sub).await?;
    Ok(Json(ApiResponse::success(response)))
}
