use uuid::Uuid;
use validator::Validate;

use crate::common::{
    auth::claims::Claims,
    error::{AppError, AppResult},
    DbPool,
};

use super::{
    model::{
        CreateDepartmentRequest, DepartmentNode, DepartmentResponse, DepartmentsResponse,
        UpdateDepartmentRequest,
    },
    repo::{DepartmentInsert, DepartmentRow, DepartmentsRepo},
};

#[derive(Clone)]
pub struct DepartmentsService {
    repo: DepartmentsRepo,
}

impl DepartmentsService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            repo: DepartmentsRepo::new(pool),
        }
    }

    pub async fn list_department_tree(
        &self,
        claims: &Claims,
    ) -> AppResult<DepartmentsResponse> {
        self.require_access(claims, "system:dept:list")?;

        let rows = self.repo.list_departments().await?;
        Ok(DepartmentsResponse {
            departments: build_department_tree(rows.into_iter().map(DepartmentNode::from).collect()),
        })
    }

    pub async fn create_department(
        &self,
        claims: &Claims,
        request: CreateDepartmentRequest,
    ) -> AppResult<DepartmentResponse> {
        self.require_access(claims, "system:dept:add")?;
        request
            .validate()
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        let id = Uuid::new_v4().to_string();
        let parent = self.load_parent(request.parent_id.as_deref()).await?;
        let (ancestors, path) = build_ancestry(parent.as_ref(), &request.name);
        let status = normalize_department_status(request.status.as_deref())?;
        let sort_order = request.sort_order.unwrap_or(0);

        self.repo
            .create_department(DepartmentInsert {
                id: &id,
                code: &request.code,
                name: &request.name,
                parent_id: request.parent_id.as_deref(),
                ancestors: &ancestors,
                path: &path,
                sort_order,
                leader: request.leader.as_deref(),
                phone: request.phone.as_deref(),
                email: request.email.as_deref(),
                status: &status,
            })
            .await?;

        Ok(DepartmentResponse {
            department: self.load_department_node(&id).await?,
        })
    }

    pub async fn update_department(
        &self,
        claims: &Claims,
        department_id: &str,
        request: UpdateDepartmentRequest,
    ) -> AppResult<DepartmentResponse> {
        self.require_access(claims, "system:dept:edit")?;
        request
            .validate()
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        let parent = self.load_parent(request.parent_id.as_deref()).await?;
        let (ancestors, path) = build_ancestry(parent.as_ref(), &request.name);
        let status = normalize_department_status(request.status.as_deref())?;
        let sort_order = request.sort_order.unwrap_or(0);

        self.repo
            .update_department(DepartmentInsert {
                id: department_id,
                code: &request.code,
                name: &request.name,
                parent_id: request.parent_id.as_deref(),
                ancestors: &ancestors,
                path: &path,
                sort_order,
                leader: request.leader.as_deref(),
                phone: request.phone.as_deref(),
                email: request.email.as_deref(),
                status: &status,
            })
            .await?;

        Ok(DepartmentResponse {
            department: self.load_department_node(department_id).await?,
        })
    }

    async fn load_department_node(&self, department_id: &str) -> AppResult<DepartmentNode> {
        let row = self.repo.find_department_by_id(department_id).await?;
        Ok(DepartmentNode::from(row))
    }

    async fn load_parent(&self, parent_id: Option<&str>) -> AppResult<Option<DepartmentRow>> {
        match parent_id {
            Some(parent_id) => Ok(Some(self.repo.find_department_by_id(parent_id).await?)),
            None => Ok(None),
        }
    }

    fn require_access(&self, claims: &Claims, permission: &str) -> AppResult<()> {
        if is_super_admin(claims) || claims.permissions.iter().any(|item| item == permission) {
            return Ok(());
        }

        Err(AppError::Forbidden(format!(
            "missing permission `{}`",
            permission
        )))
    }
}

pub fn build_department_tree(rows: Vec<DepartmentNode>) -> Vec<DepartmentNode> {
    use std::collections::HashMap;

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
        nodes.insert(row.id.clone(), row);
    }

    fn build_node(
        node_id: &str,
        nodes: &std::collections::HashMap<String, DepartmentNode>,
        children_by_parent: &std::collections::HashMap<Option<String>, Vec<String>>,
    ) -> DepartmentNode {
        let mut node = nodes
            .get(node_id)
            .expect("department row should exist for every tree node")
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

fn build_ancestry(parent: Option<&DepartmentRow>, name: &str) -> (String, String) {
    match parent {
        Some(parent) => {
            let ancestors = if parent.ancestors.is_empty() {
                parent.id.clone()
            } else {
                format!("{},{}", parent.ancestors, parent.id)
            };
            let path = format!("{}/{}", parent.path.trim_end_matches('/'), name);
            (ancestors, path)
        }
        None => (String::new(), format!("/{}", name)),
    }
}

fn normalize_department_status(status: Option<&str>) -> AppResult<String> {
    let normalized = status.unwrap_or("ACTIVE").to_uppercase();

    match normalized.as_str() {
        "ACTIVE" | "DISABLED" => Ok(normalized),
        other => Err(AppError::BadRequest(format!(
            "invalid department status `{}`",
            other
        ))),
    }
}

fn is_super_admin(claims: &Claims) -> bool {
    claims.roles.iter().any(|role| role == "SUPER_ADMIN")
}
