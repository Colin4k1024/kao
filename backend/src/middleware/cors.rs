use tower_http::cors::{Any, CorsLayer};
use std::time::Duration;

pub fn init_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .expose_headers(Any)
        .max_age(Duration::from_secs(86400))
}
