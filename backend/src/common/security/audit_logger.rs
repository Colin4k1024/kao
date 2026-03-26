use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use tracing::{info, warn};
use uuid::Uuid;

/// Audit log event types
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR")]
#[sqlx(rename_all = "snake_case")]
pub enum AuditLogEventType {
    LoginAttempt,
    LoginSuccess,
    LoginFailure,
    PasswordChange,
    PermissionDenied,
    UserCreated,
    UserUpdated,
    UserDeleted,
    PasswordResetRequested,
    PasswordResetCompleted,
    SecuritySettingsChanged,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub event_type: AuditLogEventType,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub details: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Audit logger for security events
pub struct AuditLogger {
    pool: PgPool,
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Log a security event
    pub async fn log_security_event(
        &self,
        event_type: AuditLogEventType,
        user_id: Option<Uuid>,
        username: Option<String>,
        ip_address: &str,
        user_agent: Option<&str>,
        details: serde_json::Value,
    ) -> Result<Uuid, AuditLoggerError> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query!(
            r#"
            INSERT INTO sys_audit_log (id, user_id, username, event_type, ip_address, user_agent, details, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            id,
            user_id,
            username,
            event_type.to_string(),
            ip_address,
            user_agent,
            details.to_string(),
            now
        )
        .execute(&self.pool)
        .await
        .map_err(|e| {
            warn!("Failed to log security event: {}", e);
            AuditLoggerError::DatabaseError(e.to_string())
        })?;

        info!(
            event_type = %event_type,
            user_id = ?user_id,
            username = ?username,
            ip_address = %ip_address,
            "Security event logged"
        );

        Ok(id)
    }

    /// Log a login attempt
    pub async fn log_login_attempt(
        &self,
        user_id: Option<Uuid>,
        username: Option<&str>,
        ip_address: &str,
        user_agent: Option<&str>,
        details: serde_json::Value,
    ) -> Result<Uuid, AuditLoggerError> {
        self.log_security_event(
            AuditLogEventType::LoginAttempt,
            user_id,
            username.map(|s| s.to_string()),
            ip_address,
            user_agent,
            details,
        )
        .await
    }

    /// Log a successful login
    pub async fn log_login_success(
        &self,
        user_id: Uuid,
        username: &str,
        ip_address: &str,
        user_agent: Option<&str>,
        details: serde_json::Value,
    ) -> Result<Uuid, AuditLoggerError> {
        self.log_security_event(
            AuditLogEventType::LoginSuccess,
            Some(user_id),
            Some(username.to_string()),
            ip_address,
            user_agent,
            details,
        )
        .await
    }

    /// Log a failed login attempt
    pub async fn log_login_failure(
        &self,
        username: &str,
        ip_address: &str,
        user_agent: Option<&str>,
        reason: &str,
    ) -> Result<Uuid, AuditLoggerError> {
        self.log_security_event(
            AuditLogEventType::LoginFailure,
            None,
            Some(username.to_string()),
            ip_address,
            user_agent,
            serde_json::json!({"reason": reason}),
        )
        .await
    }

    /// Log a password change event
    pub async fn log_password_change(
        &self,
        user_id: Uuid,
        username: &str,
        ip_address: &str,
        details: serde_json::Value,
    ) -> Result<Uuid, AuditLoggerError> {
        self.log_security_event(
            AuditLogEventType::PasswordChange,
            Some(user_id),
            Some(username.to_string()),
            ip_address,
            None,
            details,
        )
        .await
    }

    /// Log a permission denied event
    pub async fn log_permission_denied(
        &self,
        user_id: Uuid,
        username: &str,
        ip_address: &str,
        requested_resource: &str,
        required_permission: &str,
    ) -> Result<Uuid, AuditLoggerError> {
        self.log_security_event(
            AuditLogEventType::PermissionDenied,
            Some(user_id),
            Some(username.to_string()),
            ip_address,
            None,
            serde_json::json!({
                "requested_resource": requested_resource,
                "required_permission": required_permission
            }),
        )
        .await
    }

    /// Get recent audit logs for a user
    pub async fn get_user_audit_logs(
        &self,
        user_id: Uuid,
        limit: i64,
    ) -> Result<Vec<AuditLogEntry>, AuditLoggerError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, user_id, username, event_type, ip_address, user_agent, details, created_at
            FROM sys_audit_log
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            user_id,
            limit
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AuditLoggerError::DatabaseError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|row| AuditLogEntry {
                id: row.get(0),
                user_id: row.get(1),
                username: row.get(2),
                event_type: row.get(3),
                ip_address: row.get(4),
                user_agent: row.get(5),
                details: row.get(6),
                created_at: row.get(7),
            })
            .collect())
    }
}

/// Audit logger error
#[derive(Debug, thiserror::Error)]
pub enum AuditLoggerError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

impl std::fmt::Display for AuditLogEventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditLogEventType::LoginAttempt => write!(f, "login_attempt"),
            AuditLogEventType::LoginSuccess => write!(f, "login_success"),
            AuditLogEventType::LoginFailure => write!(f, "login_failure"),
            AuditLogEventType::PasswordChange => write!(f, "password_change"),
            AuditLogEventType::PermissionDenied => write!(f, "permission_denied"),
            AuditLogEventType::UserCreated => write!(f, "user_created"),
            AuditLogEventType::UserUpdated => write!(f, "user_updated"),
            AuditLogEventType::UserDeleted => write!(f, "user_deleted"),
            AuditLogEventType::PasswordResetRequested => write!(f, "password_reset_requested"),
            AuditLogEventType::PasswordResetCompleted => write!(f, "password_reset_completed"),
            AuditLogEventType::SecuritySettingsChanged => write!(f, "security_settings_changed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_log_event_type_display() {
        assert_eq!(
            format!("{}", AuditLogEventType::LoginSuccess),
            "login_success"
        );
        assert_eq!(
            format!("{}", AuditLogEventType::LoginFailure),
            "login_failure"
        );
        assert_eq!(
            format!("{}", AuditLogEventType::PasswordChange),
            "password_change"
        );
    }
}
