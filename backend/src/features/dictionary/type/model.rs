use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateTypeRequest {
    pub dict_name: String,
    pub dict_type: String,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTypeRequest {
    pub dict_name: Option<String>,
    pub dict_type: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TypeResponse {
    pub id: Uuid,
    pub dict_name: String,
    pub dict_type: String,
    pub status: i32,
    pub remark: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct TypeRecord {
    pub id: Uuid,
    pub dict_name: String,
    pub dict_type: String,
    pub status: i32,
    pub remark: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
