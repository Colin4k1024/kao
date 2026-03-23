use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::common::{AppError, AppResult, DbPool};

#[derive(Debug, Clone, FromRow)]
pub struct UserRow {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub dept_id: Option<String>,
    pub dept_name: Option<String>,
    pub status: String,
    pub is_super_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UserRoleRow {
    pub role_id: String,
    pub role_code: String,
}

#[derive(Clone)]
pub struct UsersRepo {
    pool: DbPool,
}

impl UsersRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn list_users(&self) -> AppResult<Vec<UserRow>> {
        let rows = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT
                u.id::text AS id,
                u.username,
                u.email,
                u.display_name,
                u.avatar_url,
                u.phone,
                u.dept_id::text AS dept_id,
                d.name AS dept_name,
                u.status,
                u.is_super_admin,
                u.created_at,
                u.updated_at
            FROM sys_users u
            LEFT JOIN sys_departments d ON d.id = u.dept_id
            WHERE u.deleted_at IS NULL
            ORDER BY u.created_at DESC, u.username
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn find_user_by_id(&self, user_id: &str) -> AppResult<UserRow> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
            SELECT
                u.id::text AS id,
                u.username,
                u.email,
                u.display_name,
                u.avatar_url,
                u.phone,
                u.dept_id::text AS dept_id,
                d.name AS dept_name,
                u.status,
                u.is_super_admin,
                u.created_at,
                u.updated_at
            FROM sys_users u
            LEFT JOIN sys_departments d ON d.id = u.dept_id
            WHERE u.id = $1::uuid
              AND u.deleted_at IS NULL
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".to_string()))?;

        Ok(row)
    }

    pub async fn list_roles_by_user_id(&self, user_id: &str) -> AppResult<Vec<UserRoleRow>> {
        let rows = sqlx::query_as::<_, UserRoleRow>(
            r#"
            SELECT
                r.id::text AS role_id,
                r.code AS role_code
            FROM sys_roles r
            INNER JOIN sys_user_roles ur ON ur.role_id = r.id
            WHERE ur.user_id = $1::uuid
            ORDER BY r.code
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn create_user(
        &self,
        id: &str,
        username: &str,
        email: &str,
        display_name: &str,
        password_hash: &str,
        dept_id: Option<&str>,
        avatar_url: Option<&str>,
        phone: Option<&str>,
        status: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO sys_users (
                id,
                username,
                email,
                display_name,
                password_hash,
                avatar_url,
                phone,
                dept_id,
                status,
                is_super_admin,
                created_at,
                updated_at,
                deleted_at
            ) VALUES (
                $1::uuid,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8::uuid,
                $9,
                FALSE,
                NOW(),
                NOW(),
                NULL
            )
            "#,
        )
        .bind(id)
        .bind(username)
        .bind(email)
        .bind(display_name)
        .bind(password_hash)
        .bind(avatar_url)
        .bind(phone)
        .bind(dept_id)
        .bind(status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_user(
        &self,
        id: &str,
        username: &str,
        email: &str,
        display_name: &str,
        dept_id: Option<&str>,
        avatar_url: Option<&str>,
        phone: Option<&str>,
        status: &str,
    ) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE sys_users
            SET username = $2,
                email = $3,
                display_name = $4,
                dept_id = $5::uuid,
                avatar_url = $6,
                phone = $7,
                status = $8,
                updated_at = NOW()
            WHERE id = $1::uuid
              AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .bind(username)
        .bind(email)
        .bind(display_name)
        .bind(dept_id)
        .bind(avatar_url)
        .bind(phone)
        .bind(status)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("user not found".to_string()));
        }

        Ok(())
    }

    pub async fn update_user_password(
        &self,
        id: &str,
        password_hash: &str,
    ) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE sys_users
            SET password_hash = $2,
                password_changed_at = NOW(),
                updated_at = NOW()
            WHERE id = $1::uuid
              AND deleted_at IS NULL
            "#,
        )
        .bind(id)
        .bind(password_hash)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("user not found".to_string()));
        }

        Ok(())
    }

    pub async fn replace_user_roles(&self, user_id: &str, role_ids: &[String]) -> AppResult<()> {
        sqlx::query("DELETE FROM sys_user_roles WHERE user_id = $1::uuid")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        for role_id in role_ids {
            sqlx::query(
                r#"
                INSERT INTO sys_user_roles (user_id, role_id, assigned_at)
                VALUES ($1::uuid, $2::uuid, NOW())
                "#,
            )
            .bind(user_id)
            .bind(role_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }
}
