use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use std::time::Duration;

// Re-export common metrics types
pub use crate::common::metrics::{
    collect_metrics as common_collect_metrics, MetricsResponse as CommonMetricsResponse,
    MetricsMiddleware, AlertManager, AlertRule, AlertSeverity, AlertOperator,
    configure_alerts as common_configure_alerts,
};

// Metrics response for the old API
#[derive(Debug, Clone, Serialize)]
pub struct MetricsResponse {
    pub http_requests_total: u64,
    pub http_request_duration_seconds_sum: f64,
    pub http_request_duration_seconds_bucket: Vec<(String, u64)>,
    pub database_connections_active: u32,
    pub database_connections_idle: u32,
    pub database_connections_total: u32,
    pub cpu_usage_percent: f64,
    pub memory_used_bytes: u64,
    pub memory_total_bytes: u64,
    pub timestamp: String,
}

impl MetricsResponse {
    pub fn to_prometheus_format(&self) -> String {
        let mut output = String::new();

        // HTTP requests total
        output.push_str(&format!(
            "# HELP http_requests_total Total number of HTTP requests\n# TYPE http_requests_total counter\nhttp_requests_total {}\n",
            self.http_requests_total
        ));

        // HTTP request duration sum
        output.push_str(&format!(
            "# HELP http_request_duration_seconds_sum Total duration of HTTP requests in seconds\n# TYPE http_request_duration_seconds_sum gauge\nhttp_request_duration_seconds_sum {:.3}\n",
            self.http_request_duration_seconds_sum
        ));

        // HTTP request duration buckets
        output.push_str("# HELP http_request_duration_seconds_bucket HTTP request duration histogram\n# TYPE http_request_duration_seconds_bucket histogram\n");
        for (le, count) in &self.http_request_duration_seconds_bucket {
            output.push_str(&format!("http_request_duration_seconds_bucket{{le=\"{}\"}} {}\n", le, count));
        }

        // Database connections
        output.push_str(&format!(
            "# HELP database_connections_active Active database connections\n# TYPE database_connections_active gauge\ndatabase_connections_active {}\n",
            self.database_connections_active
        ));
        output.push_str(&format!(
            "# HELP database_connections_idle Idle database connections\n# TYPE database_connections_idle gauge\ndatabase_connections_idle {}\n",
            self.database_connections_idle
        ));
        output.push_str(&format!(
            "# HELP database_connections_total Total database connections\n# TYPE database_connections_total gauge\ndatabase_connections_total {}\n",
            self.database_connections_total
        ));

        // CPU usage
        output.push_str(&format!(
            "# HELP cpu_usage_percent Current CPU usage percentage\n# TYPE cpu_usage_percent gauge\ncpu_usage_percent {:.1}\n",
            self.cpu_usage_percent
        ));

        // Memory usage
        output.push_str(&format!(
            "# HELP memory_used_bytes Current memory usage in bytes\n# TYPE memory_used_bytes gauge\nmemory_used_bytes {}\n",
            self.memory_used_bytes
        ));
        output.push_str(&format!(
            "# HELP memory_total_bytes Total memory in bytes\n# TYPE memory_total_bytes gauge\nmemory_total_bytes {}\n",
            self.memory_total_bytes
        ));

        output
    }
}

pub fn collect_metrics() -> MetricsResponse {
    let now = chrono::Utc::now().to_rfc3339();

    // Mock metrics for demonstration
    // In production, these would be collected from actual sources:
    // - HTTP requests: use metrics crate or custom middleware
    // - Database connections: use sqlx::Pool::state()
    // - CPU/memory: use sysinfo crate
    
    MetricsResponse {
        http_requests_total: 12345,
        http_request_duration_seconds_sum: 123.456,
        http_request_duration_seconds_bucket: vec![
            ("0.005".to_string(), 10000),
            ("0.01".to_string(), 11000),
            ("0.025".to_string(), 11500),
            ("0.05".to_string(), 11800),
            ("0.1".to_string(), 12000),
            ("0.25".to_string(), 12200),
            ("0.5".to_string(), 12300),
            ("1.0".to_string(), 12340),
        ],
        database_connections_active: 5,
        database_connections_idle: 15,
        database_connections_total: 20,
        cpu_usage_percent: 25.5,
        memory_used_bytes: 536870912, // 512MB
        memory_total_bytes: 1073741824, // 1GB
        timestamp: now,
    }
}

pub async fn get_metrics() -> Response {
    let metrics = collect_metrics();
    let prometheus_format = metrics.to_prometheus_format();

    (
        StatusCode::OK,
        [("content-type", "text/plain; charset=utf-8")],
        prometheus_format,
    )
        .into_response()
}
