use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Security scan check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCheck {
    pub name: String,
    pub status: String,  // "pass", "fail", "warning"
    pub details: String,
}

/// Security scan summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanSummary {
    pub total_checks: usize,
    pub passed_checks: usize,
    pub failed_checks: usize,
    pub warning_checks: usize,
}

/// Overall security scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanResult {
    pub status: String,  // "healthy", "warning", "critical"
    pub checks: Vec<SecurityCheck>,
    pub summary: SecurityScanSummary,
}

/// Password health information for a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHealth {
    pub user_id: Uuid,
    pub username: String,
    pub status: String,  // "valid", "expiring_soon", "expired", "force_change"
    pub days_remaining: Option<i64>,
    pub expires_at: Option<String>,
}

/// Locked account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockedAccount {
    pub user_id: Uuid,
    pub username: String,
    pub locked_until: String,
    pub reason: Option<String>,
}

/// Failed login attempt record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailedLoginAttempt {
    pub user_id: Option<Uuid>,
    pub username: String,
    pub ip_address: String,
    pub attempt_count: i64,
    pub last_attempt: String,
}

/// Suspicious input pattern from audit log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuspiciousInput {
    pub id: Uuid,
    pub username: Option<String>,
    pub ip_address: String,
    pub event_type: String,
    pub details: serde_json::Value,
    pub created_at: String,
}

/// Permission denied event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDeniedEvent {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub ip_address: String,
    pub event_type: String,
    pub details: serde_json::Value,
    pub created_at: String,
}

/// Security event summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEventSummary {
    pub total_events: i64,
    pub permission_denied_count: i64,
    pub suspicious_input_count: i64,
    pub brute_force_attempts: i64,
}

/// Configuration item for security settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfigItem {
    pub key: String,
    pub value: String,
    pub config_type: String,
    pub status: i32,
}

/// Brute force detection result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BruteForceDetection {
    pub ip_address: String,
    pub attempt_count: i64,
    pub is_blocked: bool,
    pub blocked_until: Option<String>,
}

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtConfig {
    pub expiry_hours: i64,
    pub refresh_expiry_hours: i64,
}

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicyConfig {
    pub min_length: usize,
    pub expiry_days: i64,
    pub reuse_prevention: bool,
}
