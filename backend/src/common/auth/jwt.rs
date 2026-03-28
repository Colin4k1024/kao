use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::SystemTime;

use crate::common::{auth::claims::Claims, error::AppError};

pub fn generate_jwt(claims: Claims, secret: &str) -> Result<String, AppError> {
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| AppError::Authentication(format!("Failed to encode JWT: {}", e)))?;

    Ok(token)
}

pub fn validate_jwt(token: &str, secret: &str) -> Result<Claims, AppError> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )
    .map_err(|e| AppError::Authentication(format!("Failed to decode JWT: {}", e)))?;

    Ok(token_data.claims)
}

/// Validate JWT token without database lookup
pub fn validate_token_without_db(token: &str, secret: &str) -> Result<Claims, AppError> {
    validate_jwt(token, secret)
}

/// Refresh token using JWT claims only (no database lookup)
pub fn refresh_token(
    refresh_token: &str,
    secret: &str,
    access_token_expires_in: u64,
) -> Result<String, AppError> {
    let claims = validate_jwt(refresh_token, secret)?;

    // Create new claims with current timestamp
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let new_claims = Claims {
        sub: claims.sub,
        username: claims.username,
        exp: now + access_token_expires_in as usize,
        iat: now,
        permissions: claims.permissions,
        dept_id: claims.dept_id,
        roles: claims.roles,
        token_version: claims.token_version,
    };

    generate_jwt(new_claims, secret)
}

/// Check if token is expired
pub fn is_token_expired(token: &str, secret: &str) -> Result<bool, AppError> {
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    );

    match token_data {
        Ok(data) => {
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs() as usize;
            Ok(data.claims.exp < now)
        }
        Err(_) => Ok(true), // Invalid token is considered expired
    }
}
