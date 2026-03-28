use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateMenuRequest {
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub menu_type: String, // DIRECTORY, MENU, BUTTON
    pub route_path: Option<String>,
    pub component: Option<String>,
    pub permission: Option<String>,
    pub icon: Option<String>,
    pub sort_order: Option<i32>,
    pub visible: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct MenuResponse {
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
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuTreeItem {
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
    pub status: String,
    pub children: Vec<MenuTreeItem>,
}

#[derive(sqlx::FromRow, Debug, Clone)]
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
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}