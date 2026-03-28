use axum::http::{HeaderMap, HeaderName, HeaderValue, StatusCode};
use axum::body::Body;

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
        // Use a simple hash for etag
        let hash = format!("{}", body.len());
        let etag = Some(format!("\"{}\"", hash));
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
                    .any(|tag| {
                        let trimmed = tag.trim();
                        let etag = self.etag.as_deref().unwrap_or("");
                        // Handle both quoted and unquoted ETags
                        trimmed == etag
                            || trimmed == format!("\"{}\"", etag.trim_matches('"'))
                            || etag.trim_matches('"') == trimmed.trim_matches('"')
                    })
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

/// Extension trait for adding caching to responses
pub trait ResponseExt {
    fn with_cache_control(self, cache_control: CacheControl) -> Self;
    fn with_etag(self, etag: String) -> Self;
}

/// Add extension methods to response types
impl ResponseExt for (HeaderMap, axum::response::Response<Body>) {
    fn with_cache_control(mut self, cache_control: CacheControl) -> Self {
        if let Some(value) = cache_control.header_value() {
            self.0.insert(
                HeaderName::from_static("cache-control"),
                HeaderValue::from_str(&value).expect("HeaderValue"),
            );
        }
        self
    }

    fn with_etag(mut self, etag: String) -> Self {
        self.0.insert(
            HeaderName::from_static("etag"),
            HeaderValue::from_str(&etag).expect("HeaderValue"),
        );
        self
    }
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
        let etag = response.etag.clone().unwrap();
        assert!(response.matches_etag(Some(&etag)));
    }
}
