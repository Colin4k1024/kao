use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;

use super::model::MenuRecord;

pub async fn get_menu_by_id(db: &PgPool, menu_id: Uuid) -> Result<Option<MenuRecord>, AppError> {
    let menu = sqlx::query_as::<_, MenuRecord>(
        r#"
        SELECT 
            id,
            parent_id,
            name,
            menu_type,
            route_path,
            component,
            permission,
            icon,
            sort_order,
            visible,
            status,
            created_at,
            updated_at
        FROM sys_menus 
        WHERE id = $1
        "#,
    )
    .bind(menu_id)
    .fetch_optional(db)
    .await?;

    Ok(menu)
}

pub async fn get_menu_tree(db: &PgPool) -> Result<Vec<MenuRecord>, AppError> {
    let menus = sqlx::query_as::<_, MenuRecord>(
        r#"
        SELECT 
            id,
            parent_id,
            name,
            menu_type,
            route_path,
            component,
            permission,
            icon,
            sort_order,
            visible,
            status,
            created_at,
            updated_at
        FROM sys_menus 
        WHERE status = 'ACTIVE'
        ORDER BY parent_id, sort_order
        "#
    )
    .fetch_all(db)
    .await?;

    Ok(menus)
}

pub async fn create_menu(
    db: &PgPool,
    parent_id: Option<Uuid>,
    name: String,
    menu_type: String,
    route_path: Option<String>,
    component: Option<String>,
    permission: Option<String>,
    icon: Option<String>,
    sort_order: i32,
    visible: bool,
) -> Result<MenuRecord, AppError> {
    let menu = sqlx::query_as::<_, MenuRecord>(
        r#"
        INSERT INTO sys_menus (
            parent_id, name, menu_type, route_path, component, 
            permission, icon, sort_order, visible, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'ACTIVE')
        RETURNING 
            id, parent_id, name, menu_type, route_path, component, 
            permission, icon, sort_order, visible, status, 
            created_at, updated_at
        "#,
    )
    .bind(parent_id)
    .bind(name)
    .bind(menu_type)
    .bind(route_path)
    .bind(component)
    .bind(permission)
    .bind(icon)
    .bind(sort_order)
    .bind(visible)
    .fetch_one(db)
    .await?;

    Ok(menu)
}

pub async fn update_menu(
    db: &PgPool,
    menu_id: Uuid,
    parent_id: Option<Uuid>,
    name: String,
    menu_type: String,
    route_path: Option<String>,
    component: Option<String>,
    permission: Option<String>,
    icon: Option<String>,
    sort_order: i32,
    visible: bool,
) -> Result<MenuRecord, AppError> {
    let menu = sqlx::query_as::<_, MenuRecord>(
        r#"
        UPDATE sys_menus 
        SET 
            parent_id = $2,
            name = $3,
            menu_type = $4,
            route_path = $5,
            component = $6,
            permission = $7,
            icon = $8,
            sort_order = $9,
            visible = $10,
            updated_at = NOW()
        WHERE id = $1
        RETURNING 
            id, parent_id, name, menu_type, route_path, component, 
            permission, icon, sort_order, visible, status, 
            created_at, updated_at
        "#,
    )
    .bind(menu_id)
    .bind(parent_id)
    .bind(name)
    .bind(menu_type)
    .bind(route_path)
    .bind(component)
    .bind(permission)
    .bind(icon)
    .bind(sort_order)
    .bind(visible)
    .fetch_one(db)
    .await?;

    Ok(menu)
}

pub async fn delete_menu(db: &PgPool, menu_id: Uuid) -> Result<(), AppError> {
    let count_row = sqlx::query("SELECT COUNT(*) FROM sys_menus WHERE parent_id = $1")
        .bind(menu_id)
        .fetch_one(db)
        .await?;
    let child_count: i64 = count_row.get(0);

    if child_count > 0 {
        return Err(AppError::Validation("Cannot delete menu with children".to_string()));
    }

    sqlx::query("DELETE FROM sys_menus WHERE id = $1")
        .bind(menu_id)
        .execute(db)
        .await?;

    Ok(())
}