use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateNoticeRequest {
    pub notice_title: String,
    pub notice_type: String,
    pub notice_content: Option<String>,
    pub notice_status: Option<String>,
    pub is_top: Option<String>,
    pub priority: Option<i32>,
    pub publish_time: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoticeRequest {
    pub notice_title: Option<String>,
    pub notice_type: Option<String>,
    pub notice_content: Option<String>,
    pub notice_status: Option<String>,
    pub is_top: Option<String>,
    pub priority: Option<i32>,
    pub publish_time: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct NoticeResponse {
    pub id: Uuid,
    pub notice_title: String,
    pub notice_type: String,
    pub notice_content: Option<String>,
    pub notice_status: String,
    pub is_top: String,
    pub priority: i32,
    pub publish_time: Option<String>,
    pub view_count: i32,
    pub publisher_id: Option<Uuid>,
    pub publisher_name: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct NoticeRecord {
    pub id: Uuid,
    pub notice_title: String,
    pub notice_type: String,
    pub notice_content: Option<String>,
    pub notice_status: String,
    pub is_top: String,
    pub priority: i32,
    pub publish_time: Option<chrono::DateTime<chrono::Utc>>,
    pub view_count: i32,
    pub publisher_id: Option<Uuid>,
    pub publisher_name: Option<String>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}
