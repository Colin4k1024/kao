# Security Architecture

Security principles and implementation for Kao admin management system.

## Overview

Kao implements a defense-in-depth security strategy with multiple layers of protection.

## Security Layers

```
┌──────────────────────────────────────────────────────────────────┐
│                   Defense in Depth                               │
└──────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────┐
│                    Application Layer                             │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │  Input   │  │  Auth    │  │ Authorize│  │  Rate    │        │
│  │  Sanitize│  │   JWT    │  │   RBAC   │  │  Limit   │        │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │
└──────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────────┐
│                      Infrastructure                              │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │  HTTPS   │  │  CORS    │  │  SQLi    │  │  XSS     │        │
│  │  (TLS)   │  │  Policy  │  │ Prevention│  │ Prevent  │        │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │
└──────────────────────────────────────────────────────────────────┘
```

## Authentication

### JWT Implementation

```rust
// backend/src/common/auth.rs
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,          // User ID
    pub username: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub exp: usize,           // Expiration timestamp
    pub iat: usize,           // Issued at timestamp
    pub jti: String,          // Julia token ID
}

impl Claims {
    pub fn new(user_id: &str, username: &str, roles: Vec<String>, permissions: Vec<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            roles,
            permissions,
            exp: now + 3600,  // 1 hour
            iat: now,
            jti: Uuid::new_v4().to_string(),
        }
    }
    
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        now > self.exp
    }
}

pub fn create_token(user_id: &str, username: &str, roles: Vec<String>, permissions: Vec<String>) -> String {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = EncodingKey::from_secret(secret.as_bytes());
    let header = Header::new(Algorithm::HS256);
    
    let claims = Claims::new(user_id, username, roles, permissions);
    
    encode(&header, &claims, &key)
        .expect("Failed to encode token")
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<Claims>(token, &key, &validation)
        .map_err(|_| AppError::Authentication("Invalid token".to_string()))?;
    
    if token_data.claims.is_expired() {
        return Err(AppError::Authentication("Token expired".to_string()));
    }
    
    Ok(token_data.claims)
}
```

### Token Validation Middleware

```rust
// backend/src/middleware/auth.rs
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension,
};
use tracing::info;

use crate::common::auth::{verify_token, Claims};

pub async fn extract_claims(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Extension<Claims>, (StatusCode, String)> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?;
    
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid token format".to_string()))?;
    
    let claims = verify_token(token)?;
    
    info!("Token validated for user: {}", claims.username);
    
    Ok(Extension(claims))
}
```

## Password Security

### Password Hashing

```rust
// backend/src/common/auth.rs
use bcrypt::{check_password_hash, hash, E};

pub fn hash_password(password: &str) -> Result<String, bcrypt::Error> {
    // Cost factor 12 is recommended for production
    hash(password, 12)
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, bcrypt::Error> {
    check_password_hash(hashed, password)
}

// Usage
#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Find user
    let user = find_user_by_username(&state.pool, &req.username).await?;
    
    // Verify password
    let valid = verify_password(&req.password, &user.password_hash)?;
    if !valid {
        return Ok(Json(serde_json::json!({
            "code": 401,
            "message": "Invalid password",
            "data": null
        })));
    }
    
    // Generate token
    let token = create_token(
        &user.id.to_string(),
        &user.username,
        user.roles,
        user.permissions,
    );
    
    Ok(Json(serde_json::json!({
        "code": 200,
        "message": "Login successful",
        "data": {
            "access_token": token,
            "user": {
                "id": user.id,
                "username": user.username,
                "roles": user.roles,
            }
        }
    })))
}
```

### Password Policy

| Rule | Requirement | Description |
|------|-------------|-------------|
| Minimum length | 8 characters | Minimum password length |
| Maximum length | 128 characters | Prevent DoS attacks |
| Must have | Uppercase | At least one uppercase letter |
| Must have | Lowercase | At least one lowercase letter |
| Must have | Number | At least one digit |
| Must have | Special | At least one special character |
| History | Last 5 | Prevent password reuse |
| Expiration | 90 days |强制密码过期 |
| Lockout | 5 failures | Account lockout after 5 failed attempts |

## Authorization (RBAC)

### Permission Check Middleware

```rust
// backend/src/middleware/authorize.rs
use axum::{
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
};
use tracing::warn;

use crate::common::auth::Claims;
use crate::AppState;

pub async fn check_permission(
    State(state): State<AppState>,
    mut request: Request,
    claims: Claims,
) -> Result<Request, (StatusCode, String)> {
    // Skip check for super admin
    if claims.permissions.contains(&"*".to_string()) {
        return Ok(request);
    }
    
    // Get route permission requirement
    let required_permissions = get_route_permissions(request.uri().path());
    
    for permission in required_permissions {
        if !claims.permissions.contains(&permission) {
            warn!(
                "Permission denied for user {}: missing {}",
                claims.username, permission
            );
            return Err((StatusCode::FORBIDDEN, "Permission denied".to_string()));
        }
    }
    
    Ok(request)
}
```

### RBAC Implementation

```rust
// backend/src/features/auth/service.rs
use crate::common::error::AppError;

pub async fn check_user_permission(
    user_id: &str,
    required_permission: &str,
    pool: &PgPool,
) -> Result<bool, AppError> {
    // Check if user is super admin (bypass check)
    let is_super_admin = check_super_admin(user_id, pool).await?;
    if is_super_admin {
        return Ok(true);
    }
    
    // Check if user has the permission
    let has_permission = has_permission(user_id, required_permission, pool).await?;
    
    Ok(has_permission)
}

pub async fn check_data_scope(
    user_id: &str,
    requested_scope: DataScope,
    pool: &PgPool,
) -> Result<bool, AppError> {
    let user_scope = get_user_data_scope(user_id, pool).await?;
    
    match (user_scope, requested_scope) {
        (DataScope::All, _) => Ok(true),
        (DataScope::Custom, DataScope::Custom) => Ok(true),
        (DataScope::Dept, DataScope::Dept) => Ok(true),
        (DataScope::Dept, DataScope::DeptAndChild) => Ok(true),
        (DataScope::DeptAndChild, DataScope::Dept) => Ok(true),
        (DataScope::DeptAndChild, DataScope::DeptAndChild) => Ok(true),
        (DataScope::SelfOnly, DataScope::SelfOnly) => Ok(true),
        _ => Ok(false),
    }
}
```

## Input Validation

### Request Validation Middleware

```rust
// backend/src/middleware/validate.rs
use axum::{
    extract::State,
    http::Request,
    response::IntoResponse,
};
use serde::de::DeserializeOwned;
use tracing::warn;

pub async fn validate_request<T>(
    State(_state): State<AppState>,
    mut request: Request,
) -> Result<Request, (u16, String)>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let body = hyper::body::to_bytes(request.body_mut())
        .await
        .map_err(|e| (400, format!("Invalid body: {}", e)))?;
    
    let data: T = serde_json::from_slice(&body)
        .map_err(|e| (400, format!("Invalid JSON: {}", e)))?;
    
    // Validate the data
    validate_data(&data).map_err(|e| (422, format!("Validation error: {:?}", e)))?;
    
    Ok(request)
}

fn validate_data<T>(data: &T) -> Result<(), String>
where
    T: serde::Serialize,
{
    // Use Validator crate for Rust validation
    use validator::Validate;
    
    data.validate()
        .map_err(|e| format!("Validation failed: {:?}", e))?;
    
    Ok(())
}
```

### Input Sanitization

```rust
// backend/src/common/sanitize.rs
use html_escape::encode_text;
use serde::{Deserialize, Serialize};

/// Sanitize text input to prevent XSS
pub fn sanitize_text(input: &str) -> String {
    encode_text(input).to_string()
}

/// Sanitize HTML input (allow safe tags only)
pub fn sanitize_html(input: &str) -> String {
    let dom = html_parser::parse(input).unwrap_or_default();
    let safe-dom = dom
        .into_iter()
        .filter(|node| {
            matches!(
                node,
                html_parser::Node::Text(_) | html_parser::Node::Element(_) if node.is_safe()
            )
        })
        .collect::<Vec<_>>();
    safe_dom.to_string()
}

#[derive(Deserialize)]
pub struct InputSanitizationMiddleware;

impl InputSanitizationMiddleware {
    pub fn sanitize_user_input(data: &mut serde_json::Value) {
        if let serde_json::Value::Object(map) = data {
            for (_key, value) in map.iter_mut() {
                if let serde_json::Value::String(s) = value {
                    *value = serde_json::Value::String(sanitize_text(s));
                }
            }
        }
    }
}
```

## SQL Injection Prevention

### Parameterized Queries

```rust
// ✅ CORRECT: Use parameterized queries
let users = sqlx::query!(
    "SELECT * FROM sys_user WHERE username = $1",
    username
)
.fetch_all(pool)
.await?;

// ❌ WRONG: Never use string concatenation
let query = format!("SELECT * FROM sys_user WHERE username = '{}'", username);
let users = sqlx::query(&query).fetch_all(pool).await?;
```

### Query Builder Pattern

```rust
// backend/src/common/query_builder.rs
use sqlx::PgPool;

pub struct QueryBuilder {
    query: String,
    params: Vec<String>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        QueryBuilder {
            query: String::new(),
            params: Vec::new(),
        }
    }
    
    pub fn select(mut self, table: &str) -> Self {
        self.query = format!("SELECT * FROM {}", table);
        self
    }
    
    pub fn where_eq(mut self, column: &str, value: &str) -> Self {
        self.query = format!("{} WHERE {} = $1", self.query, column);
        self.params.push(value.to_string());
        self
    }
    
    pub fn build(self) -> (String, Vec<String>) {
        (self.query, self.params)
    }
}

// Usage
let (query, params) = QueryBuilder::new()
    .select("sys_user")
    .where_eq("username", "admin")
    .build();

let user = sqlx::query(&query)
    .bind(&params[0])
    .fetch_one(pool)
    .await?;
```

## Cross-Site Request Forgery (CSRF)

### CSRF Token Implementation

```rust
// backend/src/middleware/csrf.rs
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn generate_csrf_token(State(_state): State<AppState>) -> impl IntoResponse {
    let token = Uuid::new_v4().to_string();
    
    // Store token in session/cookie
    // ( simplified for brevity )
    
    vec![("X-CSRF-Token", token.clone())]
}

pub async fn validate_csrf_token(
    State(_state): State<AppState>,
    mut headers: HeaderMap,
) -> Result<HeaderMap, (StatusCode, String)> {
    let request_token = headers
        .get("X-CSRF-Token")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::FORBIDDEN, "Missing CSRF token".to_string()))?;
    
    // Validate token ( simplified for brevity )
    if !is_valid_token(request_token) {
        return Err((StatusCode::FORBIDDEN, "Invalid CSRF token".to_string()));
    }
    
    Ok(headers)
}
```

## Cross-Origin Resource Sharing (CORS)

### CORS Configuration

```rust
// backend/src/middleware/cors.rs
use axum::Router;
use tower_http::cors::{CorsLayer, Any};

pub fn create_cors_layer() -> CorsLayer {
    // In production, use specific origins
    let allowed_origins = std::env::var("ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    
    let origins = allowed_origins
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    
    CorsLayer::new()
        .allow_origin(origins.iter().map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::X_CSRF_TOKEN,
        ])
        .expose_headers([
            http::header::CONTENT_TYPE,
            http::header::RATE_LIMIT_LIMIT,
            http::header::RATE_LIMIT_REMAINING,
            http::header::RATE_LIMIT_RESET,
        ])
}
```

## Rate Limiting

### Rate Limit Middleware

```rust
// backend/src/middleware/rate_limit.rs
use axum::{
    extract::State,
    http::Request,
    response::IntoResponse,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use time::{Duration, Instant};

pub struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    max_requests: usize,
    window_seconds: u64,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window_seconds: u64) -> Self {
        RateLimiter {
            requests: HashMap::new(),
            max_requests,
            window_seconds,
        }
    }
    
    pub async fn is_allowed(&mut self, ip: &str) -> bool {
        let now = Instant::now();
        let window_start = now - Duration::seconds(self.window_seconds as i64);
        
        let entries = self.requests.entry(ip.to_string()).or_default();
        
        // Remove old entries
        entries.retain(|&t| t > window_start);
        
        if entries.len() >= self.max_requests {
            return false;
        }
        
        entries.push(now);
        true
    }
}

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    mut request: Request,
) -> Result<Request, (u16, String)> {
    let client_ip = get_client_ip(&request);
    
    let mut limiter = state.rate_limiter.lock().await;
    
    if !limiter.is_allowed(&client_ip).await {
        return Err((
            429,
            "Rate limit exceeded. Please try again later.".to_string(),
        ));
    }
    
    Ok(request)
}
```

## Data Protection

### Sensitive Data Masking

```rust
// backend/src/common/mask.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Masked<T> {
    value: T,
    masked: bool,
}

impl<T> Masked<T> {
    pub fn new(value: T, masked: bool) -> Self {
        Masked { value, masked }
    }
    
    pub fn mask(value: T) -> Self {
        Masked {
            value,
            masked: true,
        }
    }
    
    pub fn is_masked(&self) -> bool {
        self.masked
    }
    
    pub fn get(&self) -> &T {
        &self.value
    }
}

// Usage
#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub password: Masked<String>,  // Always masked
    pub phone: Option<String>,
    pub email: Option<String>,
}
```

### Data Encryption at Rest

```rust
// backend/src/common/encryption.rs
use aes_gcm::{Aes256Gcm, KeyInit};
use base64::{engine::general_purpose, Engine};
use parking_lot::Mutex;
use std::sync::LazyLock;

static ENCRYPTION_KEY: LazyLock<Vec<u8>> = LazyLock::new(|| {
    std::env::var("ENCRYPTION_KEY")
        .expect("ENCRYPTION_KEY must be set")
        .as_bytes()
        .to_vec()
});

pub fn encrypt(data: &str) -> Result<String, String> {
    let key = Aes256Gcm::new_from_slice(&ENCRYPTION_KEY)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;
    
    let nonce = Aes256Gcm::generate_nonce(&mut rand::thread_rng());
    
    let ciphertext = key
        .encrypt(&nonce, data.as_bytes())
        .map_err(|e| format!("Failed to encrypt: {}", e))?;
    
    Ok(format!(
        "{}:{}",
        general_purpose::STANDARD.encode(nonce),
        general_purpose::STANDARD.encode(&ciphertext)
    ))
}

pub fn decrypt(encrypted: &str) -> Result<String, String> {
    let parts: Vec<&str> = encrypted.split(':').collect();
    if parts.len() != 2 {
        return Err("Invalid encrypted format".to_string());
    }
    
    let nonce = general_purpose::STANDARD
        .decode(parts[0])
        .map_err(|e| format!("Failed to decode nonce: {}", e))?;
    
    let ciphertext = general_purpose::STANDARD
        .decode(parts[1])
        .map_err(|e| format!("Failed to decode ciphertext: {}", e))?;
    
    let key = Aes256Gcm::new_from_slice(&ENCRYPTION_KEY)
        .map_err(|e| format!("Failed to create cipher: {}", e))?;
    
    let plaintext = key
        .decrypt(nonce.as_slice().into(), ciphertext.as_slice())
        .map_err(|e| format!("Failed to decrypt: {}", e))?;
    
    String::from_utf8(plaintext).map_err(|e| format!("Failed to decode UTF-8: {}", e))
}
```

## Logging Security

### Structured Logging

```rust
// backend/src/common/logger.rs
use tracing::{info, warn, error};

pub struct AuditLogger;

impl AuditLogger {
    pub fn log_login(user_id: &str, success: bool, ip: &str) {
        if success {
            info!(
                user_id = %user_id,
                ip = %ip,
                action = "login",
                status = "success",
                "User login succeeded"
            );
        } else {
            warn!(
                user_id = %user_id,
                ip = %ip,
                action = "login",
                status = "failed",
                "User login failed"
            );
        }
    }
    
    pub fn log_permission_denied(user_id: &str, permission: &str, ip: &str) {
        error!(
            user_id = %user_id,
            permission = %permission,
            ip = %ip,
            action = "permission_check",
            status = "denied",
            "Permission check failed"
        );
    }
}
```

### Security Log Events

| Event | Severity | Logged Fields |
|-------|----------|---------------|
| Login Success | INFO | user_id, ip, device, success |
| Login Failure | WARN | user_id, ip, reason, attempts |
| Permission Denied | ERROR | user_id, permission, ip |
| Password Change | INFO | user_id, ip |
| Data Export | INFO | user_id, ip, record_count |
| Account Lock | ERROR | user_id, ip, reason |

## Security Headers

```rust
// backend/src/middleware/security_headers.rs
use axum::{
    http::{HeaderMap, HeaderName, StatusCode},
    response::IntoResponse,
};

pub async fn add_security_headers(mut headers: HeaderMap) -> HeaderMap {
    // Prevent MIME type sniffing
    headers.insert(
        HeaderName::from_static("x-content-type-options"),
        "nosniff".parse().unwrap(),
    );
    
    // Enable XSS filter
    headers.insert(
        HeaderName::from_static("x-xss-protection"),
        "1; mode=block".parse().unwrap(),
    );
    
    // Prevent clickjacking
    headers.insert(
        HeaderName::from_static("x-frame-options"),
        "DENY".parse().unwrap(),
    );
    
    // Enforce HTTPS
    headers.insert(
        HeaderName::from_static("strict-transport-security"),
        "max-age=31536000; includeSubDomains".parse().unwrap(),
    );
    
    // Content Security Policy
    headers.insert(
        HeaderName::from_static("content-security-policy"),
        "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; style-src 'self' 'unsafe-inline'".parse().unwrap(),
    );
    
    // Referrer Policy
    headers.insert(
        HeaderName::from_static("referrer-policy"),
        "strict-origin-when-cross-origin".parse().unwrap(),
    );
    
    headers
}

pub async fn security_middleware(
    State(_state): State<AppState>,
    mut headers: HeaderMap,
) -> HeaderMap {
    headers = add_security_headers(headers).await;
    
    // Add rate limiting headers
    headers.insert(
        HeaderName::from_static("x-ratelimit-limit"),
        "100".parse().unwrap(),
    );
    headers.insert(
        HeaderName::from_static("x-ratelimit-remaining"),
        "99".parse().unwrap(),
    );
    
    headers
}
```

## Security Checklist

### Backend Security

- [ ] All passwords hashed with bcrypt (cost factor 12+)
- [ ] JWT tokens with expiration
- [ ] Refresh token rotation
- [ ] Rate limiting enabled
- [ ] CORS configured with specific origins
- [ ] SQL injection prevention (parameterized queries)
- [ ] XSS prevention (input sanitization)
- [ ] CSRF protection enabled
- [ ] Security headers set
- [ ] Sensitive data encrypted at rest
- [ ] Structured logging enabled
- [ ] Audit logging enabled

### Frontend Security

- [ ] HTTPS enforced
- [ ] Token stored in httpOnly cookies
- [ ] XSS prevention in templates
- [ ] CSRF tokens implemented
- [ ] Input validation on all forms
- [ ] Secure cookie attributes
- [ ] CORS configured on backend

### Deployment Security

- [ ] TLS/HTTPS enabled
- [ ] Secrets managed via environment variables
- [ ] Docker images pinned to specific versions
- [ ] No secrets in Dockerfile
- [ ] Production build without debug symbols
- [ ] Security headers in Nginx

---

## Next Steps

1. Review security headers implementation
2. Enable TLS in production
3. Configure rate limiting
4. Set up audit logging
5. Run security audit tools
