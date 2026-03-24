use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;

use super::model::RoleRecord;

pub async fn get_role_by_id(db: &PgPool, role_id: Uuid) -> Result<Option<RoleRecord>, AppError> {
    let role = sqlx::query_as::<_, RoleRecord>(
        r#"
        SELECT 
            id, code, name, description, data_scope, status, 
            is_system, created_at, updated_at
        FROM sys_roles 
        WHERE id = $1
        "#,
    )
    .bind(role_id)
    .fetch_optional(db)
    .await?;

    Ok(role)
}

pub async fn get_role_by_code(db: &PgPool, code: &str) -> Result<Option<RoleRecord>, AppError> {
    let role = sqlx::query_as::<_, RoleRecord>(
        r#"
        SELECT 
            id, code, name, description, data_scope, status, 
            is_system, created_at, updated_at
        FROM sys_roles 
        WHERE code = $1
        "#,
    )
    .bind(code)
    .fetch_optional(db)
    .await?;

    Ok(role)
}

pub async fn list_roles(db: &PgPool) -> Result<Vec<RoleRecord>, AppError> {
    let roles = sqlx::query_as::<_, RoleRecord>(
        r#"
        SELECT 
            id, code, name, description, data_scope, status, 
            is_system, created_at, updated_at
        FROM sys_roles 
        WHERE status = 'ACTIVE'
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(roles)
}

pub async fn create_role(
    db: &PgPool,
    code: String,
    name: String,
    description: Option<String>,
    data_scope: String,
) -> Result<RoleRecord, AppError> {
    let role = sqlx::query_as::<_, RoleRecord>(
        r#"
        INSERT INTO sys_roles (code, name, description, data_scope, status)
        VALUES ($1, $2, $3, $4, 'ACTIVE')
        RETURNING 
            id, code, name, description, data_scope, status, 
            is_system, created_at, updated_at
        "#,
    )
    .bind(code)
    .bind(name)
    .bind(description)
    .bind(data_scope)
    .fetch_one(db)
    .await?;

    Ok(role)
}

pub async fn update_role(
    db: &PgPool,
    role_id: Uuid,
    name: Option<String>,
    description: Option<String>,
    data_scope: Option<String>,
    status: Option<String>,
) -> Result<RoleRecord, AppError> {
    let role = sqlx::query_as::<_, RoleRecord>(
        r#"
        UPDATE sys_roles 
        SET 
            name = COALESCE($2, name),
            description = COALESCE($3, description),
            data_scope = COALESCE($4, data_scope),
            status = COALESCE($5, status),
            updated_at = NOW()
        WHERE id = $1
        RETURNING 
            id, code, name, description, data_scope, status, 
            is_system, created_at, updated_at
        "#,
    )
    .bind(role_id)
    .bind(name)
    .bind(description)
    .bind(data_scope)
    .bind(status)
    .fetch_one(db)
    .await?;

    Ok(role)
}

pub async fn delete_role(db: &PgPool, role_id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sys_roles WHERE id = $1 AND is_system = false")
        .bind(role_id)
        .execute(db)
        .await?;

    Ok(())
}

pub async fn assign_role_menus(db: &PgPool, role_id: Uuid, menu_ids: &[Uuid]) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sys_role_menus WHERE role_id = $1")
        .bind(role_id)
        .execute(db)
        .await?;

    for menu_id in menu_ids {
        sqlx::query("INSERT INTO sys_role_menus (role_id, menu_id) VALUES ($1, $2)")
            .bind(role_id)
            .bind(menu_id)
            .execute(db)
            .await?;
    }

    Ok(())
}

pub async fn get_role_menu_ids(db: &PgPool, role_id: Uuid) -> Result<Vec<Uuid>, AppError> {
    let rows = sqlx::query("SELECT menu_id FROM sys_role_menus WHERE role_id = $1")
        .bind(role_id)
        .fetch_all(db)
        .await?;
    
    let menu_ids: Vec<Uuid> = rows
        .into_iter()
        .map(|row| row.get::<Uuid, _>(0))
        .collect();

    Ok(menu_ids)
}

pub async fn assign_role_departments(db: &PgPool, role_id: Uuid, dept_ids: &[Uuid]) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sys_role_departments WHERE role_id = $1")
        .bind(role_id)
        .execute(db)
        .await?;

    for dept_id in dept_ids {
        sqlx::query("INSERT INTO sys_role_departments (role_id, dept_id) VALUES ($1, $2)")
            .bind(role_id)
            .bind(dept_id)
            .execute(db)
            .await?;
    }

    Ok(())
}

pub async fn get_role_department_ids(db: &PgPool, role_id: Uuid) -> Result<Vec<Uuid>, AppError> {
    let rows = sqlx::query("SELECT dept_id FROM sys_role_departments WHERE role_id = $1")
        .bind(role_id)
        .fetch_all(db)
        .await?;
    
    let dept_ids: Vec<Uuid> = rows
        .into_iter()
        .map(|row| row.get::<Uuid, _>(0))
        .collect();

    Ok(dept_ids)
}