// Prometheus metrics collection module
//
// This module provides comprehensive metrics collection using Prometheus patterns.
// It includes request counters, histograms for request duration, error counters,
// and various other performance metrics.

use once_cell::sync::Lazy;
use prometheus::{
    opts, register_histogram_vec, register_int_counter, register_int_gauge, HistogramVec,
    IntCounter, IntGauge,
};
use std::sync::Arc;
use tokio::sync::RwLock;

// Request counter for total requests
pub static REQUESTS_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(opts!(
        "http_requests_total",
        "Total number of HTTP requests"
    ))
    .expect("Failed to register http_requests_total counter")
});

// Request duration histogram (5 buckets)
pub static REQUEST_DURATION_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "http_request_duration_seconds",
        "Request duration in seconds",
        &["method", "path", "status"]
    )
    .expect("Failed to register http_request_duration_seconds histogram")
});

// Error counter for different status codes
pub static ERROR_COUNTER: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(opts!(
        "http_errors_total",
        "Total number of HTTP errors by status code"
    ))
    .expect("Failed to register http_errors_total counter")
});

// Active connections gauge
pub static ACTIVE_CONNECTIONS: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!(opts!(
        "active_connections",
        "Number of active HTTP connections"
    ))
    .expect("Failed to register active_connections gauge")
});

// Database query time histogram
pub static DB_QUERY_DURATION_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "db_query_duration_seconds",
        "Database query duration in seconds",
        &["query_name"]
    )
    .expect("Failed to register db_query_duration_seconds histogram")
});

// Cache hit/miss counter
pub static CACHE_OPERATIONS_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(opts!(
        "cache_operations_total",
        "Total number of cache operations (hits/misses)"
    ))
    .expect("Failed to register cache_operations_total counter")
});

// Password validation failures counter
pub static PASSWORD_VALIDATION_FAILURES: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!(opts!(
        "password_validation_failures_total",
        "Total number of password validation failures"
    ))
    .expect("Failed to register password_validation_failures_total counter")
});

// Audit log write latency histogram
pub static AUDIT_LOG_WRITE_LATENCY_SECONDS: Lazy<HistogramVec> = Lazy::new(|| {
    register_histogram_vec!(
        "audit_log_write_latency_seconds",
        "Audit log write latency in seconds",
        &["log_type"]
    )
    .expect("Failed to register audit_log_write_latency_seconds histogram")
});

// Metrics state for runtime updates
pub struct MetricsState {
    pub request_counter: IntCounter,
    pub request_duration: HistogramVec,
    pub error_counter: IntCounter,
    pub active_connections: IntGauge,
    pub db_query_duration: HistogramVec,
    pub cache_operations: IntCounter,
    pub password_validation_failures: IntCounter,
    pub audit_log_latency: HistogramVec,
}

impl Default for MetricsState {
    fn default() -> Self {
        Self {
            request_counter: REQUESTS_TOTAL.clone(),
            request_duration: REQUEST_DURATION_SECONDS.clone(),
            error_counter: ERROR_COUNTER.clone(),
            active_connections: ACTIVE_CONNECTIONS.clone(),
            db_query_duration: DB_QUERY_DURATION_SECONDS.clone(),
            cache_operations: CACHE_OPERATIONS_TOTAL.clone(),
            password_validation_failures: PASSWORD_VALIDATION_FAILURES.clone(),
            audit_log_latency: AUDIT_LOG_WRITE_LATENCY_SECONDS.clone(),
        }
    }
}

// Thread-safe metrics state
pub type MetricsStateArc = Arc<RwLock<MetricsState>>;

// Initialize metrics and return state
pub fn init_metrics() -> MetricsStateArc {
    Arc::new(RwLock::new(MetricsState::default()))
}

// Increment request counter
pub fn record_request(method: &str, path: &str, status: u16) {
    REQUESTS_TOTAL.inc();

    let status_str = status.to_string();
    let duration = request_duration_from_status(status);

    REQUEST_DURATION_SECONDS
        .get_metric_with(&[method, path, &status_str])
        .observe(duration);
}

// Record request duration
pub fn record_request_duration(method: &str, path: &str, status: u16, duration: f64) {
    let status_str = status.to_string();
    REQUEST_DURATION_SECONDS
        .get_metric_with(&[method, path, &status_str])
        .observe(duration);
}

// Helper to estimate duration from status (for when we don't track timing)
fn request_duration_from_status(_status: u16) -> f64 {
    // Default assumed duration for unmeasured requests
    0.01
}

// Record error
pub fn record_error(status_code: u16) {
    let status_str = status_code.to_string();
    ERROR_COUNTER.with_label_values(&[&status_str]).inc();
}

// Record database query time
pub fn record_db_query_time(query_name: &str, duration: f64) {
    DB_QUERY_DURATION_SECONDS
        .get_metric_with(&[query_name])
        .observe(duration);
}

// Record cache hit
pub fn record_cache_hit() {
    CACHE_OPERATIONS_TOTAL.with_label_values(&["hit"]).inc();
}

// Record cache miss
pub fn record_cache_miss() {
    CACHE_OPERATIONS_TOTAL.with_label_values(&["miss"]).inc();
}

// Record password validation failure
pub fn record_password_failure() {
    PASSWORD_VALIDATION_FAILURES.inc();
}

// Record audit log write latency
pub fn record_audit_log_latency(log_type: &str, duration: f64) {
    AUDIT_LOG_WRITE_LATENCY_SECONDS
        .get_metric_with(&[log_type])
        .observe(duration);
}

// Get current active connections count
pub fn get_active_connections() -> i64 {
    ACTIVE_CONNECTIONS.get()
}

// Set active connections count
pub fn set_active_connections(count: i64) {
    ACTIVE_CONNECTIONS.set(count);
}

// Increment active connections
pub fn increment_active_connections() {
    ACTIVE_CONNECTIONS.inc();
}

// Decrement active connections
pub fn decrement_active_connections() {
    ACTIVE_CONNECTIONS.dec();
}

// Export metrics in Prometheus format
pub fn export_metrics() -> String {
    prometheus::gather()
        .iter()
        .map(|metric| metric.proto.clone())
        .fold(String::new(), |mut acc, mut metric| {
            acc.push_str(&format!("{}\n", metric.name));
            for field in metric.field {
                acc.push_str(&format!("  {:?}\n", field));
            }
            acc
        })
}

// Metrics response for /metrics endpoint
#[derive(Clone)]
pub struct MetricsResponse {
    pub http_requests_total: u64,
    pub http_request_duration_seconds_sum: f64,
    pub http_request_duration_seconds_bucket: Vec<(String, u64)>,
    pub database_connections_active: u32,
    pub database_connections_idle: u32,
    pub database_connections_total: u32,
    pub error_count: u64,
    pub active_connections: i64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub password_failures: u64,
    pub timestamp: String,
}

impl MetricsResponse {
    pub fn new() -> Self {
        Self {
            http_requests_total: REQUESTS_TOTAL.get(),
            http_request_duration_seconds_sum: REQUEST_DURATION_SECONDS
                .get_metric_with(&["GET", "/api/health", "200"])
                .map(|h| h.get_sum())
                .unwrap_or(0.0),
            http_request_duration_seconds_bucket: vec![
                ("0.005".to_string(), 0),
                ("0.01".to_string(), 0),
                ("0.025".to_string(), 0),
                ("0.05".to_string(), 0),
                ("0.1".to_string(), 0),
                ("0.25".to_string(), 0),
                ("0.5".to_string(), 0),
                ("1.0".to_string(), 0),
            ],
            database_connections_active: 0,
            database_connections_idle: 0,
            database_connections_total: 0,
            error_count: ERROR_COUNTER.get(),
            active_connections: ACTIVE_CONNECTIONS.get(),
            cache_hits: 0,
            cache_misses: 0,
            password_failures: PASSWORD_VALIDATION_FAILURES.get(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

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
            output.push_str(&format!(
                "http_request_duration_seconds_bucket{{le=\"{}\"}} {}\n",
                le, count
            ));
        }

        // Error count
        output.push_str(&format!(
            "# HELP http_errors_total Total number of HTTP errors\n# TYPE http_errors_total counter\nhttp_errors_total {}\n",
            self.error_count
        ));

        // Active connections
        output.push_str(&format!(
            "# HELP active_connections Number of active HTTP connections\n# TYPE active_connections gauge\nactive_connections {}\n",
            self.active_connections
        ));

        // Cache operations
        output.push_str(&format!(
            "# HELP cache_operations_total Total cache operations\n# TYPE cache_operations_total counter\ncache_operations_total {}\n",
            self.cache_hits + self.cache_misses
        ));

        // Password validation failures
        output.push_str(&format!(
            "# HELP password_validation_failures_total Password validation failures\n# TYPE password_validation_failures_total counter\npassword_validation_failures_total {}\n",
            self.password_failures
        ));

        // Timestamp
        output.push_str(&format!(
            "# HELP metrics_timestamp Last metrics collection timestamp\n# TYPE metrics_timestamp gauge\nmetrics_timestamp {}\n",
            self.timestamp
        ));

        output
    }
}

// Collect all metrics
pub fn collect_metrics() -> MetricsResponse {
    MetricsResponse::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_counter() {
        record_request("GET", "/api/test", 200);
        assert!(REQUESTS_TOTAL.get() > 0);
    }

    #[test]
    fn test_error_counter() {
        record_error(500);
        assert!(ERROR_COUNTER.get() > 0);
    }

    #[test]
    fn test_cache_operations() {
        record_cache_hit();
        record_cache_miss();
        assert!(CACHE_OPERATIONS_TOTAL.get() > 0);
    }
}
