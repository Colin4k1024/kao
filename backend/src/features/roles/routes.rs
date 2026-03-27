use axum::{
    extract::{Json, Path, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
};
use uuid::Uuid;

use crate::{
    AppState,
    common::{auth::extractor::AuthUser, middleware::caching::CacheControl, response::ApiResponse},
};

use super::{
    model::{CreateRoleRequest, UpdateRoleRequest},
    service::RoleService,
};

pub fn role_routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/api/v1/roles", axum::routing::get(list_roles))
        .route("/api/v1/roles", axum::routing::post(create_role))
        .route("/api/v1/roles/{id}", axum::routing::get(get_role))
        .route("/api/v1/roles/{id}", axum::routing::put(update_role))
        .route("/api/v1/roles/{id}", axum::routing::delete(delete_role))
}

pub async fn list_roles(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    headers: HeaderMap,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    let roles = role_service.list_roles(&state.pool).await?;
    
    // Check If-None-Match for conditional requests
    let if_none_match = headers.get("if-none-match");
    
    // Generate ETag
    let body = serde_json::to_string(&roles)?;
    let etag = format!("\"{}\"", hex::encode(md5::compute(&body).0));
    let etag_str = etag.as_str();
    
    // Check if client has cached version
    if let Some(header_value) = if_none_match {
        if header_value.to_str().map(|h| h.contains(etag_str)).unwrap_or(false) {
            return Ok((
                StatusCode::NOT_MODIFIED,
                [(
                    "Cache-Control",
                    "max-age=300, public",
                )],
                "",
            ));
        }
    }
    
    let mut response = ApiResponse::success(roles);
    response.headers_mut().insert(
        HeaderName::from_static("etag"),
        HeaderValue::from_str(&etag).expect("Valid etag"),
    );
    Ok(response)
}

pub async fn get_role(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    match role_service.get_role_by_id(&state.pool, role_id).await? {
        Some(role) => Ok(ApiResponse::success(role)),
        None => Ok(ApiResponse::error(404, "Role not found".to_string())),
    }
}

pub async fn create_role(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(request): Json<CreateRoleRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    let role = role_service.create_role(&state.pool, request).await?;
    Ok(ApiResponse::success(role))
}

pub async fn update_role(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(role_id): Path<Uuid>,
    Json(request): Json<UpdateRoleRequest>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    let role = role_service.update_role(&state.pool, role_id, request).await?;
    Ok(ApiResponse::success(role))
}

pub async fn delete_role(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(role_id): Path<Uuid>,
) -> Result<impl IntoResponse, crate::common::error::AppError> {
    let role_service = RoleService::new();
    role_service.delete_role(&state.pool, role_id).await?;
    Ok(ApiResponse::success_no_data())
}