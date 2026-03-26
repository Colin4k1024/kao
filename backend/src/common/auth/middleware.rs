use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
};
use std::env;

use crate::common::error::AppError;

/// Token validator that validates JWT without database lookup
pub async fn validate_jwt_middleware(
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let secret = env::var("JWT_SECRET").unwrap_or_else(|_| "change-me-in-production".to_string());
    
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|h| h.to_str().ok());
    
    if let Some(auth_str) = auth_header {
        if auth_str.starts_with("Bearer ") {
            let token = auth_str.trim_start_matches("Bearer ");
            
            // Validate token without database lookup
            let claims = crate::common::auth::jwt::validate_token_without_db(token, &secret)?;
            
            // Clone the request and add claims to extensions
            let mut req = req;
            req.extensions_mut().insert(claims);
            
            return Ok(next.run(req).await);
        }
    }
    
    Err(AppError::Authentication("No valid authorization header".to_string()))
}

/// Extract user claims from request
pub fn get_claims_from_request(req: &Request) -> Option<crate::common::auth::claims::Claims> {
    req.extensions().get::<crate::common::auth::claims::Claims>().cloned()
}

/// Check if a user is authenticated
pub fn is_authenticated(req: &Request) -> bool {
    get_claims_from_request(req).is_some()
}
