use chrono::{DateTime, Utc};
use sqlx::FromRow;

use crate::common::{AppError, AppResult, DbPool};

#[derive(Debug, Clone, FromRow)]
pub struct DepartmentRow {
    pub id: String,
    pub code: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub ancestors: String,
    pub path: String,
    pub sort_order: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct DepartmentsRepo {
    pool: DbPool,
}

impl DepartmentsRepo {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    pub async fn list_departments(&self) -> AppResult<Vec<DepartmentRow>> {
        let rows = sqlx::query_as::<_, DepartmentRow>(
            r#"
            SELECT
                id::text AS id,
                code,
                name,
                parent_id::text AS parent_id,
                ancestors,
                path,
                sort_order,
                leader,
                phone,
                email,
                status,
                created_at,
                updated_at
            FROM sys_departments
            ORDER BY sort_order, name, code
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn find_department_by_id(&self, department_id: &str) -> AppResult<DepartmentRow> {
        let row = sqlx::query_as::<_, DepartmentRow>(
            r#"
            SELECT
                id::text AS id,
                code,
                name,
                parent_id::text AS parent_id,
                ancestors,
                path,
                sort_order,
                leader,
                phone,
                email,
                status,
                created_at,
                updated_at
            FROM sys_departments
            WHERE id = $1::uuid
            LIMIT 1
            "#,
        )
        .bind(department_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("department not found".to_string()))?;

        Ok(row)
    }

    pub async fn create_department(
        &self,
        row: DepartmentInsert<'_>,
    ) -> AppResult<()> {
        sqlx::query(
            r#"
            INSERT INTO sys_departments (
                id,
                code,
                name,
                parent_id,
                ancestors,
                path,
                sort_order,
                leader,
                phone,
                email,
                status,
                created_at,
                updated_at
            ) VALUES (
                $1::uuid,
                $2,
                $3,
                $4::uuid,
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11,
                NOW(),
                NOW()
            )
            "#,
        )
        .bind(row.id)
        .bind(row.code)
        .bind(row.name)
        .bind(row.parent_id)
        .bind(row.ancestors)
        .bind(row.path)
        .bind(row.sort_order)
        .bind(row.leader)
        .bind(row.phone)
        .bind(row.email)
        .bind(row.status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_department(
        &self,
        row: DepartmentInsert<'_>,
    ) -> AppResult<()> {
        let result = sqlx::query(
            r#"
            UPDATE sys_departments
            SET code = $2,
                name = $3,
                parent_id = $4::uuid,
                ancestors = $5,
                path = $6,
                sort_order = $7,
                leader = $8,
                phone = $9,
                email = $10,
                status = $11,
                updated_at = NOW()
            WHERE id = $1::uuid
            "#,
        )
        .bind(row.id)
        .bind(row.code)
        .bind(row.name)
        .bind(row.parent_id)
        .bind(row.ancestors)
        .bind(row.path)
        .bind(row.sort_order)
        .bind(row.leader)
        .bind(row.phone)
        .bind(row.email)
        .bind(row.status)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("department not found".to_string()));
        }

        Ok(())
    }
}

pub struct DepartmentInsert<'a> {
    pub id: &'a str,
    pub code: &'a str,
    pub name: &'a str,
    pub parent_id: Option<&'a str>,
    pub ancestors: &'a str,
    pub path: &'a str,
    pub sort_order: i32,
    pub leader: Option<&'a str>,
    pub phone: Option<&'a str>,
    pub email: Option<&'a str>,
    pub status: &'a str,
}
