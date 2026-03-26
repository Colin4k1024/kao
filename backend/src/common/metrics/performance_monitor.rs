// Performance monitoring module
//
// This module tracks performance metrics like query latency,
// request latency, cache hit/miss rate, and more.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use crate::common::metrics::{self, AlertManager, AlertRule};

// Query performance tracker
pub struct QueryPerformanceTracker {
    query_name: String,
    start_time: Instant,
    duration: Option<Duration>,
    error: Option<String>,
}

impl QueryPerformanceTracker {
    pub fn new(query_name: impl Into<String>) -> Self {
        QueryPerformanceTracker {
            query_name: query_name.into(),
            start_time: Instant::now(),
            duration: None,
            error: None,
        }
    }

    pub fn finish(mut self) -> Duration {
        let duration = self.start_time.elapsed();
        self.duration = Some(duration);
        metrics::record_db_query_time(&self.query_name, duration.as_secs_f64());
        duration
    }

    pub fn finish_with_error(mut self, error: impl Into<String>) -> Duration {
        let duration = self.start_time.elapsed();
        self.duration = Some(duration);
        self.error = Some(error.into());
        metrics::record_db_query_time(&self.query_name, duration.as_secs_f64());
        duration
    }
}

// High latency request tracker
pub struct HighLatencyRequestTracker {
    request_path: String,
    request_method: String,
    start_time: Instant,
    duration: Option<Duration>,
    error: Option<String>,
}

impl HighLatencyRequestTracker {
    pub fn new(path: impl Into<String>, method: impl Into<String>) -> Self {
        HighLatencyRequestTracker {
            request_path: path.into(),
            request_method: method.into(),
            start_time: Instant::now(),
            duration: None,
            error: None,
        }
    }

    pub fn finish(mut self) -> Duration {
        let duration = self.start_time.elapsed();
        self.duration = Some(duration);
        metrics::record_request_duration(
            &self.request_method,
            &self.request_path,
            200,
            duration.as_secs_f64(),
        );
        duration
    }

    pub fn finish_with_error(mut self, error: impl Into<String>) -> Duration {
        let duration = self.start_time.elapsed();
        self.duration = Some(duration);
        self.error = Some(error.into());
        metrics::record_request_duration(
            &self.request_method,
            &self.request_path,
            500,
            duration.as_secs_f64(),
        );
        duration
    }
}

// Cache performance tracker
pub struct CachePerformanceTracker {
    cache_key: String,
    hit: bool,
    duration: Option<Duration>,
}

impl CachePerformanceTracker {
    pub fn new(cache_key: impl Into<String>) -> Self {
        CachePerformanceTracker {
            cache_key: cache_key.into(),
            hit: false,
            duration: None,
        }
    }

    pub fn hit(mut self) -> Self {
        self.hit = true;
        metrics::record_cache_hit();
        self
    }

    pub fn miss(mut self) -> Self {
        self.hit = false;
        metrics::record_cache_miss();
        self
    }

    pub fn finish(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }
}

// Performance monitoring state
#[derive(Clone)]
pub struct PerformanceMonitorState {
    pub total_requests: u64,
    pub total_errors: u64,
    pub total_queries: u64,
    pub slow_queries: u64,
    pub slow_requests: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub alert_manager: AlertManager,
}

impl PerformanceMonitorState {
    pub fn new() -> Self {
        let mut alert_manager = AlertManager::new();
        alert_manager.add_rules(metrics::configure_alerts());
        PerformanceMonitorState {
            total_requests: 0,
            total_errors: 0,
            total_queries: 0,
            slow_queries: 0,
            slow_requests: 0,
            cache_hits: 0,
            cache_misses: 0,
            alert_manager,
        }
    }

    pub fn record_request(&mut self, success: bool, duration: Duration) {
        self.total_requests += 1;
        if !success {
            self.total_errors += 1;
        }
        if duration.as_millis() > 500 {
            self.slow_requests += 1;
        }
    }

    pub fn record_query(&mut self, success: bool, duration: Duration) {
        self.total_queries += 1;
        if !success {
            self.slow_queries += 1;
        }
        if duration.as_millis() > 100 {
            self.slow_queries += 1;
        }
    }

    pub fn record_cache_hit(&mut self) {
        self.cache_hits += 1;
        metrics::record_cache_hit();
    }

    pub fn record_cache_miss(&mut self) {
        self.cache_misses += 1;
        metrics::record_cache_miss();
    }

    pub fn get_cache_hit_rate(&self) -> f64 {
        let total = self.cache_hits + self.cache_misses;
        if total == 0 {
            return 1.0;
        }
        self.cache_hits as f64 / total as f64
    }

    pub fn get_error_rate(&self) -> f64 {
        if self.total_requests == 0 {
            return 0.0;
        }
        self.total_errors as f64 / self.total_requests as f64
    }

    pub fn check_thresholds(&mut self) {
        let metrics: HashMap<String, f64> = [
            ("http_error_rate".to_string(), self.get_error_rate()),
            ("http_request_duration_p95".to_string(), 0.5), // Mock p95
            ("database_connections_active".to_string(), 5.0), // Mock
            (
                "cache_miss_rate".to_string(),
                1.0 - self.get_cache_hit_rate(),
            ),
            ("password_validation_failures_total".to_string(), 0.0),
            ("audit_log_write_latency_p95".to_string(), 0.1),
            ("memory_usage_percent".to_string(), 0.7),
            ("cpu_usage_percent".to_string(), 0.5),
        ]
        .iter()
        .cloned()
        .collect();

        self.alert_manager.check_thresholds(&metrics);
    }

    pub fn get_active_alerts(&self) -> Vec<&crate::common::metrics::Alert> {
        self.alert_manager.get_active_alerts()
    }
}

impl Default for PerformanceMonitorState {
    fn default() -> Self {
        Self::new()
    }
}

// Thread-safe performance monitor
pub struct PerformanceMonitor {
    state: std::sync::Arc<std::sync::RwLock<PerformanceMonitorState>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        PerformanceMonitor {
            state: std::sync::Arc::new(std::sync::RwLock::new(PerformanceMonitorState::new())),
        }
    }

    pub fn record_request(&self, success: bool, duration: Duration) {
        let mut state = self.state.write().unwrap();
        state.record_request(success, duration);
    }

    pub fn record_query(&self, success: bool, duration: Duration) {
        let mut state = self.state.write().unwrap();
        state.record_query(success, duration);
    }

    pub fn record_cache_hit(&self) {
        let mut state = self.state.write().unwrap();
        state.record_cache_hit();
    }

    pub fn record_cache_miss(&self) -> f64 {
        let mut state = self.state.write().unwrap();
        state.record_cache_miss();
        state.get_cache_hit_rate()
    }

    pub fn record_cache_miss_rate(&self) -> f64 {
        let mut state = self.state.write().unwrap();
        state.record_cache_miss();
        1.0 - state.get_cache_hit_rate()
    }

    pub fn get_cache_hit_rate(&self) -> f64 {
        let state = self.state.read().unwrap();
        state.get_cache_hit_rate()
    }

    pub fn get_error_rate(&self) -> f64 {
        let state = self.state.read().unwrap();
        state.get_error_rate()
    }

    pub fn check_thresholds(&self) {
        let mut state = self.state.write().unwrap();
        state.check_thresholds();
    }

    pub fn get_active_alerts(&self) -> Vec<crate::common::metrics::Alert> {
        let state = self.state.read().unwrap();
        state.get_active_alerts().into_iter().cloned().collect()
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

// Track slow query
pub fn track_slow_query(query_name: impl Into<String>) -> QueryPerformanceTracker {
    QueryPerformanceTracker::new(query_name)
}

// Track high latency request
pub fn track_high_latency_request(
    path: impl Into<String>,
    method: impl Into<String>,
) -> HighLatencyRequestTracker {
    HighLatencyRequestTracker::new(path, method)
}

// Track cache performance
pub fn track_cache_performance(cache_key: impl Into<String>) -> CachePerformanceTracker {
    CachePerformanceTracker::new(cache_key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();
        monitor.record_request(true, Duration::from_millis(100));
        monitor.record_request(false, Duration::from_millis(200));
        assert_eq!(monitor.get_error_rate(), 0.5);
    }

    #[test]
    fn test_cache_hit_rate() {
        let monitor = PerformanceMonitor::new();
        monitor.record_cache_hit();
        monitor.record_cache_miss();
        assert!((monitor.get_cache_hit_rate() - 0.5).abs() < f64::EPSILON);
    }
}
