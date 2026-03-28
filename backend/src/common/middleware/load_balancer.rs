use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

/// Cookie name for sticky sessions
pub const LB_COOKIE: &str = "lb_session";
/// Request ID header
pub const X_REQUEST_ID: &str = "x-request-id";
/// Backend instance ID header
pub const X_BACKEND_INSTANCE: &str = "x-backend-instance";

/// Sticky session cookie data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LbCookie {
    pub session_id: String,
    pub instance_id: String,
    pub created_at: u64,
}

impl LbCookie {
    /// Create a new sticky session cookie
    pub fn new(instance_id: String) -> Self {
        Self {
            session_id: Uuid::new_v4().to_string(),
            instance_id,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    /// Serialize cookie to JSON string
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// Deserialize cookie from JSON string
    pub fn from_json_str(s: &str) -> Option<Self> {
        serde_json::from_str(s).ok()
    }
}

/// Load balancer middleware configuration
#[derive(Clone)]
pub struct LoadBalancer {
    #[allow(dead_code)]
    sticky_sessions: bool,
    instance_id: String,
    cookie_max_age: u64,
    #[allow(dead_code)]
    enable_tracing: bool,
    request_counts: Arc<std::sync::Mutex<HashMap<String, u64>>>,
}

impl LoadBalancer {
    /// Create a new load balancer middleware
    pub fn new() -> Self {
        let sticky_sessions = env::var("STICKY_SESSIONS")
            .unwrap_or_else(|_| "true".to_string())
            .to_lowercase()
            == "true";
        
        let instance_id = env::var("INSTANCE_ID")
            .unwrap_or_else(|_| format!("instance-{}", &Uuid::new_v4().to_string()[..8]));
        
        let cookie_max_age = env::var("COOKIE_MAX_AGE")
            .unwrap_or_else(|_| "86400".to_string()) // 24 hours
            .parse()
            .unwrap_or(86400);
        
        let enable_tracing = env::var("ENABLE_TRACING")
            .unwrap_or_else(|_| "true".to_string())
            .to_lowercase()
            == "true";
        
        Self {
            sticky_sessions,
            instance_id,
            cookie_max_age,
            enable_tracing,
            request_counts: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Create load balancer with custom configuration
    pub fn with_config(
        sticky_sessions: bool,
        instance_id: String,
        cookie_max_age: u64,
        enable_tracing: bool,
    ) -> Self {
        Self {
            sticky_sessions,
            instance_id,
            cookie_max_age,
            enable_tracing,
            request_counts: Arc::new(std::sync::Mutex::new(HashMap::new())),
        }
    }

    /// Extract instance ID from cookie
    pub fn extract_instance_id_from_cookie(&self, req: &Request) -> Option<String> {
        let cookie_header = req.headers().get("cookie")?;
        let cookie_str = cookie_header.to_str().ok()?;
        
        for cookie in cookie_str.split(';') {
            let cookie = cookie.trim();
            if cookie.starts_with(format!("{}=", LB_COOKIE).as_str()) {
                let value = cookie.trim_start_matches(format!("{}=", LB_COOKIE).as_str());
                return LbCookie::from_json_str(value).map(|c| c.instance_id);
            }
        }
        
        None
    }

    /// Add sticky session cookie to response
    pub fn add_sticky_cookie(&self, response: &mut Response) {
        let cookie = LbCookie::new(self.instance_id.clone());
        let cookie_str = format!(
            "{}={}; Max-Age={}; HttpOnly; Secure; SameSite=Lax",
            LB_COOKIE,
            cookie.to_json_string(),
            self.cookie_max_age
        );
        
        response.headers_mut().append(
            "set-cookie",
            HeaderValue::from_str(&cookie_str).unwrap(),
        );
    }

    /// Generate a unique request ID
    pub fn generate_request_id(&self) -> String {
        Uuid::new_v4().to_string()
    }

    /// Add request ID to response headers
    pub fn add_request_id(&self, response: &mut Response, request_id: &str) {
        response.headers_mut().insert(
            HeaderName::from_static(X_REQUEST_ID),
            HeaderValue::from_str(request_id).unwrap(),
        );
    }

    /// Increment request count for tracing
    pub fn increment_request_count(&self, path: &str) {
        let mut counts = self
            .request_counts
            .lock()
            .unwrap_or_else(|e| e.into_inner());
        
        *counts.entry(path.to_string()).or_insert(0) += 1;
    }

    /// Get request count statistics
    pub fn get_request_stats(&self) -> HashMap<String, u64> {
        self.request_counts
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }
}

impl Default for LoadBalancer {
    fn default() -> Self {
        Self::new()
    }
}

/// Load balancer middleware
pub async fn load_balancer_middleware(
    req: Request,
    next: Next,
) -> Result<Response, axum::BoxError> {
    let lb = LoadBalancer::new();
    
    let mut response = next.run(req).await;
    
    // Add sticky cookie
    lb.add_sticky_cookie(&mut response);
    
    // Add request ID to response
    lb.add_request_id(&mut response, &lb.generate_request_id());
    
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lb_cookie_generation() {
        let cookie = LbCookie::new("instance-123".to_string());
        assert!(!cookie.session_id.is_empty());
        assert_eq!(cookie.instance_id, "instance-123");
    }

    #[test]
    fn test_lb_cookie_serialization() {
        let cookie = LbCookie::new("instance-123".to_string());
        let serialized = cookie.to_json_string();
        let deserialized = LbCookie::from_json_str(&serialized);
        assert!(deserialized.is_some());
        assert_eq!(deserialized.unwrap().instance_id, "instance-123");
    }

    #[test]
    fn test_load_balancer_sticky_sessions() {
        let lb = LoadBalancer::with_config(true, "instance-1".to_string(), 86400, true);
        
        // Test generation
        let request_id = lb.generate_request_id();
        assert!(!request_id.is_empty());
    }
}
