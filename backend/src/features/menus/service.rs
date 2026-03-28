use crate::common::error::AppError;
use uuid::Uuid;

use super::{
    model::{CreateMenuRequest, MenuResponse, MenuTreeItem},
    repo::{create_menu, delete_menu, get_menu_by_id, get_menu_tree, update_menu},
};

#[derive(Default)]
pub struct MenuService;

impl MenuService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_menu_tree(&self, db: &sqlx::PgPool) -> Result<Vec<MenuTreeItem>, AppError> {
        let menus = get_menu_tree(db).await?;
        
        let mut menu_map: std::collections::HashMap<uuid::Uuid, MenuTreeItem> = std::collections::HashMap::new();
        let mut menu_children: std::collections::HashMap<uuid::Uuid, Vec<MenuTreeItem>> = std::collections::HashMap::new();

        for menu in &menus {
            let menu_item = MenuTreeItem {
                id: menu.id,
                parent_id: menu.parent_id,
                name: menu.name.clone(),
                menu_type: menu.menu_type.clone(),
                route_path: menu.route_path.clone(),
                component: menu.component.clone(),
                permission: menu.permission.clone(),
                icon: menu.icon.clone(),
                sort_order: menu.sort_order,
                visible: menu.visible,
                status: menu.status.clone(),
                children: vec![],
            };
            menu_map.insert(menu.id, menu_item);
        }

        for menu in &menus {
            if let Some(parent_id) = menu.parent_id {
                if let Some(item) = menu_map.get(&menu.id).cloned() {
                    menu_children
                        .entry(parent_id)
                        .or_default()
                        .push(item);
                }
            }
        }

        for (parent_id, children) in menu_children {
            if let Some(parent_menu) = menu_map.get_mut(&parent_id) {
                parent_menu.children = children;
            }
        }

        let roots: Vec<MenuTreeItem> = menus
            .iter()
            .filter(|menu| menu.parent_id.is_none())
            .filter_map(|menu| menu_map.get(&menu.id).cloned())
            .collect();

        Ok(roots)
    }

    pub async fn get_menu_by_id(
        &self,
        db: &sqlx::PgPool,
        menu_id: Uuid,
    ) -> Result<Option<MenuResponse>, AppError> {
        let menu = get_menu_by_id(db, menu_id).await?;
        Ok(menu.map(|m| MenuResponse {
            id: m.id,
            parent_id: m.parent_id,
            name: m.name,
            menu_type: m.menu_type,
            route_path: m.route_path,
            component: m.component,
            permission: m.permission,
            icon: m.icon,
            sort_order: m.sort_order,
            visible: m.visible,
            status: m.status,
            created_at: m.created_at,
            updated_at: m.updated_at,
        }))
    }

    pub async fn create_menu(
        &self,
        db: &sqlx::PgPool,
        req: CreateMenuRequest,
    ) -> Result<MenuResponse, AppError> {
        let menu = create_menu(
            db,
            req.parent_id,
            req.name,
            req.menu_type,
            req.route_path,
            req.component,
            req.permission,
            req.icon,
            req.sort_order.unwrap_or(0),
            req.visible.unwrap_or(true),
        )
        .await?;

        Ok(MenuResponse {
            id: menu.id,
            parent_id: menu.parent_id,
            name: menu.name,
            menu_type: menu.menu_type,
            route_path: menu.route_path,
            component: menu.component,
            permission: menu.permission,
            icon: menu.icon,
            sort_order: menu.sort_order,
            visible: menu.visible,
            status: menu.status,
            created_at: menu.created_at,
            updated_at: menu.updated_at,
        })
    }

    pub async fn update_menu(
        &self,
        db: &sqlx::PgPool,
        menu_id: Uuid,
        req: CreateMenuRequest,
    ) -> Result<MenuResponse, AppError> {
        let menu = update_menu(
            db,
            menu_id,
            req.parent_id,
            req.name,
            req.menu_type,
            req.route_path,
            req.component,
            req.permission,
            req.icon,
            req.sort_order.unwrap_or(0),
            req.visible.unwrap_or(true),
        )
        .await?;

        Ok(MenuResponse {
            id: menu.id,
            parent_id: menu.parent_id,
            name: menu.name,
            menu_type: menu.menu_type,
            route_path: menu.route_path,
            component: menu.component,
            permission: menu.permission,
            icon: menu.icon,
            sort_order: menu.sort_order,
            visible: menu.visible,
            status: menu.status,
            created_at: menu.created_at,
            updated_at: menu.updated_at,
        })
    }

    pub async fn delete_menu(&self, db: &sqlx::PgPool, menu_id: Uuid) -> Result<(), AppError> {
        delete_menu(db, menu_id).await
    }
}