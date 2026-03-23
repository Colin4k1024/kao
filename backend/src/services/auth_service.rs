use crate::error::{AppError, AppResult};
use crate::models::auth::Claims;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const JWT_SECRET: &[u8] = b"your-super-secret-jwt-key";
const TOKEN_EXPIRY_SECS: u64 = 7 * 24 * 60 * 60; // 7 days

pub fn generate_token(user_id: &str, email: &str) -> AppResult<String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + TOKEN_EXPIRY_SECS;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|e| AppError::Internal(e.to_string()))
}

pub fn verify_token(token: &str) -> AppResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| AppError::Unauthorized("Invalid token".to_string()))
}
