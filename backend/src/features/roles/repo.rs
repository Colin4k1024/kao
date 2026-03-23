use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::common::{AppError, AppResult, DbPool};

#[derive(Debug, Clone, FromRow)]
pub struct RoleRow {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub data_scope: String,
    pub status: String,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct RoleAssignmentRow {
    pub id: String,
}

#[derive(Clone)]
pub struct RolesRepo {
    pool: DbPool,
}

impl RolesRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn list_roles(&self) -> AppResult<Vec<RoleRow>> {
        let rows = sqlx::query_as::<_, RoleRow>(
            r#"
            SELECT
                id::text AS id,
                code,
                name,
                description,
                data_scope,
                status,
                is_system,
                created_at,
                updated_at
            FROM sys_roles
            ORDER BY created_at DESC, code
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn find_role_by_id(&self, role_id: &str) -> AppResult<RoleRow> {
        let row = sqlx::query_as::<_, RoleRow>(
            r#"
            SELECT
                id::text AS id,
                code,
                name,
                description,
                data_scope,
                status,
                is_system,
                created_at,
                updated_at
            FROM sys_roles
            WHERE id = $1::uuid
            LIMIT 1
            "#,
        )
        .bind(role_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("role not found".to_string()))?;

        Ok(row)
    }

    pub async fn create_role(
        &self,
        id: &str,
        code: &str,
        name: &str,
        description: Option<&str>,
        data_scope: &str,
        status: &str,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO sys_roles (
                id,
                code,
                name,
                description,
                data_scope,
                status,
                is_system,
                created_at,
                updated_at
            ) VALUES (
                $1::uuid,
                $2,
                $3,
                $4,
                $5,
                $6,
                FALSE,
                NOW(),
                NOW()
            )
            "#,
        )
        .bind(id)
        .bind(code)
        .bind(name)
        .bind(description)
        .bind(data_scope)
        .bind(status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_role(
        &self,
        id: &str,
        code: &str,
        name: &str,
        description: Option<&str>,
        data_scope: &str,
        status: &str,
    ) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE sys_roles
            SET code = $2,
                name = $3,
                description = $4,
                data_scope = $5,
                status = $6,
                updated_at = NOW()
            WHERE id = $1::uuid
            "#,
        )
        .bind(id)
        .bind(code)
        .bind(name)
        .bind(description)
        .bind(data_scope)
        .bind(status)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("role not found".to_string()));
        }

        Ok(())
    }

    pub async fn replace_role_menus(&self, role_id: &str, menu_ids: &[String]) -> AppResult<()> {
        sqlx::query("DELETE FROM sys_role_menus WHERE role_id = $1::uuid")
            .bind(role_id)
            .execute(&self.pool)
            .await?;

        for menu_id in menu_ids {
            sqlx::query(
                r#"
                INSERT INTO sys_role_menus (role_id, menu_id, assigned_at)
                VALUES ($1::uuid, $2::uuid, NOW())
                "#,
            )
            .bind(role_id)
            .bind(menu_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn replace_role_departments(
        &self,
        role_id: &str,
        department_ids: &[String],
    ) -> AppResult<()> {
        sqlx::query("DELETE FROM sys_role_departments WHERE role_id = $1::uuid")
            .bind(role_id)
            .execute(&self.pool)
            .await?;

        for department_id in department_ids {
            sqlx::query(
                r#"
                INSERT INTO sys_role_departments (role_id, dept_id, assigned_at)
                VALUES ($1::uuid, $2::uuid, NOW())
                "#,
            )
            .bind(role_id)
            .bind(department_id)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn list_menu_ids_by_role_id(&self, role_id: &str) -> AppResult<Vec<String>> {
        let ids = sqlx::query_scalar::<_, String>(
            r#"
            SELECT menu_id::text
            FROM sys_role_menus
            WHERE role_id = $1::uuid
            ORDER BY menu_id
            "#,
        )
        .bind(role_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(ids)
    }

    pub async fn list_department_ids_by_role_id(&self, role_id: &str) -> AppResult<Vec<String>> {
        let ids = sqlx::query_scalar::<_, String>(
            r#"
            SELECT dept_id::text
            FROM sys_role_departments
            WHERE role_id = $1::uuid
            ORDER BY dept_id
            "#,
        )
        .bind(role_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(ids)
    }
}
