use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::common::{AppError, AppResult, DbPool};

#[derive(Debug, Clone, FromRow)]
pub struct UserSessionRow {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub display_name: String,
    pub password_hash: String,
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
pub struct RoleRow {
    pub id: String,
    pub code: String,
    pub name: String,
    pub data_scope: String,
}

#[derive(Clone)]
pub struct AuthRepo {
    pool: DbPool,
}

impl AuthRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn find_user_by_username(
        &self,
        username: &str,
    ) -> AppResult<Option<UserSessionRow>> {
        let user = sqlx::query_as::<_, UserSessionRow>(
            r#"
            SELECT
                u.id::text AS id,
                u.username,
                u.email,
                u.display_name,
                u.password_hash,
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
            WHERE u.username = $1
              AND u.deleted_at IS NULL
            LIMIT 1
            "#,
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_id(&self, user_id: &str) -> AppResult<UserSessionRow> {
        let user = sqlx::query_as::<_, UserSessionRow>(
            r#"
            SELECT
                u.id::text AS id,
                u.username,
                u.email,
                u.display_name,
                u.password_hash,
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

        Ok(user)
    }

    pub async fn list_roles_by_user_id(&self, user_id: &str) -> AppResult<Vec<RoleRow>> {
        let rows = sqlx::query_as::<_, RoleRow>(
            r#"
            SELECT DISTINCT
                r.id::text AS id,
                r.code,
                r.name,
                r.data_scope
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

    pub async fn list_permissions_by_user_id(&self, user_id: &str) -> AppResult<Vec<String>> {
        let permissions = sqlx::query_scalar::<_, String>(
            r#"
            SELECT DISTINCT m.permission
            FROM sys_menus m
            INNER JOIN sys_role_menus rm ON rm.menu_id = m.id
            INNER JOIN sys_user_roles ur ON ur.role_id = rm.role_id
            WHERE ur.user_id = $1::uuid
              AND m.permission IS NOT NULL
            ORDER BY m.permission
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(permissions)
    }
}
