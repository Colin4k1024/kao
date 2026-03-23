use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct MenuRow {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub menu_type: String,
    pub route_path: Option<String>,
    pub component: Option<String>,
    pub permission: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub visible: bool,
    pub keep_alive: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuNode {
    pub id: String,
    pub parent_id: Option<String>,
    pub name: String,
    pub menu_type: String,
    pub route_path: Option<String>,
    pub component: Option<String>,
    pub permission: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub visible: bool,
    pub keep_alive: bool,
    pub children: Vec<MenuNode>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MenuTreeResponse {
    pub menus: Vec<MenuNode>,
}

impl From<MenuRow> for MenuNode {
    fn from(row: MenuRow) -> Self {
        Self {
            id: row.id,
            parent_id: row.parent_id,
            name: row.name,
            menu_type: row.menu_type,
            route_path: row.route_path,
            component: row.component,
            permission: row.permission,
            icon: row.icon,
            sort_order: row.sort_order,
            visible: row.visible,
            keep_alive: row.keep_alive,
            children: Vec::new(),
        }
    }
}
