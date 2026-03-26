use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    ExpiredToken,
    #[error("Invalid credentials")]
    InvalidCredentials,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub username: String,
    pub exp: i64,
    pub iat: i64,
    #[serde(default)]
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
    pub user_id: String,
    pub username: String,
}

pub fn create_token(user_id: &str, username: &str, secret: &str, expires_in: &str) -> Result<String, JwtError> {
    let now = Utc::now();
    let expiration = match expires_in {
        s if s.ends_with('d') => {
            let days: i64 = s.trim_end_matches('d').parse().unwrap_or(7);
            now + Duration::days(days)
        }
        s if s.ends_with('h') => {
            let hours: i64 = s.trim_end_matches('h').parse().unwrap_or(24);
            now + Duration::hours(hours)
        }
        s if s.ends_with('m') => {
            let minutes: i64 = s.trim_end_matches('m').parse().unwrap_or(60);
            now + Duration::minutes(minutes)
        }
        _ => now + Duration::days(7),
    };

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
        token_type: "access".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| JwtError::InvalidCredentials)
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| {
        if e.to_string().contains("exp") {
            JwtError::ExpiredToken
        } else {
            JwtError::InvalidToken
        }
    })
}

pub fn create_access_token(
    user_id: &str,
    username: &str,
    secret: &str,
    expires_in_seconds: i64,
) -> Result<String, JwtError> {
    let now = Utc::now();
    let expiration = now + Duration::seconds(expires_in_seconds);

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
        token_type: "access".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| JwtError::InvalidCredentials)
}

pub fn create_refresh_token(
    user_id: &str,
    username: &str,
    secret: &str,
    expires_in_seconds: i64,
) -> Result<String, JwtError> {
    let now = Utc::now();
    let expiration = now + Duration::seconds(expires_in_seconds);

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
        token_type: "refresh".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|_| JwtError::InvalidCredentials)
}

pub fn verify_access_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
    let claims = verify_token(token, secret)?;
    if claims.token_type != "access" {
        return Err(JwtError::InvalidCredentials);
    }
    Ok(claims)
}

pub fn verify_refresh_token(token: &str, secret: &str) -> Result<Claims, JwtError> {
    let claims = verify_token(token, secret)?;
    if claims.token_type != "refresh" {
        return Err(JwtError::InvalidCredentials);
    }
    Ok(claims)
}
