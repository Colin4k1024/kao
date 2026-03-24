use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

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