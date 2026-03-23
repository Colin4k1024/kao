use crate::features::menus::model::MenuTreeResponse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::repo::{RoleRow, UserSessionRow};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 2, max = 64))]
    pub username: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub dept_id: Option<String>,
    pub dept_name: Option<String>,
    pub status: String,
    pub is_super_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RoleSummary {
    pub id: String,
    pub code: String,
    pub name: String,
    pub data_scope: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProfileResponse {
    pub user: UserProfile,
    pub roles: Vec<RoleSummary>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PermissionsResponse {
    pub role_ids: Vec<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub profile: ProfileResponse,
    pub permissions: PermissionsResponse,
    pub menus: MenuTreeResponse,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionSnapshot {
    pub user_id: String,
    pub email: String,
    pub dept_id: Option<String>,
    pub role_ids: Vec<String>,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SessionBundle {
    pub profile: ProfileResponse,
    pub permissions: PermissionsResponse,
    pub menus: MenuTreeResponse,
}

impl From<&UserSessionRow> for UserProfile {
    fn from(row: &UserSessionRow) -> Self {
        Self {
            id: row.id.clone(),
            username: row.username.clone(),
            email: row.email.clone(),
            display_name: row.display_name.clone(),
            avatar_url: row.avatar_url.clone(),
            phone: row.phone.clone(),
            dept_id: row.dept_id.clone(),
            dept_name: row.dept_name.clone(),
            status: row.status.clone(),
            is_super_admin: row.is_super_admin,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

impl From<&RoleRow> for RoleSummary {
    fn from(row: &RoleRow) -> Self {
        Self {
            id: row.id.clone(),
            code: row.code.clone(),
            name: row.name.clone(),
            data_scope: row.data_scope.clone(),
        }
    }
}
