use axum::{
    response::{IntoResponse, Response},
    Json,
};
use chrono::Utc;
use serde::Serialize;
use std::env;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<HealthStatus>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetricsEndpointStatus {
    pub endpoint: String,
    pub status: HealthStatus,
    pub metrics_available: bool,
    pub metrics_count: usize,
}

pub async fn check_health() -> Response {
    let _start_time = chrono::Utc::now();
    
    // Check database connection
    let db_status = check_database().await;
    
    // Check Redis (optional)
    let redis_status = check_redis().await;
    
    // Check job scheduler (optional)
    let job_scheduler_status = check_job_scheduler().await;
    
    // Check metrics endpoint
    let metrics_status = check_metrics().await;
    
    // Determine overall status
    let overall_status = determine_overall_status(
        &db_status, 
        redis_status.as_ref(), 
        job_scheduler_status.as_ref(),
        metrics_status.as_ref(),
    );
    
    let response = HealthCheckResponse {
        status: overall_status,
        checks: HealthChecks {
            database: db_status,
            redis: redis_status,
            job_scheduler: job_scheduler_status,
            metrics: metrics_status,
        },
        timestamp: Utc::now().to_rfc3339(),
    };
    
    Json(response).into_response()
}

async fn check_database() -> HealthStatus {
    // Try to execute a simple query to verify database connection
    use crate::common::db;
    use crate::config::Settings;
    
    let settings = Settings::new();
    match db::create_pool(&settings).await {
        Ok(_pool) => {
            HealthStatus::Healthy
        }
        Err(_) => {
            HealthStatus::Unhealthy
        }
    }
}

async fn check_redis() -> Option<HealthStatus> {
    // Redis is optional, return None if not configured
    if env::var("REDIS_URL").is_err() {
        return None;
    }
    
    // In production, actually test Redis connection
    // For now, return healthy as placeholder
    Some(HealthStatus::Healthy)
}

async fn check_job_scheduler() -> Option<HealthStatus> {
    // Job scheduler is optional, return None if not configured
    // In production, check scheduler health
    None
}

async fn check_metrics() -> Option<HealthStatus> {
    // Check if metrics endpoint is available
    // In production, actually make HTTP request
    Some(HealthStatus::Healthy)
}

fn determine_overall_status(
    db_status: &HealthStatus,
    redis_status: Option<&HealthStatus>,
    job_scheduler_status: Option<&HealthStatus>,
    metrics_status: Option<&HealthStatus>,
) -> HealthStatus {
    // If any critical dependency is unhealthy, overall is unhealthy
    if db_status == &HealthStatus::Unhealthy {
        return HealthStatus::Unhealthy;
    }
    
    // If any dependency is degraded, overall is degraded
    if db_status == &HealthStatus::Degraded 
        || redis_status == Some(&HealthStatus::Degraded)
        || job_scheduler_status == Some(&HealthStatus::Degraded)
        || metrics_status == Some(&HealthStatus::Degraded)
    {
        return HealthStatus::Degraded;
    }
    
    HealthStatus::Healthy
}

// Health check with additional metrics
#[derive(Debug, Clone, Serialize)]
pub struct DetailedHealthCheckResponse {
    pub status: HealthStatus,
    pub timestamp: String,
    pub uptime: String,
    pub database: DatabaseHealth,
    pub metrics: Option<MetricsHealth>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DatabaseHealth {
    pub status: HealthStatus,
    pub connection_string: String,
    pub pool_size: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetricsHealth {
    pub endpoint: String,
    pub status: HealthStatus,
    pub metrics_collected: bool,
    pub last_scrape: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MetricsSummary {
    pub total_requests: u64,
    pub request_duration_sum: f64,
    pub error_count: u64,
    pub active_connections: i64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub password_failures: u64,
    pub database_connections_active: u32,
    pub database_connections_idle: u32,
}

// Collect comprehensive metrics summary
pub async fn get_metrics_summary() -> Response {
    let metrics = crate::common::metrics::collect_metrics();
    
    let summary = MetricsSummary {
        total_requests: metrics.http_requests_total,
        request_duration_sum: metrics.http_request_duration_seconds_sum,
        error_count: metrics.error_count,
        active_connections: metrics.active_connections,
        cache_hits: metrics.cache_hits,
        cache_misses: metrics.cache_misses,
        password_failures: metrics.password_failures,
        database_connections_active: metrics.database_connections_active,
        database_connections_idle: metrics.database_connections_idle,
    };
    
    Json(summary).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_status_determination() {
        let db_status = HealthStatus::Healthy;
        let redis_status = Some(HealthStatus::Healthy);
        let job_scheduler_status = Some(HealthStatus::Healthy);
        let metrics_status = Some(HealthStatus::Healthy);
        
        let overall = determine_overall_status(
            &db_status, 
            redis_status.as_ref(), 
            job_scheduler_status.as_ref(),
            metrics_status.as_ref(),
        );
        
        assert_eq!(overall, HealthStatus::Healthy);
    }

    #[test]
    fn test_health_status_with_degraded_database() {
        let db_status = HealthStatus::Degraded;
        let redis_status = Some(HealthStatus::Healthy);
        let job_scheduler_status = Some(HealthStatus::Healthy);
        let metrics_status = Some(HealthStatus::Healthy);
        
        let overall = determine_overall_status(
            &db_status, 
            redis_status.as_ref(), 
            job_scheduler_status.as_ref(),
            metrics_status.as_ref(),
        );
        
        assert_eq!(overall, HealthStatus::Degraded);
    }

    #[test]
    fn test_health_status_with_unhealthy_database() {
        let db_status = HealthStatus::Unhealthy;
        let redis_status = Some(HealthStatus::Healthy);
        let job_scheduler_status = Some(HealthStatus::Healthy);
        let metrics_status = Some(HealthStatus::Healthy);
        
        let overall = determine_overall_status(
            &db_status, 
            redis_status.as_ref(), 
            job_scheduler_status.as_ref(),
            metrics_status.as_ref(),
        );
        
        assert_eq!(overall, HealthStatus::Unhealthy);
    }
}
