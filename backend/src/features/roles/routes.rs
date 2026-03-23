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
    model::{CreateRoleRequest, RoleResponse, RolesResponse, UpdateRoleRequest},
    service::RolesService,
};

pub fn router(state: AppState) -> Router {
    let protected_state = state.clone();

    Router::new()
        .route("/api/v1/roles", get(list_roles).post(create_role))
        .route("/api/v1/roles/:id", put(update_role))
        .layer(from_fn_with_state(protected_state, auth_middleware))
        .with_state(state)
}

async fn list_roles(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
) -> AppResult<Json<ApiResponse<RolesResponse>>> {
    let service = RolesService::new(state.db.clone());
    let response = service.list_roles(&claims).await?;
    Ok(Json(ApiResponse::success(response)))
}

async fn create_role(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
    Json(request): Json<CreateRoleRequest>,
) -> AppResult<Json<ApiResponse<RoleResponse>>> {
    let service = RolesService::new(state.db.clone());
    let response = service.create_role(&claims, request).await?;
    Ok(Json(ApiResponse::success(response)))
}

async fn update_role(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
    Path(role_id): Path<String>,
    Json(request): Json<UpdateRoleRequest>,
) -> AppResult<Json<ApiResponse<RoleResponse>>> {
    let service = RolesService::new(state.db.clone());
    let response = service.update_role(&claims, &role_id, request).await?;
    Ok(Json(ApiResponse::success(response)))
}
