use std::sync::Arc;

use crate::common::{
    auth::{claims::Claims, jwt::generate_jwt},
    error::AppError,
    security::validate_password,
};
use uuid::Uuid;

use super::{
    model::{hash_password, verify_password, LoginRequest, LoginResponse, UserProfile},
    repo::{find_user_by_username, get_user_menu_tree, get_user_permissions, get_user_roles},
};

// Settings is imported from config module
use crate::config::Settings;

pub struct AuthService {
    config: Arc<Settings>,
}

impl AuthService {
    pub fn new(config: Arc<Settings>) -> Self {
        Self { config }
    }

    pub async fn login(&self, db: &sqlx::PgPool, req: LoginRequest) -> Result<LoginResponse, AppError> {
        // Validate password complexity before hashing
        let policy = crate::common::security::PasswordPolicy::default();
        if !req.password.is_empty() {
            validate_password(&req.password, &policy)
                .map_err(|_| AppError::Validation("Password does not meet complexity requirements".to_string()))?;
        }

        // Find user by username
        let user = find_user_by_username(db, &req.username)
            .await?
            .ok_or_else(|| AppError::Authentication("Invalid username or password".to_string()))?;

        // Verify password
        let is_valid = verify_password(&req.password, &user.password_hash)?;
        if !is_valid {
            return Err(AppError::Authentication("Invalid username or password".to_string()));
        }

        // Get user permissions and roles
        let permissions = get_user_permissions(db, user.id).await?;
        let roles = get_user_roles(db, user.id).await?;

        // Create JWT claims
        let claims = Claims::new(
            user.id,
            user.username.clone(),
            permissions,
            user.dept_id,
            roles,
        );

        // Generate JWT token
        let token = generate_jwt(claims, &self.config.jwt.secret)?;

        Ok(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 24 * 60 * 60, // 24 hours
        })
    }

    pub async fn register(
        &self,
        db: &sqlx::PgPool,
        username: String,
        password: String,
        email: Option<String>,
        display_name: String,
    ) -> Result<(), AppError> {
        // Validate password
        let policy = crate::common::security::PasswordPolicy::default();
        validate_password(&password, &policy)
            .map_err(|_| AppError::Validation("Password does not meet complexity requirements".to_string()))?;

        // Check username is not in password
        if password.to_lowercase().contains(&username.to_lowercase()) {
            return Err(AppError::Validation("Password must not contain username".to_string()));
        }

        // Hash password
        let password_hash = hash_password(&password)?;

        // Create user in database
        let _ = sqlx::query(
            r#"
            INSERT INTO sys_users (username, password_hash, email, display_name, status, created_at)
            VALUES ($1, $2, $3, $4, 'ACTIVE', NOW())
            "#,
        )
        .bind(&username)
        .bind(&password_hash)
        .bind(&email)
        .bind(&display_name)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create user: {}", e)))?;

        Ok(())
    }

    pub async fn get_current_user_profile(
        &self,
        db: &sqlx::PgPool,
        user_id: Uuid,
    ) -> Result<UserProfile, AppError> {
        // Get user details
        let user = sqlx::query_as::<_, (Uuid, String, String, Option<String>, Option<Uuid>, Option<String>)>(
            r#"
            SELECT
                id,
                username,
                display_name,
                email,
                dept_id,
                avatar_url
            FROM sys_users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(db)
        .await
        .map_err(|_| AppError::Authentication("User not found".to_string()))?;

        // Get user permissions and roles
        let permissions = get_user_permissions(db, user_id).await?;
        let roles = get_user_roles(db, user_id).await?;

        Ok(UserProfile {
            id: user.0,
            username: user.1,
            display_name: user.2,
            email: user.3,
            dept_id: user.4,
            avatar_url: user.5,
            permissions,
            roles,
        })
    }

    pub async fn change_password(
        &self,
        db: &sqlx::PgPool,
        user_id: Uuid,
        old_password: String,
        new_password: String,
    ) -> Result<(), AppError> {
        // Validate new password
        let policy = crate::common::security::PasswordPolicy::default();
        validate_password(&new_password, &policy)
            .map_err(|_| AppError::Validation("New password does not meet complexity requirements".to_string()))?;

        // Get user
        let user = sqlx::query_as::<_, (Uuid, String, String)>(
            r#"
            SELECT id, username, password_hash
            FROM sys_users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_one(db)
        .await
        .map_err(|_| AppError::Authentication("User not found".to_string()))?;

        // Verify old password
        let is_valid = verify_password(&old_password, &user.2)?;
        if !is_valid {
            return Err(AppError::Authentication("Current password is incorrect".to_string()));
        }

        // Check username is not in new password
        if new_password.to_lowercase().contains(&user.1.to_lowercase()) {
            return Err(AppError::Validation("New password must not contain username".to_string()));
        }

        // Update password
        let new_hash = hash_password(&new_password)?;
        let _ = sqlx::query(
            r#"
            UPDATE sys_users
            SET password_hash = $1,
                last_password_changed_at = NOW(),
                password_expires_at = NOW() + INTERVAL '90 days'
            WHERE id = $2
            "#,
        )
        .bind(&new_hash)
        .bind(user_id)
        .execute(db)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to change password: {}", e)))?;

        Ok(())
    }
}
