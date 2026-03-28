// Log aggregator module
//
// This module provides log aggregation functionality for sending
// logs to Elasticsearch and other log aggregation systems.

use std::collections::VecDeque;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::common::logging::formatter::{AuditLogEntry, SecurityEventLog, StructuredLog};

// Log aggregator configuration
#[derive(Debug, Clone)]
pub struct LogAggregatorConfig {
    pub enable_buffering: bool,
    pub buffer_size: usize,
    pub batch_size: usize,
    pub flush_interval_ms: u64,
    pub enable_elasticsearch: bool,
    pub elasticsearch_url: Option<String>,
    pub elasticsearch_user: Option<String>,
    pub elasticsearch_password: Option<String>,
    pub index_prefix: String,
    pub log_retention_days: u32,
}

impl Default for LogAggregatorConfig {
    fn default() -> Self {
        LogAggregatorConfig {
            enable_buffering: true,
            buffer_size: 1000,
            batch_size: 100,
            flush_interval_ms: 1000,
            enable_elasticsearch: false,
            elasticsearch_url: None,
            elasticsearch_user: None,
            elasticsearch_password: None,
            index_prefix: "logs".to_string(),
            log_retention_days: 30,
        }
    }
}

// Log aggregator state
#[derive(Debug, Clone)]
pub struct LogAggregatorState {
    pub buffered_logs: VecDeque<StructuredLog>,
    pub buffer_size: usize,
    pub batch_size: usize,
    pub flush_interval: Duration,
    pub elasticsearch_enabled: bool,
    pub elasticsearch_url: Option<String>,
}

impl LogAggregatorState {
    pub fn new(config: &LogAggregatorConfig) -> Self {
        LogAggregatorState {
            buffered_logs: VecDeque::new(),
            buffer_size: config.buffer_size,
            batch_size: config.batch_size,
            flush_interval: Duration::from_millis(config.flush_interval_ms),
            elasticsearch_enabled: config.enable_elasticsearch,
            elasticsearch_url: config.elasticsearch_url.clone(),
        }
    }

    pub fn is_buffer_full(&self) -> bool {
        self.buffered_logs.len() >= self.buffer_size
    }

    pub fn should_flush(&self, logs: &[StructuredLog]) -> bool {
        logs.len() >= self.batch_size
    }

    pub fn get_batch(&mut self) -> Vec<StructuredLog> {
        let mut batch = Vec::new();
        let count = self.buffered_logs.len().min(self.batch_size);
        for _ in 0..count {
            if let Some(log) = self.buffered_logs.pop_front() {
                batch.push(log);
            }
        }
        batch
    }

    pub fn add_log(&mut self, log: StructuredLog) {
        if self.buffered_logs.len() >= self.buffer_size {
            // Drop oldest log if buffer is full
            self.buffered_logs.pop_front();
        }
        self.buffered_logs.push_back(log);
    }
}

// Log aggregator
pub struct LogAggregator {
    state: LogAggregatorState,
    config: LogAggregatorConfig,
    #[allow(dead_code)]
    tx: Option<mpsc::Sender<StructuredLog>>,
}

impl LogAggregator {
    pub fn new(config: LogAggregatorConfig) -> Self {
        let state = LogAggregatorState::new(&config);

        LogAggregator {
            state,
            config,
            tx: None,
        }
    }

    pub fn with_buffering(mut self, enabled: bool) -> Self {
        self.config.enable_buffering = enabled;
        self
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.config.buffer_size = size;
        self
    }

    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.config.batch_size = size;
        self
    }

    pub fn with_elasticsearch(mut self, enabled: bool) -> Self {
        self.config.enable_elasticsearch = enabled;
        self
    }

    pub fn with_elasticsearch_url(mut self, url: impl Into<String>) -> Self {
        self.config.elasticsearch_url = Some(url.into());
        self
    }

    pub fn with_index_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.config.index_prefix = prefix.into();
        self
    }

    pub fn with_retention_days(mut self, days: u32) -> Self {
        self.config.log_retention_days = days;
        self
    }

    // Send log to aggregator
    pub fn send_log(&mut self, log: StructuredLog) -> Result<(), String> {
        if self.config.enable_buffering {
            self.state.add_log(log);
        } else {
            self.flush_logs(&[log])?;
        }
        Ok(())
    }

    // Send logs in batch
    pub fn send_logs(&mut self, logs: Vec<StructuredLog>) -> Result<(), String> {
        for log in logs {
            self.send_log(log)?;
        }
        Ok(())
    }

    // Flush buffered logs
    pub fn flush_logs(&mut self, logs: &[StructuredLog]) -> Result<(), String> {
        if logs.is_empty() {
            return Ok(());
        }

        // In production, send to Elasticsearch
        if self.config.enable_elasticsearch {
            match self.send_to_elasticsearch(logs) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Failed to send logs to Elasticsearch: {}", e);
                    // Continue with other logs
                }
            }
        }

        // Clear buffer if all logs were processed
        if self.config.enable_buffering {
            self.state.buffered_logs.clear();
        }

        Ok(())
    }

    // Send logs to Elasticsearch
    fn send_to_elasticsearch(&self, logs: &[StructuredLog]) -> Result<(), String> {
        let elasticsearch_url = self
            .config
            .elasticsearch_url
            .as_ref()
            .ok_or("Elasticsearch URL not configured")?;

        // In production, this would use reqwest to send logs to Elasticsearch
        // For now, just log the batch size
        println!(
            "Sending {} logs to Elasticsearch at {}",
            logs.len(),
            elasticsearch_url
        );

        Ok(())
    }

    // Create audit log entry
    pub fn create_audit_log(
        &mut self,
        user_id: impl Into<String>,
        username: impl Into<String>,
    ) -> AuditLogEntry {
        AuditLogEntry::new(user_id, username)
    }

    // Log security event
    pub fn log_security_event(&mut self, event: SecurityEventLog) -> Result<(), String> {
        let structured_log =
            StructuredLog::new(event.level, format!("Security event: {}", event.message))
                .with_user_id(event.user_id.unwrap_or_default())
                .with_metadata(
                    crate::common::logging::formatter::LogMetadata::new()
                        .with_ip_address(event.ip_address.unwrap_or_default())
                        .with_user_agent(event.user_agent.unwrap_or_default())
                        .with_method("SECURITY".to_string()),
                );

        self.send_log(structured_log)
    }

    // Export logs in batches
    pub fn export_logs(&mut self) -> Vec<StructuredLog> {
        self.state.get_batch()
    }

    // Get current buffer stats
    pub fn buffer_stats(&self) -> (usize, usize) {
        (self.state.buffered_logs.len(), self.state.buffer_size)
    }

    // Clear buffer
    pub fn clear_buffer(&mut self) {
        self.state.buffered_logs.clear();
    }
}

impl Default for LogAggregator {
    fn default() -> Self {
        LogAggregator::new(LogAggregatorConfig::default())
    }
}

// Create default aggregator
pub fn create_aggregator() -> LogAggregator {
    LogAggregator::new(LogAggregatorConfig::default())
}

// Create aggregator with elasticsearch
pub fn create_aggregator_with_es(es_url: impl Into<String>) -> LogAggregator {
    let config = LogAggregatorConfig {
        enable_elasticsearch: true,
        elasticsearch_url: Some(es_url.into()),
        ..Default::default()
    };
    LogAggregator::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_aggregator_basic() {
        let mut aggregator = LogAggregator::new(LogAggregatorConfig::default());

        let log = StructuredLog::new(
            crate::common::logging::formatter::LogLevel::Info,
            "Test log",
        );

        assert!(aggregator.send_log(log).is_ok());
    }

    #[test]
    fn test_buffer_stats() {
        let aggregator = LogAggregator::new(LogAggregatorConfig::default());

        let (current, max) = aggregator.buffer_stats();
        assert_eq!(current, 0);
        assert_eq!(max, 1000);
    }

    #[test]
    fn test_audit_log_creation() {
        let mut aggregator = LogAggregator::new(LogAggregatorConfig::default());

        let audit_log = aggregator.create_audit_log("user123", "John Doe");

        assert_eq!(audit_log.user_id, "user123");
        assert_eq!(audit_log.username, "John Doe");
    }
}
