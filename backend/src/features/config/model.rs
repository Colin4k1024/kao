use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateConfigRequest {
    pub config_name: String,
    pub config_key: String,
    pub config_value: String,
    pub config_type: Option<String>,
    pub is_encrypt: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateConfigRequest {
    pub config_name: Option<String>,
    pub config_key: Option<String>,
    pub config_value: Option<String>,
    pub config_type: Option<String>,
    pub is_encrypt: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ConfigResponse {
    pub id: Uuid,
    pub config_name: String,
    pub config_key: String,
    pub config_value: String,
    pub config_type: String,
    pub is_encrypt: String,
    pub status: i32,
    pub remark: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct ConfigRecord {
    pub id: Uuid,
    pub config_name: String,
    pub config_key: String,
    pub config_value: String,
    pub config_type: String,
    pub is_encrypt: String,
    pub status: i32,
    pub remark: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
