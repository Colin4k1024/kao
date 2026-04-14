use std::sync::Arc;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::common::middleware::rate_limiter::{RateLimiter, RateLimitConfig, rate_limit_middleware};

/// Create a rate limit middleware for login endpoint
pub fn login_rate_limit_middleware() -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, axum::http::StatusCode>> + Send>> + Clone {
    let rate_limiter = Arc::new(RateLimiter::new(RateLimitConfig::login()));

    move |req: Request, next: Next| {
        let rate_limiter = rate_limiter.clone();
        Box::pin(async move {
            rate_limit_middleware(req, next, rate_limiter).await
        })
    }
}