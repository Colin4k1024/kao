use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: Option<String>,
    pub display_name: String,
    pub password: String,
    pub dept_id: Option<Uuid>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub status: Option<String>, // ACTIVE, DISABLED, LOCKED
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub password: Option<String>,
    pub dept_id: Option<Uuid>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub display_name: String,
    pub dept_id: Option<Uuid>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub is_super_admin: bool,
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_login_ip: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct UserRecord {
    pub id: Uuid,
    pub username: String,
    pub email: Option<String>,
    pub display_name: String,
    pub password_hash: String,
    pub dept_id: Option<Uuid>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub status: String,
    pub is_super_admin: bool,
    pub last_login_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_login_ip: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}