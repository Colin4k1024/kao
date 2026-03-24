use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub dept_id: Option<Uuid>,
    pub avatar_url: Option<String>,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CurrentSessionResponse {
    pub user: UserProfile,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
    pub menu_tree: Value, // Will be populated with menu tree data
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|e| AppError::Internal(format!("Password verification failed: {}", e)))
}