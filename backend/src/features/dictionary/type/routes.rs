use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::common::{auth::extractor::AuthUser, error::AppError, response::ApiResponse};
use crate::AppState;

use super::{
    model::{CreateTypeRequest, UpdateTypeRequest},
    service::TypeService,
};

pub fn type_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/dictionary/types", axum::routing::get(list_types))
        .route("/dictionary/types", axum::routing::post(create_type))
        .route("/dictionary/types/:id", axum::routing::get(get_type))
        .route("/dictionary/types/:id", axum::routing::put(update_type))
        .route("/dictionary/types/:id", axum::routing::delete(delete_type))
}

pub async fn list_types(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    let types = service.list_types_cached(&state.pool, &state.cache).await?;

    // Add cache headers for client-side caching
    let response = ApiResponse::success(serde_json::json!({
        "list": types,
        "total": types.len()
    }));
    let mut axum_response = response.into_response();
    let headers = axum_response.headers_mut();
    headers.insert(
        axum::http::HeaderName::from_static("cache-control"),
        axum::http::HeaderValue::from_static("public, max-age=300"),
    );
    Ok(axum_response)
}

pub async fn get_type(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(type_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    match service.get_type_by_id(&state.pool, type_id).await? {
        Some(t) => Ok(ApiResponse::success(t)),
        None => Ok(ApiResponse::error(404, "Type not found".to_string())),
    }
}

pub async fn create_type(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(request): Json<CreateTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    let t = service.create_type(&state.pool, &state.cache, request).await?;
    Ok(ApiResponse::success(t))
}

pub async fn update_type(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(type_id): Path<Uuid>,
    Json(request): Json<UpdateTypeRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    let t = service.update_type(&state.pool, &state.cache, type_id, request).await?;
    Ok(ApiResponse::success(t))
}

pub async fn delete_type(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(type_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = TypeService::new();
    service.delete_type(&state.pool, &state.cache, type_id).await?;
    Ok(ApiResponse::success_no_data())
}
