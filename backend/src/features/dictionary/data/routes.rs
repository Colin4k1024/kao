use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::common::{auth::extractor::AuthUser, error::AppError, response::ApiResponse};
use crate::AppState;

use super::{
    model::{CreateDataRequest, UpdateDataRequest},
    service::DataService,
};

pub fn data_routes() -> axum::Router<crate::AppState> {
    axum::Router::new()
        .route("/dictionary/data", axum::routing::get(list_data))
        .route("/dictionary/data", axum::routing::post(create_data))
        .route("/dictionary/data/:id", axum::routing::get(get_data))
        .route("/dictionary/data/:id", axum::routing::put(update_data))
        .route("/dictionary/data/:id", axum::routing::delete(delete_data))
        .route("/dictionary/data/type/:dict_type", axum::routing::get(list_data_by_type))
}

pub async fn list_data(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    // Return empty list for empty type query - cached version not needed for this edge case
    let data = service.list_data_by_type(&state.pool, "").await?;
    Ok(ApiResponse::success(serde_json::json!({
        "list": data,
        "total": data.len()
    })))
}

pub async fn list_data_by_type(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(dict_type): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    let data = service.list_data_by_type_cached(&state.pool, &state.cache, &dict_type).await?;

    // Add cache headers for client-side caching
    let response = ApiResponse::success(data);
    let mut axum_response = response.into_response();
    let headers = axum_response.headers_mut();
    headers.insert(
        axum::http::HeaderName::from_static("cache-control"),
        axum::http::HeaderValue::from_static("public, max-age=300"),
    );
    Ok(axum_response)
}

pub async fn get_data(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(data_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    match service.get_data_by_id(&state.pool, data_id).await? {
        Some(d) => Ok(ApiResponse::success(d)),
        None => Ok(ApiResponse::error(404, "Data not found".to_string())),
    }
}

pub async fn create_data(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(request): Json<CreateDataRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    let d = service.create_data(&state.pool, &state.cache, request).await?;
    Ok(ApiResponse::success(d))
}

pub async fn update_data(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(data_id): Path<Uuid>,
    Json(request): Json<UpdateDataRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    let d = service.update_data(&state.pool, &state.cache, data_id, request).await?;
    Ok(ApiResponse::success(d))
}

pub async fn delete_data(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(data_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {
    let service = DataService::new();
    service.delete_data(&state.pool, &state.cache, data_id).await?;
    Ok(ApiResponse::success_no_data())
}
