use crate::common::{config::AppConfig, error::AppError};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};

use super::claims::Claims;

pub fn encode(claims: &Claims, config: &AppConfig) -> Result<String, AppError> {
    jsonwebtoken::encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|err| AppError::Internal(err.to_string()))
}

pub fn decode(token: &str, config: &AppConfig) -> Result<Claims, AppError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;

    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_bytes()),
        &validation,
    )
    .map(|decoded| decoded.claims)
    .map_err(|err| AppError::Unauthorized(err.to_string()))
}
