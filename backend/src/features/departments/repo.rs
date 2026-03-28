use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;

use super::model::DepartmentRecord;

pub async fn get_department_by_id(db: &PgPool, dept_id: Uuid) -> Result<Option<DepartmentRecord>, AppError> {
    let dept = sqlx::query_as::<_, DepartmentRecord>(
        r#"
        SELECT 
            id, parent_id, code, name, ancestors, path, sort_order,
            leader, phone, email, status, created_at, updated_at
        FROM sys_departments 
        WHERE id = $1
        "#,
    )
    .bind(dept_id)
    .fetch_optional(db)
    .await?;

    Ok(dept)
}

pub async fn get_department_by_code(db: &PgPool, code: &str) -> Result<Option<DepartmentRecord>, AppError> {
    let dept = sqlx::query_as::<_, DepartmentRecord>(
        r#"
        SELECT 
            id, parent_id, code, name, ancestors, path, sort_order,
            leader, phone, email, status, created_at, updated_at
        FROM sys_departments 
        WHERE code = $1
        "#,
    )
    .bind(code)
    .fetch_optional(db)
    .await?;

    Ok(dept)
}

pub async fn list_departments(db: &PgPool) -> Result<Vec<DepartmentRecord>, AppError> {
    let depts = sqlx::query_as::<_, DepartmentRecord>(
        r#"
        SELECT 
            id, parent_id, code, name, ancestors, path, sort_order,
            leader, phone, email, status, created_at, updated_at
        FROM sys_departments 
        WHERE status = 'ACTIVE'
        ORDER BY sort_order, created_at
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(depts)
}

#[allow(clippy::too_many_arguments)]
pub async fn create_department(
    db: &PgPool,
    parent_id: Option<Uuid>,
    code: String,
    name: String,
    sort_order: i32,
    leader: Option<String>,
    phone: Option<String>,
    email: Option<String>,
) -> Result<DepartmentRecord, AppError> {
    let (ancestors, path) = if let Some(pid) = parent_id {
        let parent = get_department_by_id(db, pid)
            .await?
            .ok_or_else(|| AppError::Validation { field: "parent_id".to_string(), message: "Parent department not found".to_string() })?;
        
        let ancestors = if parent.ancestors.is_empty() {
            pid.to_string()
        } else {
            format!("{},{}", parent.ancestors, pid)
        };
        
        let path = format!("{}/{}", parent.path, code);
        
        (ancestors, path)
    } else {
        (String::new(), code.clone())
    };

    let dept = sqlx::query_as::<_, DepartmentRecord>(
        r#"
        INSERT INTO sys_departments (
            parent_id, code, name, ancestors, path, sort_order,
            leader, phone, email, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'ACTIVE')
        RETURNING 
            id, parent_id, code, name, ancestors, path, sort_order,
            leader, phone, email, status, created_at, updated_at
        "#,
    )
    .bind(parent_id)
    .bind(code)
    .bind(name)
    .bind(ancestors)
    .bind(path)
    .bind(sort_order)
    .bind(leader)
    .bind(phone)
    .bind(email)
    .fetch_one(db)
    .await?;

    Ok(dept)
}

#[allow(clippy::too_many_arguments)]
pub async fn update_department(
    db: &PgPool,
    dept_id: Uuid,
    parent_id: Option<Uuid>,
    code: Option<String>,
    name: Option<String>,
    sort_order: Option<i32>,
    leader: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    status: Option<String>,
) -> Result<DepartmentRecord, AppError> {
    let dept = sqlx::query_as::<_, DepartmentRecord>(
        r#"
        UPDATE sys_departments 
        SET 
            parent_id = $2,
            code = COALESCE($3, code),
            name = COALESCE($4, name),
            sort_order = COALESCE($5, sort_order),
            leader = COALESCE($6, leader),
            phone = COALESCE($7, phone),
            email = COALESCE($8, email),
            status = COALESCE($9, status),
            updated_at = NOW()
        WHERE id = $1
        RETURNING 
            id, parent_id, code, name, ancestors, path, sort_order,
            leader, phone, email, status, created_at, updated_at
        "#,
    )
    .bind(dept_id)
    .bind(parent_id)
    .bind(code)
    .bind(name)
    .bind(sort_order)
    .bind(leader)
    .bind(phone)
    .bind(email)
    .bind(status)
    .fetch_one(db)
    .await?;

    Ok(dept)
}

pub async fn delete_department(db: &PgPool, dept_id: Uuid) -> Result<(), AppError> {
    let child_count_row = sqlx::query("SELECT COUNT(*) FROM sys_departments WHERE parent_id = $1")
        .bind(dept_id)
        .fetch_one(db)
        .await?;
    let child_count: i64 = child_count_row.get(0);

    if child_count > 0 {
        return Err(AppError::Validation { field: "dept_id".to_string(), message: "Cannot delete department with children".to_string() });
    }

    let user_count_row = sqlx::query("SELECT COUNT(*) FROM sys_users WHERE dept_id = $1")
        .bind(dept_id)
        .fetch_one(db)
        .await?;
    let user_count: i64 = user_count_row.get(0);

    if user_count > 0 {
        return Err(AppError::Validation { field: "dept_id".to_string(), message: "Cannot delete department with users".to_string() });
    }

    sqlx::query("DELETE FROM sys_departments WHERE id = $1")
        .bind(dept_id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn get_child_departments(db: &PgPool, dept_id: Uuid) -> Result<Vec<Uuid>, AppError> {
    let rows = sqlx::query("SELECT id FROM sys_departments WHERE ancestors LIKE '%' || $1::text || '%'")
        .bind(dept_id.to_string())
        .fetch_all(db)
        .await?;
    
    let children: Vec<Uuid> = rows
        .into_iter()
        .map(|row| row.get::<Uuid, _>(0))
        .collect();

    Ok(children)
}