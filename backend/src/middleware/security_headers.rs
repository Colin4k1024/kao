use axum::{
    body::Body,
    extract::Request,
    http::{HeaderName, HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};

/// Security headers to be applied to all responses
const SECURITY_HEADERS: &[(&str, &str)] = &[
    ("x-frame-options", "DENY"),
    ("x-content-type-options", "nosniff"),
    ("x-xss-protection", "1; mode=block"),
    ("referrer-policy", "strict-origin-when-cross-origin"),
    ("permissions-policy", "geolocation=(), microphone=(), camera=()"),
    ("strict-transport-security", "max-age=31536000; includeSubDomains"),
];

/// Middleware that adds security headers to all responses
pub async fn security_headers_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    // Add security headers to response
    for (header_name, header_value) in SECURITY_HEADERS {
        let header_name = HeaderName::from_static(header_name);
        let header_value = HeaderValue::from_static(header_value);

        response.headers_mut().insert(header_name, header_value);
    }

    response
}

/// Content Security Policy header name
pub static CSP_HEADER: HeaderName = HeaderName::from_static("content-security-policy");