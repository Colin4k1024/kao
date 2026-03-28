use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::common::error::AppError;

/// Login request with validation rules.
///
/// Validation rules:
/// - username: 3-30 characters, alphanumeric and underscore only
/// - password: minimum 8 characters, at least one uppercase, one lowercase, one digit
#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 30, message = "Username must be 3-30 characters"))]
    #[validate(custom(function = "validate_username_format", message = "Username must be alphanumeric only"))]
    pub username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom(function = "validate_password_complexity", message = "Password must contain uppercase, lowercase, and digit"))]
    pub password: String,
}

/// Validates username format: alphanumeric and underscore only
fn validate_username_format(username: &str) -> Result<(), validator::ValidationError> {
    if username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Ok(())
    } else {
        Err(validator::ValidationError::new("username_format"))
    }
}

/// Validates password complexity: at least one uppercase, one lowercase, one digit
fn validate_password_complexity(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if has_uppercase && has_lowercase && has_digit {
        Ok(())
    } else {
        Err(validator::ValidationError::new("password_complexity"))
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: usize,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: uuid::Uuid,
    pub username: String,
    pub display_name: String,
    pub email: Option<String>,
    pub dept_id: Option<uuid::Uuid>,
    pub avatar_url: Option<String>,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CurrentSessionResponse {
    pub user: UserProfile,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
    pub menu_tree: serde_json::Value,
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Failed to hash password: {}", e)))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|e| AppError::Internal(format!("Password verification failed: {}", e)))
}
