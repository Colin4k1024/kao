//! Rate limiting middleware for auth endpoints.
//!
//! This module provides rate limiting to prevent brute force attacks.
//! Different limits are applied to different endpoints:
//! - Login: 5 attempts per minute per IP
//! - Register: 3 attempts per hour per IP
//! - Refresh: 10 attempts per hour per IP
//!
//! Uses in-memory storage for development. In production, use Redis.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde::Serialize;

/// Rate limit configuration
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests allowed in the window
    pub max_requests: u32,
    /// Window duration in seconds
    pub window_secs: u64,
}

impl RateLimitConfig {
    pub fn login() -> Self {
        Self {
            max_requests: 5,
            window_secs: 60, // 1 minute
        }
    }

    pub fn register() -> Self {
        Self {
            max_requests: 3,
            window_secs: 3600, // 1 hour
        }
    }

    pub fn refresh() -> Self {
        Self {
            max_requests: 10,
            window_secs: 3600, // 1 hour
        }
    }
}

/// In-memory rate limiter store
pub struct RateLimiter {
    /// Map of IP -> (request count, window start time)
    store: Arc<RwLock<HashMap<String, (u32, Instant)>>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            store: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    /// Check if a request from the given IP is allowed
    pub async fn is_allowed(&self, ip: &str) -> bool {
        let mut store = self.store.write().await;
        let now = Instant::now();
        let window_duration = Duration::from_secs(self.config.window_secs);

        let entry = store.entry(ip.to_string()).or_insert((0, now));

        // Check if window has expired
        if now.duration_since(entry.1) >= window_duration {
            // Reset the window
            *entry = (1, now);
            return true;
        }

        // Check if under limit
        if entry.0 < self.config.max_requests {
            entry.0 += 1;
            true
        } else {
            false
        }
    }

    /// Get remaining requests for an IP
    pub async fn remaining(&self, ip: &str) -> u32 {
        let store = self.store.read().await;
        let now = Instant::now();
        let window_duration = Duration::from_secs(self.config.window_secs);

        if let Some((count, start)) = store.get(ip) {
            if now.duration_since(*start) < window_duration {
                return self.config.max_requests.saturating_sub(*count);
            }
        }
        self.config.max_requests
    }

    /// Get retry-after seconds
    pub async fn retry_after(&self, ip: &str) -> u64 {
        let store = self.store.read().await;
        let now = Instant::now();

        if let Some((_, start)) = store.get(ip) {
            let elapsed = now.duration_since(*start).as_secs();
            let remaining = self.config.window_secs.saturating_sub(elapsed);
            if remaining > 0 {
                return remaining;
            }
        }
        0
    }
}

/// Response type for rate limit exceeded
#[derive(Debug, Serialize)]
pub struct RateLimitExceededResponse {
    pub code: u32,
    pub message: String,
    pub retry_after: u64,
}

/// Rate limit middleware that checks against a rate limiter.
///
/// Returns 429 Too Many Requests if rate limit is exceeded.
pub async fn rate_limit_middleware(
    request: Request,
    next: Next,
    rate_limiter: Arc<RateLimiter>,
) -> Result<Response, StatusCode> {
    // Extract client IP from request
    let client_ip = extract_client_ip(&request);

    if !rate_limiter.is_allowed(&client_ip).await {
        let retry_after = rate_limiter.retry_after(&client_ip).await;

        let response = RateLimitExceededResponse {
            code: 429,
            message: "Too many requests. Please try again later.".to_string(),
            retry_after,
        };

        let json = serde_json::to_string(&response).unwrap_or_else(|_| r#"{"code":429,"message":"Too many requests"}"#.to_string());

        return Ok(Response::builder()
            .status(StatusCode::TOO_MANY_REQUESTS)
            .header("Content-Type", "application/json")
            .header("Retry-After", retry_after.to_string())
            .body(Body::from(json))
            .unwrap());
    }

    Ok(next.run(request).await)
}

/// Extract client IP from request headers or connection info
fn extract_client_ip(request: &Request) -> String {
    // Try X-Forwarded-For header first (for reverse proxy)
    if let Some(forwarded) = request.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            // Take the first IP in the chain
            if let Some(ip) = forwarded_str.split(',').next() {
                return ip.trim().to_string();
            }
        }
    }

    // Try X-Real-IP header
    if let Some(real_ip) = request.headers().get("X-Real-IP") {
        if let Ok(ip) = real_ip.to_str() {
            return ip.trim().to_string();
        }
    }

    // Try connection info
    if let Some(conn_info) = request.extensions().get::<axum::extract::ConnectInfo<std::net::SocketAddr>>() {
        return conn_info.0.ip().to_string();
    }

    // Fallback to unknown
    "unknown".to_string()
}

// Pre-configured rate limiters for different endpoints
lazy_static::lazy_static! {
    pub static ref LOGIN_RATE_LIMITER: Arc<RateLimiter> = Arc::new(RateLimiter::new(RateLimitConfig::login()));
    pub static ref REGISTER_RATE_LIMITER: Arc<RateLimiter> = Arc::new(RateLimiter::new(RateLimitConfig::register()));
    pub static ref REFRESH_RATE_LIMITER: Arc<RateLimiter> = Arc::new(RateLimiter::new(RateLimitConfig::refresh()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_allows_under_limit() {
        let limiter = Arc::new(RateLimiter::new(RateLimitConfig {
            max_requests: 3,
            window_secs: 60,
        }));

        let ip = "192.168.1.1";

        // First 3 requests should be allowed
        assert!(limiter.is_allowed(ip).await);
        assert!(limiter.is_allowed(ip).await);
        assert!(limiter.is_allowed(ip).await);
    }

    #[tokio::test]
    async fn test_rate_limiter_blocks_over_limit() {
        let limiter = Arc::new(RateLimiter::new(RateLimitConfig {
            max_requests: 3,
            window_secs: 60,
        }));

        let ip = "192.168.1.2";

        // First 3 requests allowed
        limiter.is_allowed(ip).await;
        limiter.is_allowed(ip).await;
        limiter.is_allowed(ip).await;

        // 4th should be blocked
        assert!(!limiter.is_allowed(ip).await);
    }

    #[tokio::test]
    async fn test_rate_limiter_remaining() {
        let limiter = Arc::new(RateLimiter::new(RateLimitConfig {
            max_requests: 5,
            window_secs: 60,
        }));

        let ip = "192.168.1.3";

        assert_eq!(limiter.remaining(ip).await, 5);

        limiter.is_allowed(ip).await;
        assert_eq!(limiter.remaining(ip).await, 4);

        limiter.is_allowed(ip).await;
        limiter.is_allowed(ip).await;
        assert_eq!(limiter.remaining(ip).await, 2);
    }
}
