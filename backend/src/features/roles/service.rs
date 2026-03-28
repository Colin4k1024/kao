use crate::common::error::AppError;
use uuid::Uuid;

use super::{
    model::{CreateRoleRequest, RoleResponse, UpdateRoleRequest},
    repo::{
        assign_role_departments, assign_role_menus, create_role, delete_role, get_role_by_code,
        get_role_by_id, get_role_department_ids, get_role_menu_ids, list_roles, update_role,
    },
};

#[derive(Default)]
pub struct RoleService;

impl RoleService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn list_roles(&self, db: &sqlx::PgPool) -> Result<Vec<RoleResponse>, AppError> {
        let roles = list_roles(db).await?;
        Ok(roles
            .into_iter()
            .map(|r| RoleResponse {
                id: r.id,
                code: r.code,
                name: r.name,
                description: r.description,
                data_scope: r.data_scope,
                status: r.status,
                is_system: r.is_system,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect())
    }

    pub async fn get_role_by_id(
        &self,
        db: &sqlx::PgPool,
        role_id: Uuid,
    ) -> Result<Option<RoleResponse>, AppError> {
        let role = get_role_by_id(db, role_id).await?;
        Ok(role.map(|r| RoleResponse {
            id: r.id,
            code: r.code,
            name: r.name,
            description: r.description,
            data_scope: r.data_scope,
            status: r.status,
            is_system: r.is_system,
            created_at: r.created_at,
            updated_at: r.updated_at,
        }))
    }

    pub async fn create_role(
        &self,
        db: &sqlx::PgPool,
        req: CreateRoleRequest,
    ) -> Result<RoleResponse, AppError> {
        // Check if role code already exists
        if get_role_by_code(db, &req.code).await?.is_some() {
            return Err(AppError::Validation { field: "code".to_string(), message: "Role code already exists".to_string() });
        }

        let role = create_role(
            db,
            req.code,
            req.name,
            req.description,
            req.data_scope,
        )
        .await?;

        // Assign menus if provided
        if let Some(menu_ids) = req.menu_ids {
            assign_role_menus(db, role.id, &menu_ids).await?;
        }

        // Assign departments if provided (for CUSTOM scope)
        if let Some(dept_ids) = req.dept_ids {
            assign_role_departments(db, role.id, &dept_ids).await?;
        }

        Ok(RoleResponse {
            id: role.id,
            code: role.code,
            name: role.name,
            description: role.description,
            data_scope: role.data_scope,
            status: role.status,
            is_system: role.is_system,
            created_at: role.created_at,
            updated_at: role.updated_at,
        })
    }

    pub async fn update_role(
        &self,
        db: &sqlx::PgPool,
        role_id: Uuid,
        req: UpdateRoleRequest,
    ) -> Result<RoleResponse, AppError> {
        let role = update_role(
            db,
            role_id,
            req.name,
            req.description,
            req.data_scope,
            req.status,
        )
        .await?;

        // Update menus if provided
        if let Some(menu_ids) = req.menu_ids {
            assign_role_menus(db, role_id, &menu_ids).await?;
        }

        // Update departments if provided
        if let Some(dept_ids) = req.dept_ids {
            assign_role_departments(db, role_id, &dept_ids).await?;
        }

        Ok(RoleResponse {
            id: role.id,
            code: role.code,
            name: role.name,
            description: role.description,
            data_scope: role.data_scope,
            status: role.status,
            is_system: role.is_system,
            created_at: role.created_at,
            updated_at: role.updated_at,
        })
    }

    pub async fn delete_role(&self, db: &sqlx::PgPool, role_id: Uuid) -> Result<(), AppError> {
        delete_role(db, role_id).await
    }

    pub async fn get_role_permissions(
        &self,
        db: &sqlx::PgPool,
        role_id: Uuid,
    ) -> Result<(Vec<Uuid>, Vec<Uuid>), AppError> {
        let menu_ids = get_role_menu_ids(db, role_id).await?;
        let dept_ids = get_role_department_ids(db, role_id).await?;
        Ok((menu_ids, dept_ids))
    }
}