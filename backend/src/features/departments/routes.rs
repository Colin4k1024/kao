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
    model::{CreateDepartmentRequest, DepartmentResponse, DepartmentsResponse, UpdateDepartmentRequest},
    service::DepartmentsService,
};

pub fn router(state: AppState) -> Router {
    let protected_state = state.clone();

    Router::new()
        .route("/api/v1/departments/tree", get(list_department_tree))
        .route("/api/v1/departments", post(create_department))
        .route("/api/v1/departments/:id", put(update_department))
        .layer(from_fn_with_state(protected_state, auth_middleware))
        .with_state(state)
}

async fn list_department_tree(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
) -> AppResult<Json<ApiResponse<DepartmentsResponse>>> {
    let service = DepartmentsService::new(state.db.clone());
    let response = service.list_department_tree(&claims).await?;
    Ok(Json(ApiResponse::success(response)))
}

async fn create_department(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
    Json(request): Json<CreateDepartmentRequest>,
) -> AppResult<Json<ApiResponse<DepartmentResponse>>> {
    let service = DepartmentsService::new(state.db.clone());
    let response = service.create_department(&claims, request).await?;
    Ok(Json(ApiResponse::success(response)))
}

async fn update_department(
    State(state): State<AppState>,
    CurrentClaims(claims): CurrentClaims,
    Path(department_id): Path<String>,
    Json(request): Json<UpdateDepartmentRequest>,
) -> AppResult<Json<ApiResponse<DepartmentResponse>>> {
    let service = DepartmentsService::new(state.db.clone());
    let response = service
        .update_department(&claims, &department_id, request)
        .await?;
    Ok(Json(ApiResponse::success(response)))
}
