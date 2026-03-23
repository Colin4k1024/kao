use crate::app::AppState;
use crate::common::{
    auth::{claims::Claims, jwt},
    error::{AppError, AppResult},
};
use axum::{
    body::Body,
    extract::State,
    http::{header::AUTHORIZATION, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    match authenticate(
        &state,
        request
            .headers()
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok()),
    ) {
        Ok(claims) => {
            request.extensions_mut().insert(claims);
            next.run(request).await
        }
        Err(err) => err.into_response(),
    }
}

fn authenticate(state: &AppState, header: Option<&str>) -> AppResult<Claims> {
    let token = header
        .and_then(parse_bearer_token)
        .ok_or_else(|| AppError::Unauthorized("missing bearer token".to_string()))?;

    jwt::decode(token, &state.config)
}

fn parse_bearer_token(value: &str) -> Option<&str> {
    value
        .strip_prefix("Bearer ")
        .map(str::trim)
        .filter(|token| !token.is_empty())
}
