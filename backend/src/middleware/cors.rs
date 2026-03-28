use tower_http::cors::{CorsLayer, AllowCredentials, AllowHeaders, AllowMethods, ExposeHeaders};
use std::time::Duration;
use std::env;
use axum::http::{header, HeaderValue, Method};

/// Initialize CORS layer with restricted origins.
///
/// CORS is restricted to specific origins configured via CORS_ALLOWED_ORIGINS env var.
/// Defaults to localhost:5173 (Vite dev server) if not configured.
/// Production should always set explicit origins.
pub fn init_cors() -> CorsLayer {
    let allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());

    let origins: Vec<HeaderValue> = allowed_origins
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<HeaderValue>().unwrap())
        .collect();

    if origins.is_empty() {
        panic!("CORS_ALLOWED_ORIGINS must contain at least one origin");
    }

    tracing::info!("CORS allowed origins: {:?}", origins);

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods(AllowMethods::list([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ]))
        .allow_headers(AllowHeaders::list([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ORIGIN,
        ]))
        .expose_headers(ExposeHeaders::list([
            header::CONTENT_LENGTH,
            header::CONTENT_TYPE,
        ]))
        .allow_credentials(AllowCredentials::yes())
        .max_age(Duration::from_secs(86400));

    cors
}