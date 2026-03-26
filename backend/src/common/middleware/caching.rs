use axum::{
    body::Body,
    extract::MatchedPath,
    http::{HeaderMap, HeaderName, HeaderValue, Request, StatusCode},
    response::Response,
};
use std::time::{Duration, Instant};
use std::sync::Arc;

/// Cache control settings for a response
#[derive(Debug, Clone)]
pub struct CacheControl {
    pub max_age: u64,
    pub mutable: bool,
    pub cacheable: bool,
}

impl Default for CacheControl {
    fn default() -> Self {
        Self {
            max_age: 60, // 1 minute default
            mutable: false,
            cacheable: true,
        }
    }
}

impl CacheControl {
    /// Create a new CacheControl with specified max-age
    pub fn new(max_age: u64) -> Self {
        Self {
            max_age,
            ..Default::default()
        }
    }

    /// Create immutable cache control (for public, long-lived data)
    pub fn immutable(max_age: u64) -> Self {
        Self {
            max_age,
            mutable: false,
            cacheable: true,
        }
    }

    /// Create mutable cache control (for data that may change)
    pub fn mutable(max_age: u64) -> Self {
        Self {
            max_age,
            mutable: true,
            cacheable: true,
        }
    }

    /// Generate Cache-Control header value
    pub fn header_value(&self) -> Option<String> {
        if !self.cacheable {
            return None;
        }

        let mut value = format!("max-age={}", self.max_age);
        if !self.mutable {
            value.push_str(", immutable");
        }
        Some(value)
    }
}

/// Cache response wrapper
#[derive(Debug, Clone)]
pub struct CacheResponse {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: String,
    pub etag: Option<String>,
    pub cache_control: Option<String>,
}

impl CacheResponse {
    /// Create a new CacheResponse
    pub fn new(status: StatusCode, body: String) -> Self {
        let etag = Some(format!("\"{}\"", md5::compute(&body)));
        Self {
            status,
            headers: HeaderMap::new(),
            body,
            etag,
            cache_control: None,
        }
    }

    /// Set cache control headers
    pub fn with_cache_control(mut self, cache_control: CacheControl) -> Self {
        self.cache_control = cache_control.header_value();
        self
    }

    /// Set ETag
    pub fn with_etag(mut self, etag: String) -> Self {
        self.etag = Some(etag);
        self
    }

    /// Check if request matches ETag
    pub fn matches_etag(&self, if_none_match: Option<&str>) -> bool {
        if_none_match
            .map(|header| {
                header
                    .split(',')
                    .any(|tag| tag.trim().trim_matches('"') == self.etag.as_deref().unwrap_or(""))
            })
            .unwrap_or(false)
    }
}

/// Cache manager for generating cache keys
pub struct CacheManager;

impl CacheManager {
    /// Generate cache key for a request
    pub fn generate_cache_key(path: &str, method: &str, query: &str) -> String {
        format!("{}:{}:{}", method, path, query)
    }

    /// Generate cache key for user data
    pub fn user_cache_key(user_id: &str) -> String {
        format!("user:{}", user_id)
    }

    /// Generate cache key for menu tree
    pub fn menu_cache_key() -> &'static str {
        "menu:tree"
    }

    /// Generate cache key for role list
    pub fn role_cache_key() -> &'static str {
        "role:list"
    }

    /// Generate cache key for department tree
    pub fn department_cache_key() -> &'static str {
        "department:tree"
    }
}

/// Middleware to add caching headers
pub struct CacheControlMiddleware;

impl CacheControlMiddleware {
    /// Create new cache control middleware
    pub fn new() -> Self {
        Self
    }
}

impl Default for CacheControlMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

///Extension trait for adding caching to responses
pub trait ResponseExt {
    fn with_cache_control(self, cache_control: CacheControl) -> Self;
    fn with_etag(self, etag: String) -> Self;
    fn is_cacheable(&self) -> bool;
}

impl ResponseExt for Response<Body> {
    fn with_cache_control(mut self, cache_control: CacheControl) -> Self {
        if let Some(value) = cache_control.header_value() {
            self.headers_mut().insert(
                HeaderName::from_static("cache-control"),
                HeaderValue::from_static(&value),
            );
        }
        self
    }

    fn with_etag(mut self, etag: String) -> Self {
        self.headers_mut().insert(
            HeaderName::from_static("etag"),
            HeaderValue::from_str(&etag).unwrap_or_default(),
        );
        self
    }

    fn is_cacheable(&self) -> bool {
        self.status().is_success()
    }
}

/// Cache middleware implementation
pub async fn cache_middleware(
    req: Request<Body>,
    next: axum::middleware::Next,
) -> Response<Body> {
    let uri = req.uri().clone();
    let method = req.method().clone();
    let path = uri.path().to_string();
    let query = uri.query().unwrap_or("").to_string();

    // Generate cache key
    let cache_key = CacheManager::generate_cache_key(&path, method.as_str(), &query);

    // Check if request has If-None-Match header
    let if_none_match = req
        .headers()
        .get("if-none-match")
        .and_then(|h| h.to_str().ok());

    // Check if we have cached response (simplified - would use Redis in production)
    if let Some cached_etag) = get_cached_response(&cache_key) {
        if cached_etag.matches_etag(if_none_match) {
            return Response::builder()
                .status(StatusCode::NOT_MODIFIED)
                .header("cache-control", "max-age=60")
                .body(Body::empty())
                .unwrap();
        }
    }

    // Execute request
    let response = next.run(req).await;

    // Add caching headers for GET requests
    if method.as_str() == "GET" && response.status().is_success() {
        response.with_cache_control(CacheControl::new(60))
    } else {
        response
    }
}

/// In-memory cache storage (placeholder for Redis)
fn get_cached_response(_key: &str) -> Option<CacheResponse> {
    // In production, this would fetch from Redis
    // For now, return None to always miss
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_control_default() {
        let cc = CacheControl::default();
        assert_eq!(cc.max_age, 60);
        assert!(!cc.mutable);
        assert!(cc.cacheable);
    }

    #[test]
    fn test_cache_control_header_value() {
        let cc = CacheControl::immutable(3600);
        let value = cc.header_value().unwrap();
        assert!(value.contains("max-age=3600"));
        assert!(value.contains("immutable"));
    }

    #[test]
    fn test_etag_matching() {
        let response = CacheResponse::new(StatusCode::OK, "test body".to_string());
        let etag = response.etag.unwrap();
        assert!(response.matches_etag(Some(&etag)));
    }
}
