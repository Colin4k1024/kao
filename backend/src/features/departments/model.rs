use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateDepartmentRequest {
    #[validate(length(min = 2, max = 64))]
    pub code: String,
    #[validate(length(min = 2, max = 128))]
    pub name: String,
    pub parent_id: Option<String>,
    pub sort_order: Option<i32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateDepartmentRequest {
    #[validate(length(min = 2, max = 64))]
    pub code: String,
    #[validate(length(min = 2, max = 128))]
    pub name: String,
    pub parent_id: Option<String>,
    pub sort_order: Option<i32>,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentRow {
    pub id: String,
    pub code: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub ancestors: String,
    pub path: String,
    pub sort_order: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentNode {
    pub id: String,
    pub code: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub ancestors: String,
    pub path: String,
    pub sort_order: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub children: Vec<DepartmentNode>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentsResponse {
    pub departments: Vec<DepartmentNode>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DepartmentResponse {
    pub department: DepartmentNode,
}

impl From<DepartmentRow> for DepartmentNode {
    fn from(row: DepartmentRow) -> Self {
        Self {
            id: row.id,
            code: row.code,
            name: row.name,
            parent_id: row.parent_id,
            ancestors: row.ancestors,
            path: row.path,
            sort_order: row.sort_order,
            leader: row.leader,
            phone: row.phone,
            email: row.email,
            status: row.status,
            created_at: row.created_at,
            updated_at: row.updated_at,
            children: Vec::new(),
        }
    }
}
