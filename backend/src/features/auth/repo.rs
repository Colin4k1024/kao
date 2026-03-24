use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::common::error::AppError;

#[derive(sqlx::FromRow, Debug)]
pub struct UserRecord {
    pub id: Uuid,
    pub username: String,
    pub display_name: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub dept_id: Option<Uuid>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub is_super_admin: bool,
}

#[derive(sqlx::FromRow, Debug)]
pub struct MenuRecord {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub menu_type: String,
    pub route_path: Option<String>,
    pub component: Option<String>,
    pub permission: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub visible: bool,
}

pub async fn find_user_by_username(db: &PgPool, username: &str) -> Result<Option<UserRecord>, AppError> {
    let user = sqlx::query_as::<_, UserRecord>(
        r#"
        SELECT 
            id,
            username,
            display_name,
            password_hash,
            email,
            dept_id,
            avatar_url,
            status,
            is_super_admin
        FROM sys_users 
        WHERE username = $1 AND status = 'ACTIVE' AND (deleted_at IS NULL OR deleted_at > NOW())
        "#,
    )
    .bind(username)
    .fetch_optional(db)
    .await?;

    Ok(user)
}

pub async fn get_user_permissions(db: &PgPool, user_id: Uuid) -> Result<Vec<String>, AppError> {
    let is_super_admin_row = sqlx::query("SELECT is_super_admin FROM sys_users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await?;
    
    let is_super_admin = is_super_admin_row
        .map(|row| row.get::<bool, _>("is_super_admin"))
        .unwrap_or(false);
    
    let permissions: Vec<String> = if is_super_admin {
        vec!["*:*:*".to_string()]
    } else {
        let rows = sqlx::query(
            r#"
            SELECT DISTINCT sm.permission
            FROM sys_user_roles sur
            JOIN sys_role_menus srm ON sur.role_id = srm.role_id
            JOIN sys_menus sm ON srm.menu_id = sm.id
            WHERE sur.user_id = $1 AND sm.permission IS NOT NULL AND sm.status = 'ACTIVE'
            "#,
        )
        .bind(user_id)
        .fetch_all(db)
        .await?;
        
        rows.into_iter()
            .filter_map(|row| row.get::<Option<String>, _>("permission"))
            .collect()
    };

    Ok(permissions)
}

pub async fn get_user_roles(db: &PgPool, user_id: Uuid) -> Result<Vec<String>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT sr.code
        FROM sys_user_roles sur
        JOIN sys_roles sr ON sur.role_id = sr.id
        WHERE sur.user_id = $1 AND sr.status = 'ACTIVE'
        "#,
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;
    
    let roles: Vec<String> = rows
        .into_iter()
        .map(|row| row.get::<String, _>(0))
        .collect();

    Ok(roles)
}

pub async fn get_user_menu_tree(db: &PgPool, user_id: Uuid) -> Result<serde_json::Value, AppError> {
    let is_super_admin_row = sqlx::query("SELECT is_super_admin FROM sys_users WHERE id = $1")
        .bind(user_id)
        .fetch_optional(db)
        .await?;
    
    let is_super_admin = is_super_admin_row
        .map(|row| row.get::<bool, _>("is_super_admin"))
        .unwrap_or(false);

    let menus: Vec<MenuRecord> = if is_super_admin {
        sqlx::query_as::<_, MenuRecord>(
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
                visible
            FROM sys_menus 
            WHERE status = 'ACTIVE'
            ORDER BY parent_id, sort_order
            "#
        ).fetch_all(db).await?
    } else {
        sqlx::query_as::<_, MenuRecord>(
            r#"
            SELECT DISTINCT
                sm.id,
                sm.parent_id,
                sm.name,
                sm.menu_type,
                sm.route_path,
                sm.component,
                sm.permission,
                sm.icon,
                sm.sort_order,
                sm.visible
            FROM sys_menus sm
            JOIN sys_role_menus srm ON sm.id = srm.menu_id
            JOIN sys_user_roles sur ON srm.role_id = sur.role_id
            WHERE sur.user_id = $1 AND sm.status = 'ACTIVE'
            ORDER BY sm.parent_id, sm.sort_order
            "#,
        ).bind(user_id).fetch_all(db).await?
    };

    let mut menu_map: std::collections::HashMap<uuid::Uuid, serde_json::Value> = std::collections::HashMap::new();
    let mut menu_children: std::collections::HashMap<uuid::Uuid, Vec<serde_json::Value>> = std::collections::HashMap::new();

    for menu in &menus {
        let menu_json = serde_json::json!({
            "id": menu.id,
            "name": menu.name,
            "menuType": menu.menu_type,
            "routePath": menu.route_path,
            "component": menu.component,
            "permission": menu.permission,
            "icon": menu.icon,
            "sortOrder": menu.sort_order,
            "visible": menu.visible,
            "children": []
        });
        menu_map.insert(menu.id, menu_json);
    }

    for menu in &menus {
        if let Some(parent_id) = menu.parent_id {
            menu_children.entry(parent_id).or_insert_with(Vec::new).push(
                menu_map.get(&menu.id).unwrap().clone()
            );
        }
    }

    for (parent_id, children) in menu_children {
        if let Some(parent_menu) = menu_map.get_mut(&parent_id) {
            if let Some(children_field) = parent_menu.as_object_mut().unwrap().get_mut("children") {
                *children_field = serde_json::Value::Array(children);
            }
        }
    }

    let root_menus: Vec<serde_json::Value> = menus
        .iter()
        .filter(|menu| menu.parent_id.is_none())
        .filter_map(|menu| menu_map.get(&menu.id).cloned())
        .collect();

    Ok(serde_json::json!(root_menus))
}