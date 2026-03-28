// Structured log formatter
//
// This module provides structured logging with common fields
// for log aggregation and analysis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Common log fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogField {
    pub key: String,
    pub value: String,
}

impl LogField {
    pub fn new(key: impl Into<String>, value: impl Into<String>) -> Self {
        LogField {
            key: key.into(),
            value: value.into(),
        }
    }
}

// Log metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMetadata {
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub path: Option<String>,
    pub method: Option<String>,
    pub trace_id: Option<String>,
    pub request_id: Option<String>,
    pub user_id: Option<String>,
    pub additional: HashMap<String, String>,
}

impl Default for LogMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl LogMetadata {
    pub fn new() -> Self {
        LogMetadata {
            ip_address: None,
            user_agent: None,
            path: None,
            method: None,
            trace_id: None,
            request_id: None,
            user_id: None,
            additional: HashMap::new(),
        }
    }

    pub fn with_ip_address(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    pub fn with_user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Some(agent.into());
        self
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }

    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn add_additional(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional.insert(key.into(), value.into());
        self
    }
}

// Log level
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "trace"),
            LogLevel::Debug => write!(f, "debug"),
            LogLevel::Info => write!(f, "info"),
            LogLevel::Warn => write!(f, "warn"),
            LogLevel::Error => write!(f, "error"),
        }
    }
}

// Structured log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredLog {
    pub timestamp: String,
    pub level: LogLevel,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LogMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<LogField>>,
}

impl StructuredLog {
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        StructuredLog {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level,
            message: message.into(),
            user_id: None,
            request_id: None,
            trace_id: None,
            metadata: None,
            stack_trace: None,
            fields: None,
        }
    }

    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }

    pub fn with_metadata(mut self, metadata: LogMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn with_stack_trace(mut self, stack_trace: impl Into<String>) -> Self {
        self.stack_trace = Some(stack_trace.into());
        self
    }

    pub fn with_field(mut self, field: LogField) -> Self {
        if self.fields.is_none() {
            self.fields = Some(Vec::new());
        }
        if let Some(ref mut fields) = self.fields {
            fields.push(field);
        }
        self
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "Failed to serialize log".to_string())
    }

    // Convert to Elasticsearch-compatible format
    pub fn to_elasticsearch_format(&self) -> serde_json::Value {
        let mut obj = serde_json::Map::new();

        obj.insert(
            "timestamp".to_string(),
            serde_json::Value::String(self.timestamp.clone()),
        );
        obj.insert(
            "level".to_string(),
            serde_json::Value::String(self.level.to_string()),
        );
        obj.insert(
            "message".to_string(),
            serde_json::Value::String(self.message.clone()),
        );

        if let Some(ref user_id) = self.user_id {
            obj.insert(
                "user_id".to_string(),
                serde_json::Value::String(user_id.clone()),
            );
        }

        if let Some(ref request_id) = self.request_id {
            obj.insert(
                "request_id".to_string(),
                serde_json::Value::String(request_id.clone()),
            );
        }

        if let Some(ref trace_id) = self.trace_id {
            obj.insert(
                "trace_id".to_string(),
                serde_json::Value::String(trace_id.clone()),
            );
        }

        if let Some(ref metadata) = self.metadata {
            let mut meta_obj = serde_json::Map::new();

            if let Some(ref ip) = metadata.ip_address {
                meta_obj.insert(
                    "ip_address".to_string(),
                    serde_json::Value::String(ip.clone()),
                );
            }
            if let Some(ref agent) = metadata.user_agent {
                meta_obj.insert(
                    "user_agent".to_string(),
                    serde_json::Value::String(agent.clone()),
                );
            }
            if let Some(ref path) = metadata.path {
                meta_obj.insert("path".to_string(), serde_json::Value::String(path.clone()));
            }
            if let Some(ref method) = metadata.method {
                meta_obj.insert(
                    "method".to_string(),
                    serde_json::Value::String(method.clone()),
                );
            }
            if let Some(ref trace) = metadata.trace_id {
                meta_obj.insert(
                    "trace_id".to_string(),
                    serde_json::Value::String(trace.clone()),
                );
            }
            if let Some(ref req_id) = metadata.request_id {
                meta_obj.insert(
                    "request_id".to_string(),
                    serde_json::Value::String(req_id.clone()),
                );
            }
            if let Some(ref user) = metadata.user_id {
                meta_obj.insert(
                    "user_id".to_string(),
                    serde_json::Value::String(user.clone()),
                );
            }

            // Add additional fields
            for (key, value) in &metadata.additional {
                meta_obj.insert(key.clone(), serde_json::Value::String(value.clone()));
            }

            obj.insert("metadata".to_string(), serde_json::Value::Object(meta_obj));
        }

        if let Some(ref stack) = self.stack_trace {
            obj.insert(
                "stack_trace".to_string(),
                serde_json::Value::String(stack.clone()),
            );
        }

        if let Some(ref fields) = self.fields {
            let mut fields_obj = serde_json::Map::new();
            for field in fields {
                fields_obj.insert(
                    field.key.clone(),
                    serde_json::Value::String(field.value.clone()),
                );
            }
            obj.insert("fields".to_string(), serde_json::Value::Object(fields_obj));
        }

        serde_json::Value::Object(obj)
    }
}

// Security event log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEventLog {
    pub timestamp: String,
    pub event_type: String,
    pub level: LogLevel,
    pub message: String,
    pub user_id: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: Option<serde_json::Value>,
    pub successful: bool,
    pub trace_id: Option<String>,
}

impl SecurityEventLog {
    pub fn new(event_type: impl Into<String>, message: impl Into<String>) -> Self {
        SecurityEventLog {
            timestamp: chrono::Utc::now().to_rfc3339(),
            event_type: event_type.into(),
            level: LogLevel::Info,
            message: message.into(),
            user_id: None,
            ip_address: None,
            user_agent: None,
            details: None,
            successful: true,
            trace_id: None,
        }
    }

    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    pub fn with_ip_address(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }

    pub fn with_user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Some(agent.into());
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    pub fn failed(mut self) -> Self {
        self.successful = false;
        self.level = LogLevel::Warn;
        self
    }

    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }
}

// Security event logger
pub fn log_security_event(
    event_type: impl Into<String>,
    message: impl Into<String>,
) -> SecurityEventLog {
    SecurityEventLog::new(event_type, message)
}

// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: String,
    pub log_id: String,
    pub user_id: String,
    pub username: String,
    pub event_type: String,
    pub ip_address: String,
    pub user_agent: String,
    pub details: serde_json::Value,
    pub created_at: String,
    pub trace_id: Option<String>,
}

impl AuditLogEntry {
    pub fn new(user_id: impl Into<String>, username: impl Into<String>) -> Self {
        AuditLogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            log_id: Uuid::new_v4().to_string(),
            user_id: user_id.into(),
            username: username.into(),
            event_type: "unknown".to_string(),
            ip_address: "unknown".to_string(),
            user_agent: "unknown".to_string(),
            details: serde_json::json!({}),
            created_at: chrono::Utc::now().to_rfc3339(),
            trace_id: None,
        }
    }

    pub fn with_event_type(mut self, event_type: impl Into<String>) -> Self {
        self.event_type = event_type.into();
        self
    }

    pub fn with_ip_address(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = ip.into();
        self
    }

    pub fn with_user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = agent.into();
        self
    }

    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }

    pub fn with_trace_id(mut self, trace_id: impl Into<String>) -> Self {
        self.trace_id = Some(trace_id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_structured_log_serialization() {
        let log = StructuredLog::new(LogLevel::Info, "Test message")
            .with_user_id("user123")
            .with_request_id("req456");

        let json = log.to_json();
        assert!(json.contains("\"level\":\"info\""));
        assert!(json.contains("\"user_id\":\"user123\""));
    }

    #[test]
    fn test_elasticsearch_format() {
        let log = StructuredLog::new(LogLevel::Error, "Test error").with_metadata(
            LogMetadata::new()
                .with_ip_address("192.168.1.1")
                .with_user_agent("Mozilla/5.0")
                .with_path("/api/test")
                .with_method("GET"),
        );

        let elasticsearch_format = log.to_elasticsearch_format();
        assert!(elasticsearch_format.is_object());
    }

    #[test]
    fn test_security_event_log() {
        let event = log_security_event("login", "User login attempt")
            .with_user_id("user123")
            .failed();

        assert!(!event.successful);
        assert_eq!(event.level, LogLevel::Warn);
    }
}
