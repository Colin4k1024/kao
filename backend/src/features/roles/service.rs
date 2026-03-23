use uuid::Uuid;
use validator::Validate;

use crate::common::{
    auth::claims::Claims,
    error::{AppError, AppResult},
    DbPool,
};

use super::{
    model::{
        CreateRoleRequest, RoleDetail, RoleListItem, RoleResponse, RolesResponse,
        UpdateRoleRequest,
    },
    repo::{RoleRow, RolesRepo},
};

#[derive(Clone)]
pub struct RolesService {
    repo: RolesRepo,
}

impl RolesService {
    pub fn new(pool: DbPool) -> Self {
        Self {
            repo: RolesRepo::new(pool),
        }
    }

    pub async fn list_roles(&self, claims: &Claims) -> AppResult<RolesResponse> {
        self.require_access(claims, "system:role:list")?;

        let rows = self.repo.list_roles().await?;
        let roles = rows
            .into_iter()
            .map(|row| self.to_list_item(row))
            .collect();

        Ok(RolesResponse { roles })
    }

    pub async fn create_role(
        &self,
        claims: &Claims,
        request: CreateRoleRequest,
    ) -> AppResult<RoleResponse> {
        self.require_access(claims, "system:role:add")?;
        request
            .validate()
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        let data_scope = normalize_data_scope(&request.data_scope)?;
        let status = normalize_role_status(request.status.as_deref())?;
        let role_id = Uuid::new_v4().to_string();

        self.repo
            .create_role(
                &role_id,
                &request.code,
                &request.name,
                request.description.as_deref(),
                &data_scope,
                &status,
            )
            .await?;
        self.repo
            .replace_role_menus(&role_id, &request.menu_ids)
            .await?;
        self.repo
            .replace_role_departments(&role_id, &request.department_ids)
            .await?;

        Ok(RoleResponse {
            role: self.load_role_detail(&role_id).await?,
        })
    }

    pub async fn update_role(
        &self,
        claims: &Claims,
        role_id: &str,
        request: UpdateRoleRequest,
    ) -> AppResult<RoleResponse> {
        self.require_access(claims, "system:role:edit")?;
        request
            .validate()
            .map_err(|err| AppError::BadRequest(err.to_string()))?;

        let data_scope = normalize_data_scope(&request.data_scope)?;
        let status = normalize_role_status(request.status.as_deref())?;

        self.repo
            .update_role(
                role_id,
                &request.code,
                &request.name,
                request.description.as_deref(),
                &data_scope,
                &status,
            )
            .await?;
        self.repo
            .replace_role_menus(role_id, &request.menu_ids)
            .await?;
        self.repo
            .replace_role_departments(role_id, &request.department_ids)
            .await?;

        Ok(RoleResponse {
            role: self.load_role_detail(role_id).await?,
        })
    }

    async fn load_role_detail(&self, role_id: &str) -> AppResult<RoleDetail> {
        let row = self.repo.find_role_by_id(role_id).await?;
        self.to_detail(row).await
    }

    async fn to_detail(&self, row: RoleRow) -> AppResult<RoleDetail> {
        let menu_ids = self.repo.list_menu_ids_by_role_id(&row.id).await?;
        let department_ids = self.repo.list_department_ids_by_role_id(&row.id).await?;

        Ok(RoleDetail {
            id: row.id,
            code: row.code,
            name: row.name,
            description: row.description,
            data_scope: row.data_scope,
            status: row.status,
            is_system: row.is_system,
            menu_ids,
            department_ids,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }

    fn to_list_item(&self, row: RoleRow) -> RoleListItem {
        RoleListItem {
            id: row.id,
            code: row.code,
            name: row.name,
            description: row.description,
            data_scope: row.data_scope,
            status: row.status,
            is_system: row.is_system,
            created_at: row.created_at,
            updated_at: row.updated_at,
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

fn normalize_role_status(status: Option<&str>) -> AppResult<String> {
    let normalized = status.unwrap_or("ACTIVE").to_uppercase();

    match normalized.as_str() {
        "ACTIVE" | "DISABLED" => Ok(normalized),
        other => Err(AppError::BadRequest(format!(
            "invalid role status `{}`",
            other
        ))),
    }
}

fn normalize_data_scope(data_scope: &str) -> AppResult<String> {
    let normalized = data_scope.to_uppercase();

    match normalized.as_str() {
        "ALL" | "CUSTOM" | "DEPT" | "DEPT_AND_CHILD" | "SELF" => Ok(normalized),
        other => Err(AppError::BadRequest(format!(
            "invalid data scope `{}`",
            other
        ))),
    }
}

fn is_super_admin(claims: &Claims) -> bool {
    claims.roles.iter().any(|role| role == "SUPER_ADMIN")
}
