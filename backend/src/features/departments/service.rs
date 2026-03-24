use crate::common::error::AppError;
use uuid::Uuid;

use super::{
    model::{CreateDepartmentRequest, DepartmentResponse, DepartmentTreeItem, UpdateDepartmentRequest},
    repo::{
        create_department, delete_department, get_child_departments, get_department_by_code,
        get_department_by_id, list_departments, update_department,
    },
};

pub struct DepartmentService;

impl DepartmentService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_department_tree(
        &self,
        db: &sqlx::PgPool,
    ) -> Result<Vec<DepartmentTreeItem>, AppError> {
        let depts = list_departments(db).await?;

        let mut dept_map: std::collections::HashMap<Uuid, DepartmentTreeItem> =
            std::collections::HashMap::new();
        let mut dept_children: std::collections::HashMap<Uuid, Vec<DepartmentTreeItem>> =
            std::collections::HashMap::new();

        for dept in &depts {
            let tree_item = DepartmentTreeItem {
                id: dept.id,
                parent_id: dept.parent_id,
                code: dept.code.clone(),
                name: dept.name.clone(),
                ancestors: dept.ancestors.clone(),
                path: dept.path.clone(),
                sort_order: dept.sort_order,
                leader: dept.leader.clone(),
                phone: dept.phone.clone(),
                email: dept.email.clone(),
                status: dept.status.clone(),
                children: vec![],
            };
            dept_map.insert(dept.id, tree_item);
        }

        for dept in &depts {
            if let Some(parent_id) = dept.parent_id {
                if let Some(item) = dept_map.get(&dept.id).cloned() {
                    dept_children
                        .entry(parent_id)
                        .or_insert_with(Vec::new)
                        .push(item);
                }
            }
        }

        for (parent_id, children) in dept_children {
            if let Some(parent) = dept_map.get_mut(&parent_id) {
                parent.children = children;
            }
        }

        let roots: Vec<DepartmentTreeItem> = depts
            .iter()
            .filter(|d| d.parent_id.is_none())
            .filter_map(|d| dept_map.get(&d.id).cloned())
            .collect();

        Ok(roots)
    }

    pub async fn get_department_by_id(
        &self,
        db: &sqlx::PgPool,
        dept_id: Uuid,
    ) -> Result<Option<DepartmentResponse>, AppError> {
        let dept = get_department_by_id(db, dept_id).await?;
        Ok(dept.map(|d| DepartmentResponse {
            id: d.id,
            parent_id: d.parent_id,
            code: d.code,
            name: d.name,
            ancestors: d.ancestors,
            path: d.path,
            sort_order: d.sort_order,
            leader: d.leader,
            phone: d.phone,
            email: d.email,
            status: d.status,
            created_at: d.created_at,
            updated_at: d.updated_at,
        }))
    }

    pub async fn create_department(
        &self,
        db: &sqlx::PgPool,
        req: CreateDepartmentRequest,
    ) -> Result<DepartmentResponse, AppError> {
        if get_department_by_code(db, &req.code).await?.is_some() {
            return Err(AppError::Validation(
                "Department code already exists".to_string(),
            ));
        }

        let dept = create_department(
            db,
            req.parent_id,
            req.code,
            req.name,
            req.sort_order.unwrap_or(0),
            req.leader,
            req.phone,
            req.email,
        )
        .await?;

        Ok(DepartmentResponse {
            id: dept.id,
            parent_id: dept.parent_id,
            code: dept.code,
            name: dept.name,
            ancestors: dept.ancestors,
            path: dept.path,
            sort_order: dept.sort_order,
            leader: dept.leader,
            phone: dept.phone,
            email: dept.email,
            status: dept.status,
            created_at: dept.created_at,
            updated_at: dept.updated_at,
        })
    }

    pub async fn update_department(
        &self,
        db: &sqlx::PgPool,
        dept_id: Uuid,
        req: UpdateDepartmentRequest,
    ) -> Result<DepartmentResponse, AppError> {
        let dept = update_department(
            db,
            dept_id,
            req.parent_id,
            req.code,
            req.name,
            req.sort_order,
            req.leader,
            req.phone,
            req.email,
            req.status,
        )
        .await?;

        Ok(DepartmentResponse {
            id: dept.id,
            parent_id: dept.parent_id,
            code: dept.code,
            name: dept.name,
            ancestors: dept.ancestors,
            path: dept.path,
            sort_order: dept.sort_order,
            leader: dept.leader,
            phone: dept.phone,
            email: dept.email,
            status: dept.status,
            created_at: dept.created_at,
            updated_at: dept.updated_at,
        })
    }

    pub async fn delete_department(&self, db: &sqlx::PgPool, dept_id: Uuid) -> Result<(), AppError> {
        delete_department(db, dept_id).await
    }
}