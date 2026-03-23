use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateRoleRequest {
    #[validate(length(min = 2, max = 64))]
    pub code: String,
    #[validate(length(min = 2, max = 128))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 2, max = 32))]
    pub data_scope: String,
    pub status: Option<String>,
    pub menu_ids: Vec<String>,
    pub department_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 2, max = 64))]
    pub code: String,
    #[validate(length(min = 2, max = 128))]
    pub name: String,
    pub description: Option<String>,
    #[validate(length(min = 2, max = 32))]
    pub data_scope: String,
    pub status: Option<String>,
    pub menu_ids: Vec<String>,
    pub department_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleListItem {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub data_scope: String,
    pub status: String,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleDetail {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub data_scope: String,
    pub status: String,
    pub is_system: bool,
    pub menu_ids: Vec<String>,
    pub department_ids: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RoleResponse {
    pub role: RoleDetail,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RolesResponse {
    pub roles: Vec<RoleListItem>,
}
