use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub post_name: String,
    pub post_code: String,
    pub post_group: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePostRequest {
    pub post_name: Option<String>,
    pub post_code: Option<String>,
    pub post_group: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PostResponse {
    pub id: i64,
    pub post_name: String,
    pub post_code: String,
    pub post_group: Option<String>,
    pub sort: i32,
    pub status: i32,
    pub created_by: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct PostRecord {
    pub id: i64,
    pub post_name: String,
    pub post_code: String,
    pub post_group: Option<String>,
    pub sort: i32,
    pub status: i32,
    pub created_by: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
