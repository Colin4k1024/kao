use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::Response,
};

pub async fn auth_required(req: Request, next: Next) -> Result<Response, axum::BoxError> {
    Ok(next.run(req).await)
}

pub async fn attach_config(req: Request, next: Next) -> Result<Response, axum::BoxError> {
    Ok(next.run(req).await)
}