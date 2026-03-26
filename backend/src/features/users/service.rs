use crate::common::error::AppError;
use crate::common::security::{validate_password, check_username_in_password, PasswordValidationError, PasswordPolicy};
use uuid::Uuid;

use super::{
    model::{CreateUserRequest, UpdateUserRequest, UserResponse},
    repo::{create_user, delete_user, get_user_by_id, list_users, update_user, update_user_password},
};
use crate::features::auth::model::{hash_password, verify_password};

pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn list_users(
        &self,
        db: &sqlx::PgPool,
        page: i64,
        page_size: i64,
        dept_id: Option<Uuid>,
    ) -> Result<(Vec<UserResponse>, i64), AppError> {
        let (users, total) = list_users(db, page, page_size, dept_id).await?;
        let responses: Vec<UserResponse> = users
            .into_iter()
            .map(|u| UserResponse {
                id: u.id,
                username: u.username,
                email: u.email,
                display_name: u.display_name,
                dept_id: u.dept_id,
                phone: u.phone,
                avatar_url: u.avatar_url,
                status: u.status,
                is_super_admin: u.is_super_admin,
                last_login_at: u.last_login_at,
                last_login_ip: u.last_login_ip,
                created_at: u.created_at,
                updated_at: u.updated_at,
            })
            .collect();

        Ok((responses, total))
    }

    pub async fn get_user_by_id(
        &self,
        db: &sqlx::PgPool,
        user_id: Uuid,
    ) -> Result<Option<UserResponse>, AppError> {
        let user = get_user_by_id(db, user_id).await?;
        Ok(user.map(|u| UserResponse {
            id: u.id,
            username: u.username,
            email: u.email,
            display_name: u.display_name,
            dept_id: u.dept_id,
            phone: u.phone,
            avatar_url: u.avatar_url,
            status: u.status,
            is_super_admin: u.is_super_admin,
            last_login_at: u.last_login_at,
            last_login_ip: u.last_login_ip,
            created_at: u.created_at,
            updated_at: u.updated_at,
        }))
    }

    pub async fn create_user(
        &self,
        db: &sqlx::PgPool,
        req: CreateUserRequest,
    ) -> Result<UserResponse, AppError> {
        // Validate password
        let policy = PasswordPolicy::default();
        validate_password(&req.password, &policy)
            .map_err(|e| AppError::Validation(format!("Password validation failed: {}", e)))?;
        
        check_username_in_password(&req.password, &req.username)
            .map_err(|_| AppError::Validation("Password must not contain username".to_string()))?;

        let password_hash = hash_password(&req.password)?;
        
        let user = create_user(
            db,
            req.username,
            req.email,
            req.display_name,
            password_hash,
            req.dept_id,
            req.phone,
            req.avatar_url,
        )
        .await?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            dept_id: user.dept_id,
            phone: user.phone,
            avatar_url: user.avatar_url,
            status: user.status,
            is_super_admin: user.is_super_admin,
            last_login_at: user.last_login_at,
            last_login_ip: user.last_login_ip,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    pub async fn update_user(
        &self,
        db: &sqlx::PgPool,
        user_id: Uuid,
        req: UpdateUserRequest,
    ) -> Result<UserResponse, AppError> {
        // If password is being updated, validate it
        if let Some(ref password) = req.password {
            let policy = PasswordPolicy::default();
            validate_password(password, &policy)
                .map_err(|e| AppError::Validation(format!("Password validation failed: {}", e)))?;

            // Get username for username check
            let user = get_user_by_id(db, user_id)
                .await?
                .ok_or_else(|| AppError::Validation("User not found".to_string()))?;

            check_username_in_password(password, &user.username)
                .map_err(|_| AppError::Validation("Password must not contain username".to_string()))?;
        }

        let user = update_user(
            db,
            user_id,
            req.email,
            req.display_name,
            req.dept_id,
            req.phone,
            req.avatar_url,
            req.status,
        )
        .await?;

        Ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            dept_id: user.dept_id,
            phone: user.phone,
            avatar_url: user.avatar_url,
            status: user.status,
            is_super_admin: user.is_super_admin,
            last_login_at: user.last_login_at,
            last_login_ip: user.last_login_ip,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    pub async fn update_password(
        &self,
        db: &sqlx::PgPool,
        user_id: Uuid,
        old_password: String,
        new_password: String,
    ) -> Result<(), AppError> {
        let user = get_user_by_id(db, user_id)
            .await?
            .ok_or_else(|| AppError::Validation("User not found".to_string()))?;

        verify_password(&old_password, &user.password_hash)?;

        // Validate new password
        let policy = PasswordPolicy::default();
        validate_password(&new_password, &policy)
            .map_err(|e| AppError::Validation(format!("Password validation failed: {}", e)))?;

        check_username_in_password(&new_password, &user.username)
            .map_err(|_| AppError::Validation("Password must not contain username".to_string()))?;

        let new_hash = hash_password(&new_password)?;
        update_user_password(db, user_id, new_hash).await?;

        Ok(())
    }

    pub async fn delete_user(&self, db: &sqlx::PgPool, user_id: Uuid) -> Result<(), AppError> {
        delete_user(db, user_id).await
    }
}
