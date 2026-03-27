// Metrics middleware for Axum
//
// This middleware automatically collects metrics for all HTTP requests
// including request duration, method, path, and status codes.

use axum::{
    extract::MatchedPath,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;

use crate::common::metrics;

// Metrics middleware function that records metrics for each request
pub async fn metrics_middleware<B>(
    request: Request<B>,
    next: Next,
) -> Response
where
    B: axum::body::HttpBody + Send + 'static,
    B::Data: Send,
{
    // Get matched path for metrics
    let matched_path = request
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let method = request.method().clone();
    let start = Instant::now();

    // Record request start
    metrics::record_request(method.clone().as_str(), &matched_path, 200);
    metrics::increment_active_connections();

    // Process the request
    let response = next.run(request).await;

    // Calculate duration
    let duration = start.elapsed().as_secs_f64();

    // Get status code
    let status = response.status().as_u16();

    // Record request completion
    metrics::record_request_duration(
        method.as_str(),
        &matched_path,
        status,
        duration,
    );

    // Record errors
    if status >= 400 {
        metrics::record_error(status);
    }

    // Decrement active connections
    metrics::decrement_active_connections();

    response
}

// Helper function to record database query time
pub fn record_query_duration(query_name: &str, duration: std::time::Duration) {
    metrics::record_db_query_time(query_name, duration.as_secs_f64());
}

// Helper function to record cache operations
pub fn record_cache_operation(hit: bool) {
    if hit {
        metrics::record_cache_hit();
    } else {
        metrics::record_cache_miss();
    }
}

// Middleware for password validation failures
pub fn record_password_failure() {
    metrics::record_password_failure();
}

// Middleware for audit log latency
pub fn record_audit_log_latency(log_type: &str, duration: std::time::Duration) {
    metrics::record_audit_log_latency(log_type, duration.as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_record_query_duration() {
        let duration = std::time::Duration::from_millis(50);
        record_query_duration("get_user_by_id", duration);
        // Just verify the function doesn't panic
    }

    #[test]
    fn test_record_cache_hit() {
        record_cache_operation(true);
        // Just verify the function doesn't panic
    }

    #[test]
    fn test_record_cache_miss() {
        record_cache_operation(false);
        // Just verify the function doesn't panic
    }
}
