use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::{auth::jwt::validate_jwt, error::AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub username: String,
    pub permissions: Vec<String>,
    pub dept_id: Option<Uuid>,
    pub roles: Vec<String>,
}

impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|header| header.to_str().ok())
            .ok_or_else(|| AppError::Authentication("No authorization header".to_string()))?;

        let token = if auth_header.starts_with("Bearer ") {
            auth_header.trim_start_matches("Bearer ").trim()
        } else {
            return Err(AppError::Authentication(
                "Invalid authorization header format".to_string(),
            ));
        };

        let claims = validate_jwt(token, "change-me-in-development")?;

        let user = AuthUser {
            id: Uuid::parse_str(&claims.sub)
                .map_err(|_| AppError::Authentication("Invalid user ID in token".to_string()))?,
            username: claims.username,
            permissions: claims.permissions,
            dept_id: claims.dept_id.and_then(|id| Uuid::parse_str(&id).ok()),
            roles: claims.roles,
        };

        Ok(user)
    }
}
