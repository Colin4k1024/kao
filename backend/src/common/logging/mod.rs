// Common logging module
//
// This module provides logging infrastructure with structured logging,
// log aggregation, and various logging helpers.

pub mod aggregator;
pub mod formatter;

pub use aggregator::{
    create_aggregator, create_aggregator_with_es, LogAggregator, LogAggregatorConfig,
};
pub use formatter::{
    log_security_event, AuditLogEntry, LogField, LogLevel, LogMetadata, SecurityEventLog,
    StructuredLog,
};

// Logging configuration
#[derive(Debug, Clone)]
pub struct LoggingConfig {
    pub log_level: LogLevel,
    pub log_to_stdout: bool,
    pub enable_file_logging: bool,
    pub log_file_path: Option<String>,
    pub enable_json_format: bool,
    pub log_retention_days: u32,
}

impl Default for LoggingConfig {
    fn default() -> Self {
        LoggingConfig {
            log_level: LogLevel::Info,
            log_to_stdout: true,
            enable_file_logging: false,
            log_file_path: None,
            enable_json_format: false,
            log_retention_days: 30,
        }
    }
}

impl LoggingConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.log_level = level;
        self
    }

    pub fn enable_stdout(mut self) -> Self {
        self.log_to_stdout = true;
        self
    }

    pub fn disable_stdout(mut self) -> Self {
        self.log_to_stdout = false;
        self
    }

    pub fn enable_file_logging(mut self, path: impl Into<String>) -> Self {
        self.enable_file_logging = true;
        self.log_file_path = Some(path.into());
        self
    }

    pub fn enable_json_format(mut self) -> Self {
        self.enable_json_format = true;
        self
    }
}

// Global logger state
pub struct GlobalLoggerState {
    pub config: LoggingConfig,
    pub aggregator: LogAggregator,
}

impl GlobalLoggerState {
    pub fn new(config: LoggingConfig) -> Self {
        let aggregator = LogAggregator::new(LogAggregatorConfig::default());
        GlobalLoggerState { config, aggregator }
    }

    pub fn with_aggregator(mut self, aggregator: LogAggregator) -> Self {
        self.aggregator = aggregator;
        self
    }

    pub fn info(&mut self, message: impl Into<String>) {
        if self.config.log_level == LogLevel::Info || self.config.log_level == LogLevel::Debug {
            let log = StructuredLog::new(LogLevel::Info, message)
                .with_metadata(LogMetadata::new().with_method("INFO".to_string()));
            self.log(log);
        }
    }

    pub fn debug(&mut self, message: impl Into<String>) {
        if self.config.log_level == LogLevel::Debug {
            let log = StructuredLog::new(LogLevel::Debug, message)
                .with_metadata(LogMetadata::new().with_method("DEBUG".to_string()));
            self.log(log);
        }
    }

    pub fn warn(&mut self, message: impl Into<String>) {
        let log = StructuredLog::new(LogLevel::Warn, message)
            .with_metadata(LogMetadata::new().with_method("WARN".to_string()));
        self.log(log);
    }

    pub fn error(&mut self, message: impl Into<String>) {
        let log = StructuredLog::new(LogLevel::Error, message)
            .with_metadata(LogMetadata::new().with_method("ERROR".to_string()));
        self.log(log);
    }

    pub fn log(&mut self, log: StructuredLog) {
        if self.config.log_to_stdout {
            if self.config.enable_json_format {
                println!("{}", log.to_json());
            } else {
                println!("[{}] {}", log.level, log.message);
            }
        }

        // Log to aggregator
        let _ = self.aggregator.send_log(log);
    }
}

impl Default for GlobalLoggerState {
    fn default() -> Self {
        GlobalLoggerState::new(LoggingConfig::default())
    }
}

// Initialize global logger
pub fn init_logger() -> GlobalLoggerState {
    GlobalLoggerState::new(LoggingConfig::new())
}

// Initialize logger with level
pub fn init_logger_with_level(level: LogLevel) -> GlobalLoggerState {
    GlobalLoggerState::new(LoggingConfig::new().with_level(level))
}

// Initialize logger with elasticsearch
pub fn init_logger_with_es(es_url: impl Into<String>) -> GlobalLoggerState {
    let config = LogAggregatorConfig {
        enable_elasticsearch: true,
        elasticsearch_url: Some(es_url.into()),
        ..Default::default()
    };
    GlobalLoggerState::new(LoggingConfig::new()).with_aggregator(LogAggregator::new(config))
}

// Middleware for request logging
pub struct RequestLoggingMiddleware {
    enabled: bool,
}

impl RequestLoggingMiddleware {
    pub fn new(enabled: bool) -> Self {
        RequestLoggingMiddleware { enabled }
    }

    pub fn enable(mut self) -> Self {
        self.enabled = true;
        self
    }

    pub fn disable(mut self) -> Self {
        self.enabled = false;
        self
    }
}

impl Default for RequestLoggingMiddleware {
    fn default() -> Self {
        RequestLoggingMiddleware::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_initialization() {
        let logger = init_logger();
        assert!(logger.config.log_to_stdout);
    }

    #[test]
    fn test_logger_with_level() {
        let logger = init_logger_with_level(LogLevel::Debug);
        assert_eq!(logger.config.log_level, LogLevel::Debug);
    }
}
