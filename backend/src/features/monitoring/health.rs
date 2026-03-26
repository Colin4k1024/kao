use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct HealthCheck {
    pub status: HealthStatus,
    #[serde(skip)]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthCheckResponse {
    pub status: HealthStatus,
    pub checks: HealthChecks,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthChecks {
    pub database: HealthStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redis: Option<HealthStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_scheduler: Option<HealthStatus>,
}

pub async fn check_health() -> Response {
    // Check database connection
    let db_status = check_database().await;
    
    // Check Redis (optional)
    let redis_status = check_redis().await;
    
    // Check job scheduler (optional)
    let job_scheduler_status = check_job_scheduler().await;
    
    // Determine overall status
    let overall_status = determine_overall_status(&db_status, redis_status.as_ref(), job_scheduler_status.as_ref());
    
    let response = HealthCheckResponse {
        status: overall_status,
        checks: HealthChecks {
            database: db_status,
            redis: redis_status,
            job_scheduler: job_scheduler_status,
        },
        timestamp: Utc::now().to_rfc3339(),
    };
    
    Json(response).into_response()
}

async fn check_database() -> HealthStatus {
    // For now, return degraded as placeholder since database connection may not be available
    // In production, actually test database connection using sqlx
    HealthStatus::Degraded
}

async fn check_redis() -> Option<HealthStatus> {
    // Redis is optional, return None if not configured
    if std::env::var("REDIS_URL").is_err() {
        return None;
    }
    
    // In production, actually test Redis connection
    Some(HealthStatus::Healthy)
}

async fn check_job_scheduler() -> Option<HealthStatus> {
    // Job scheduler is optional, return None if not configured
    None
}

fn determine_overall_status(
    db_status: &HealthStatus,
    redis_status: Option<&HealthStatus>,
    job_scheduler_status: Option<&HealthStatus>,
) -> HealthStatus {
    // If any critical dependency is unhealthy, overall is unhealthy
    if db_status == &HealthStatus::Unhealthy {
        return HealthStatus::Unhealthy;
    }
    
    // If any dependency is degraded, overall is degraded
    if db_status == &HealthStatus::Degraded 
        || redis_status == Some(&HealthStatus::Degraded)
        || job_scheduler_status == Some(&HealthStatus::Degraded)
    {
        return HealthStatus::Degraded;
    }
    
    HealthStatus::Healthy
}
