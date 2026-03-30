// Security scan module for monitoring
//
// Provides security scanning endpoints:
// - Full security scan
// - Configuration scan
// - Input validation scan
// - Authentication scan
// - Authorization scan
// - Security events
// - Password health per user

pub mod model;
pub mod routes;
pub mod service;

pub use model::*;
pub use routes::security_router;
pub use service::SecurityScanService;

// Re-export existing types for backward compatibility
use crate::common::response::ApiResponse;
use axum::{
    extract::{Path, State},
    response::Response,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use crate::AppState;

// ============================================================================
// Models (for backward compatibility)
// ============================================================================

/// Audit log entry from sys_audit_log table
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuditLog {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub event_type: String,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub details: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

/// Query parameters for security events (audit log) endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogQueryParams {
    #[serde(default)]
    pub event_type: Option<String>,
    #[serde(default)]
    pub user_id: Option<Uuid>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_page_size")]
    pub page_size: i64,
}

fn default_page() -> i64 {
    1
}

fn default_page_size() -> i64 {
    10
}

/// Paginated response for audit logs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogListResponse {
    pub list: Vec<AuditLog>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64,
}

/// Password health response for a specific user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordHealthResponse {
    pub user_id: Uuid,
    pub username: String,
    pub password_expires_at: Option<DateTime<Utc>>,
    pub days_until_expiration: Option<i64>,
    pub is_expired: bool,
    pub is_locked: bool,
    pub locked_until: Option<DateTime<Utc>>,
    pub failed_login_attempts: i32,
}

/// Internal user record with password expiration fields
#[derive(Debug, FromRow)]
pub struct UserWithPasswordHealth {
    pub id: Uuid,
    pub username: String,
    pub password_expires_at: Option<DateTime<Utc>>,
    pub locked_until: Option<DateTime<Utc>>,
    pub failed_login_attempts: i32,
}

// ============================================================================
// Service (for backward compatibility)
// ============================================================================

pub struct SecurityService {
    pool: PgPool,
}

impl SecurityService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get paginated audit logs with optional filtering
    pub async fn get_audit_logs(
        &self,
        params: AuditLogQueryParams,
    ) -> Result<AuditLogListResponse, sqlx::Error> {
        let page = params.page.max(1);
        let page_size = params.page_size.clamp(1, 100);
        let offset = (page - 1) * page_size;

        // Build dynamic WHERE clause based on provided filters
        let mut conditions = Vec::new();
        let mut param_index = 1;

        if params.event_type.is_some() {
            conditions.push(format!("event_type = ${}", param_index));
            param_index += 1;
        }
        if params.user_id.is_some() {
            conditions.push(format!("user_id = ${}", param_index));
            param_index += 1;
        }
        if params.start_time.is_some() {
            conditions.push(format!("created_at >= ${}", param_index));
            param_index += 1;
        }
        if params.end_time.is_some() {
            conditions.push(format!("created_at <= ${}", param_index));
            param_index += 1;
        }

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        // Build and execute count query
        let count_query = format!(
            "SELECT COUNT(*) FROM sys_audit_log {}",
            where_clause
        );

        let mut count_builder = sqlx::query_scalar::<_, i64>(&count_query);
        if let Some(ref event_type) = params.event_type {
            count_builder = count_builder.bind(event_type);
        }
        if let Some(ref user_id) = params.user_id {
            count_builder = count_builder.bind(user_id);
        }
        if let Some(ref start_time) = params.start_time {
            count_builder = count_builder.bind(start_time);
        }
        if let Some(ref end_time) = params.end_time {
            count_builder = count_builder.bind(end_time);
        }
        let total = count_builder.fetch_one(&self.pool).await?;

        // Build and execute data query
        let data_query = format!(
            r#"SELECT id, user_id, username, event_type, ip_address, user_agent, details, created_at
               FROM sys_audit_log {}
               ORDER BY created_at DESC
               LIMIT ${} OFFSET ${}"#,
            where_clause,
            param_index,
            param_index + 1
        );

        let mut data_builder = sqlx::query_as::<_, AuditLog>(&data_query);
        if let Some(ref event_type) = params.event_type {
            data_builder = data_builder.bind(event_type);
        }
        if let Some(ref user_id) = params.user_id {
            data_builder = data_builder.bind(user_id);
        }
        if let Some(ref start_time) = params.start_time {
            data_builder = data_builder.bind(start_time);
        }
        if let Some(ref end_time) = params.end_time {
            data_builder = data_builder.bind(end_time);
        }
        data_builder = data_builder.bind(page_size).bind(offset);

        let logs = data_builder.fetch_all(&self.pool).await?;

        Ok(AuditLogListResponse {
            list: logs,
            total,
            page,
            page_size,
        })
    }

    /// Get password health for a specific user
    pub async fn get_password_health(
        &self,
        user_id: Uuid,
    ) -> Result<Option<PasswordHealthResponse>, sqlx::Error> {
        let user = sqlx::query_as::<_, UserWithPasswordHealth>(
            r#"
            SELECT id, username, password_expires_at, locked_until, failed_login_attempts
            FROM sys_user
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match user {
            Some(user) => {
                let now = Utc::now();

                // Calculate days until expiration
                let (days_until_expiration, is_expired) = match user.password_expires_at {
                    Some(expires_at) => {
                        let duration = expires_at.signed_duration_since(now);
                        let days = duration.num_days();
                        (Some(days), days < 0)
                    }
                    None => (None, false),
                };

                // Check if account is locked
                let is_locked = user
                    .locked_until
                    .map(|locked_until| locked_until > now)
                    .unwrap_or(false);

                Ok(Some(PasswordHealthResponse {
                    user_id: user.id,
                    username: user.username,
                    password_expires_at: user.password_expires_at,
                    days_until_expiration,
                    is_expired,
                    is_locked,
                    locked_until: user.locked_until,
                    failed_login_attempts: user.failed_login_attempts,
                }))
            }
            None => Ok(None),
        }
    }
}

// ============================================================================
// Controller (for backward compatibility)
// ============================================================================

pub struct SecurityController;

impl SecurityController {
    /// GET /api/monitoring/security/events
    pub async fn get_security_events_handler(
        State(state): State<AppState>,
        query: axum::extract::Query<AuditLogQueryParams>,
    ) -> Response {
        let service = SecurityService::new(state.pool.clone());

        match service.get_audit_logs(query.0).await {
            Ok(response) => ApiResponse::success(response),
            Err(e) => {
                log::error!("Failed to get security events: {}", e);
                ApiResponse::error(500, format!("Failed to get security events: {}", e))
            }
        }
    }

    /// GET /api/monitoring/security/password-health/:user_id
    pub async fn get_password_health_handler(
        State(state): State<AppState>,
        Path(user_id): Path<Uuid>,
    ) -> Response {
        let service = SecurityService::new(state.pool.clone());

        match service.get_password_health(user_id).await {
            Ok(Some(response)) => ApiResponse::success(response),
            Ok(None) => ApiResponse::error(404, "User not found".to_string()),
            Err(e) => {
                log::error!("Failed to get password health: {}", e);
                ApiResponse::error(500, format!("Failed to get password health: {}", e))
            }
        }
    }
}
