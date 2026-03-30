use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Job status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Stopped = 0,
    Running = 1,
}

impl From<i32> for JobStatus {
    fn from(v: i32) -> Self {
        match v {
            1 => JobStatus::Running,
            _ => JobStatus::Stopped,
        }
    }
}

impl From<JobStatus> for i32 {
    fn from(s: JobStatus) -> Self {
        s as i32
    }
}

/// Job execute status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecuteStatus {
    Pending = 0,
    Running = 1,
    Success = 2,
    Failed = 3,
}

/// Database record for job
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct JobRecord {
    pub id: i64,
    pub job_name: String,
    pub job_code: String,
    pub job_group: String,
    pub job_status: i32,
    pub cron_expression: String,
    pub retry_count: i32,
    pub retry_interval: i32,
    pub timeout: i32,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// API response for job
#[derive(Debug, Clone, Serialize)]
pub struct Job {
    pub id: i64,
    pub job_name: String,
    pub job_code: String,
    pub job_group: String,
    pub job_status: i32,
    pub cron_expression: String,
    pub retry_count: i32,
    pub retry_interval: i32,
    pub timeout: i32,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<JobRecord> for Job {
    fn from(r: JobRecord) -> Self {
        Self {
            id: r.id,
            job_name: r.job_name,
            job_code: r.job_code,
            job_group: r.job_group,
            job_status: r.job_status,
            cron_expression: r.cron_expression,
            retry_count: r.retry_count,
            retry_interval: r.retry_interval,
            timeout: r.timeout,
            description: r.description,
            created_by: r.created_by,
            created_at: r.created_at.to_rfc3339(),
            updated_at: r.updated_at.to_rfc3339(),
        }
    }
}

/// Database record for job log
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct JobLogRecord {
    pub id: i64,
    pub job_id: i64,
    pub job_name: String,
    pub job_code: String,
    pub job_group: String,
    pub execute_status: i32,
    pub execute_message: Option<String>,
    pub execute_time: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// API response for job log
#[derive(Debug, Clone, Serialize)]
pub struct JobLog {
    pub id: i64,
    pub job_id: i64,
    pub job_name: String,
    pub job_code: String,
    pub job_group: String,
    pub execute_status: i32,
    pub execute_message: Option<String>,
    pub execute_time: String,
    pub created_at: String,
}

impl From<JobLogRecord> for JobLog {
    fn from(r: JobLogRecord) -> Self {
        Self {
            id: r.id,
            job_id: r.job_id,
            job_name: r.job_name,
            job_code: r.job_code,
            job_group: r.job_group,
            execute_status: r.execute_status,
            execute_message: r.execute_message,
            execute_time: r.execute_time.unwrap_or_default(),
            created_at: r.created_at.to_rfc3339(),
        }
    }
}

/// Request to create a new job
#[derive(Debug, Clone, Deserialize)]
pub struct CreateJobRequest {
    pub job_name: String,
    pub job_code: String,
    pub job_group: String,
    pub job_status: Option<i32>,
    pub cron_expression: String,
    pub retry_count: Option<i32>,
    pub retry_interval: Option<i32>,
    pub timeout: Option<i32>,
    pub description: Option<String>,
}

/// Request to update an existing job
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateJobRequest {
    pub job_name: Option<String>,
    pub job_group: Option<String>,
    pub job_status: Option<i32>,
    pub cron_expression: Option<String>,
    pub retry_count: Option<i32>,
    pub retry_interval: Option<i32>,
    pub timeout: Option<i32>,
    pub description: Option<String>,
}

/// Query parameters for listing jobs
#[derive(Debug, Clone, Deserialize, Default)]
pub struct JobListParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub keyword: Option<String>,
    pub job_name: Option<String>,
    pub job_code: Option<String>,
    pub job_status: Option<i32>,
}

/// Query parameters for listing job logs
#[derive(Debug, Clone, Deserialize, Default)]
pub struct JobLogListParams {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub job_id: Option<i64>,
    pub job_name: Option<String>,
    pub execute_status: Option<i32>,
}

/// Response for job list
#[derive(Debug, Clone, Serialize)]
pub struct JobListResponse {
    pub list: Vec<Job>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// Response for job log list
#[derive(Debug, Clone, Serialize)]
pub struct JobLogListResponse {
    pub list: Vec<JobLog>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}
