use std::collections::HashMap;

use crate::common::{AppResult, DbPool};

use super::{
    model::{MenuNode, MenuRow, MenuTreeResponse},
    repo::MenusRepo,
};

#[derive(Clone)]
pub struct MenusService {
    repo: MenusRepo,
}

impl MenusService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            repo: MenusRepo::new(pool),
        }
    }

    pub async fn list_menu_tree_by_user_id(&self, user_id: &str) -> AppResult<MenuTreeResponse> {
        let rows = self.repo.list_menu_rows_by_user_id(user_id).await?;
        Ok(MenuTreeResponse {
            menus: build_menu_tree(rows),
        })
    }
}

pub fn build_menu_tree(rows: Vec<MenuRow>) -> Vec<MenuNode> {
    let mut rows = rows;
    rows.sort_by(|left, right| {
        left.sort_order
            .cmp(&right.sort_order)
            .then_with(|| left.name.cmp(&right.name))
            .then_with(|| left.id.cmp(&right.id))
    });
    rows.dedup_by(|left, right| left.id == right.id);

    let mut nodes = HashMap::new();
    let mut children_by_parent: HashMap<Option<String>, Vec<String>> = HashMap::new();

    for row in rows {
        children_by_parent
            .entry(row.parent_id.clone())
            .or_default()
            .push(row.id.clone());
        nodes.insert(row.id.clone(), MenuNode::from(row));
    }

    fn build_node(
        node_id: &str,
        nodes: &HashMap<String, MenuNode>,
        children_by_parent: &HashMap<Option<String>, Vec<String>>,
    ) -> MenuNode {
        let mut node = nodes
            .get(node_id)
            .expect("menu row should exist for every tree node")
            .clone();
        let child_ids = children_by_parent
            .get(&Some(node_id.to_string()))
            .cloned()
            .unwrap_or_default();
        node.children = child_ids
            .into_iter()
            .map(|child_id| build_node(&child_id, nodes, children_by_parent))
            .collect();
        node
    }

    children_by_parent
        .get(&None)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .map(|root_id| build_node(&root_id, &nodes, &children_by_parent))
        .collect()
}
