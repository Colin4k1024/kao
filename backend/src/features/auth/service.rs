use std::sync::Arc;

use crate::common::{
    auth::{claims::Claims, jwt::generate_jwt},
    config::Config,
    error::AppError,
};
use uuid::Uuid;

use super::{
    model::{hash_password, verify_password, LoginRequest, LoginResponse, UserProfile},
    repo::{find_user_by_username, get_user_menu_tree, get_user_permissions, get_user_roles},
};

pub struct AuthService {
    config: Arc<Config>,
}

impl AuthService {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn login(&self, db: &sqlx::PgPool, req: LoginRequest) -> Result<LoginResponse, AppError> {
        // Find user by username
        let user = find_user_by_username(db, &req.username)
            .await?
            .ok_or_else(|| AppError::Authentication("Invalid username or password".to_string()))?;

        // Verify password
        verify_password(&req.password, &user.password_hash)?;

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
        let token = generate_jwt(claims, &self.config.jwt_secret)?;

        Ok(LoginResponse {
            access_token: token,
            token_type: "Bearer".to_string(),
            expires_in: 24 * 60 * 60, // 24 hours
        })
    }

    pub async fn get_current_user_profile(
        &self,
        db: &sqlx::PgPool,
        user_id: Uuid,
    ) -> Result<UserProfile, AppError> {
        // Get user details
        let user = sqlx::query!(
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
            user_id
        )
        .fetch_one(db)
        .await
        .map_err(|_| AppError::Authentication("User not found".to_string()))?;

        // Get user permissions and roles
        let permissions = get_user_permissions(db, user_id).await?;
        let roles = get_user_roles(db, user_id).await?;

        Ok(UserProfile {
            id: user.id,
            username: user.username,
            display_name: user.display_name,
            email: user.email,
            dept_id: user.dept_id,
            avatar_url: user.avatar_url,
            permissions,
            roles,
        })
    }
}