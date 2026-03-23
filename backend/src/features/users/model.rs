use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 2, max = 64))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 2, max = 128))]
    pub display_name: String,
    #[validate(length(min = 6, max = 128))]
    pub password: String,
    pub dept_id: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
    #[validate(length(min = 1))]
    pub role_ids: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 2, max = 64))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 2, max = 128))]
    pub display_name: String,
    pub dept_id: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub status: Option<String>,
    #[validate(length(min = 1))]
    pub role_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserListItem {
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
    pub role_ids: Vec<String>,
    pub role_codes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub user: UserListItem,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UsersResponse {
    pub users: Vec<UserListItem>,
}

