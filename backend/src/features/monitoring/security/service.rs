use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use super::model::*;

/// Security scan service for monitoring security status
pub struct SecurityScanService {
    pool: PgPool,
}

impl SecurityScanService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Run full security scan across all categories
    pub async fn run_full_scan(&self) -> Result<SecurityScanResult, sqlx::Error> {
        let mut checks = Vec::new();

        // Run each category of scans
        checks.extend(self.scan_password_health().await?);
        checks.extend(self.scan_locked_accounts().await?);
        checks.extend(self.scan_failed_logins().await?);
        checks.extend(self.scan_brute_force().await?);
        checks.extend(self.scan_configuration().await?);
        checks.extend(self.scan_audit_events().await?);

        // Calculate summary
        let total_checks = checks.len();
        let passed_checks = checks.iter().filter(|c| c.status == "pass").count();
        let failed_checks = checks.iter().filter(|c| c.status == "fail").count();
        let warning_checks = checks.iter().filter(|c| c.status == "warning").count();

        // Determine overall status
        let status = if failed_checks > 0 {
            "critical"
        } else if warning_checks > 0 {
            "warning"
        } else {
            "healthy"
        };

        Ok(SecurityScanResult {
            status: status.to_string(),
            checks,
            summary: SecurityScanSummary {
                total_checks,
                passed_checks,
                failed_checks,
                warning_checks,
            },
        })
    }

    /// Scan password health across all users
    pub async fn scan_password_health(&self) -> Result<Vec<SecurityCheck>, sqlx::Error> {
        let mut checks = Vec::new();

        // Check for expired passwords
        let expired_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_user
            WHERE deleted_at IS NULL
            AND password_expires_at IS NOT NULL
            AND password_expires_at < NOW()
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if expired_count > 0 {
            checks.push(SecurityCheck {
                name: "password_expiration".to_string(),
                status: "fail".to_string(),
                details: format!("{} user(s) have expired passwords", expired_count),
            });
        } else {
            checks.push(SecurityCheck {
                name: "password_expiration".to_string(),
                status: "pass".to_string(),
                details: "No users have expired passwords".to_string(),
            });
        }

        // Check for passwords expiring soon (within 7 days)
        let expiring_soon_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_user
            WHERE deleted_at IS NULL
            AND password_expires_at IS NOT NULL
            AND password_expires_at > NOW()
            AND password_expires_at < NOW() + INTERVAL '7 days'
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if expiring_soon_count > 0 {
            checks.push(SecurityCheck {
                name: "password_expiring_soon".to_string(),
                status: "warning".to_string(),
                details: format!("{} user(s) passwords expiring within 7 days", expiring_soon_count),
            });
        } else {
            checks.push(SecurityCheck {
                name: "password_expiring_soon".to_string(),
                status: "pass".to_string(),
                details: "No passwords expiring soon".to_string(),
            });
        }

        // Check for users without password expiration set
        let no_expiry_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_user
            WHERE deleted_at IS NULL
            AND password_expires_at IS NULL
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if no_expiry_count > 0 {
            checks.push(SecurityCheck {
                name: "password_expiry_configured".to_string(),
                status: "warning".to_string(),
                details: format!("{} user(s) do not have password expiration configured", no_expiry_count),
            });
        } else {
            checks.push(SecurityCheck {
                name: "password_expiry_configured".to_string(),
                status: "pass".to_string(),
                details: "All users have password expiration configured".to_string(),
            });
        }

        Ok(checks)
    }

    /// Scan for locked accounts
    pub async fn scan_locked_accounts(&self) -> Result<Vec<SecurityCheck>, sqlx::Error> {
        let mut checks = Vec::new();

        // Check for currently locked accounts
        let locked_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_user
            WHERE deleted_at IS NULL
            AND locked_until IS NOT NULL
            AND locked_until > NOW()
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if locked_count > 0 {
            checks.push(SecurityCheck {
                name: "account_lockout".to_string(),
                status: "warning".to_string(),
                details: format!("{} account(s) currently locked", locked_count),
            });
        } else {
            checks.push(SecurityCheck {
                name: "account_lockout".to_string(),
                status: "pass".to_string(),
                details: "No accounts currently locked".to_string(),
            });
        }

        // Check if account lockout is properly configured
        let lockout_configured: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM sys_user
                WHERE deleted_at IS NULL
                AND failed_login_attempts >= 0
            )
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if lockout_configured {
            checks.push(SecurityCheck {
                name: "lockout_tracking".to_string(),
                status: "pass".to_string(),
                details: "Account lockout tracking is active".to_string(),
            });
        } else {
            checks.push(SecurityCheck {
                name: "lockout_tracking".to_string(),
                status: "warning".to_string(),
                details: "Account lockout tracking may not be properly configured".to_string(),
            });
        }

        Ok(checks)
    }

    /// Scan for failed login attempts
    pub async fn scan_failed_logins(&self) -> Result<Vec<SecurityCheck>, sqlx::Error> {
        let mut checks = Vec::new();

        // Check failed logins in the last hour
        let recent_failures: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_login_log
            WHERE status = 1
            AND login_time > NOW() - INTERVAL '1 hour'
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if recent_failures > 10 {
            checks.push(SecurityCheck {
                name: "failed_login_rate".to_string(),
                status: "warning".to_string(),
                details: format!("{} failed login attempts in the last hour", recent_failures),
            });
        } else if recent_failures > 0 {
            checks.push(SecurityCheck {
                name: "failed_login_rate".to_string(),
                status: "pass".to_string(),
                details: format!("{} failed login attempts in the last hour (normal)", recent_failures),
            });
        } else {
            checks.push(SecurityCheck {
                name: "failed_login_rate".to_string(),
                status: "pass".to_string(),
                details: "No failed login attempts in the last hour".to_string(),
            });
        }

        // Check for users with multiple recent failures
        let high_failure_users: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(DISTINCT username) FROM sys_login_log
            WHERE status = 1
            AND login_time > NOW() - INTERVAL '24 hours'
            GROUP BY username
            HAVING COUNT(*) >= 5
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if high_failure_users > 0 {
            checks.push(SecurityCheck {
                name: "high_failure_users".to_string(),
                status: "warning".to_string(),
                details: format!("{} user(s) with 5+ failed attempts in 24 hours", high_failure_users),
            });
        } else {
            checks.push(SecurityCheck {
                name: "high_failure_users".to_string(),
                status: "pass".to_string(),
                details: "No users with excessive failed login attempts".to_string(),
            });
        }

        Ok(checks)
    }

    /// Scan for brute force attempts
    pub async fn scan_brute_force(&self) -> Result<Vec<SecurityCheck>, sqlx::Error> {
        let mut checks = Vec::new();

        // Check for brute force patterns (same IP, many attempts)
        let brute_force_ips: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(DISTINCT ip_address) FROM sys_login_log
            WHERE status = 1
            AND login_time > NOW() - INTERVAL '1 hour'
            GROUP BY ip_address
            HAVING COUNT(*) >= 10
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if brute_force_ips > 0 {
            checks.push(SecurityCheck {
                name: "brute_force_detection".to_string(),
                status: "fail".to_string(),
                details: format!("Potential brute force from {} IP address(es)", brute_force_ips),
            });
        } else {
            checks.push(SecurityCheck {
                name: "brute_force_detection".to_string(),
                status: "pass".to_string(),
                details: "No brute force patterns detected".to_string(),
            });
        }

        // Check login log table exists and has recent data
        let login_log_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM sys_login_log LIMIT 1)",
        )
        .fetch_one(&self.pool)
        .await?;

        if login_log_exists {
            checks.push(SecurityCheck {
                name: "login_logging".to_string(),
                status: "pass".to_string(),
                details: "Login events are being logged".to_string(),
            });
        } else {
            checks.push(SecurityCheck {
                name: "login_logging".to_string(),
                status: "warning".to_string(),
                details: "No login log data available".to_string(),
            });
        }

        Ok(checks)
    }

    /// Scan security-related configuration
    pub async fn scan_configuration(&self) -> Result<Vec<SecurityCheck>, sqlx::Error> {
        let mut checks = Vec::new();

        // Check CORS configuration
        let cors_configured: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM sys_config
                WHERE config_key LIKE '%cors%'
                AND deleted_at IS NULL
            )
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if cors_configured {
            checks.push(SecurityCheck {
                name: "cors_configuration".to_string(),
                status: "pass".to_string(),
                details: "CORS settings are configured".to_string(),
            });
        } else {
            checks.push(SecurityCheck {
                name: "cors_configuration".to_string(),
                status: "warning".to_string(),
                details: "No CORS configuration found".to_string(),
            });
        }

        // Check JWT expiry configuration
        let jwt_configured: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM sys_config
                WHERE config_key LIKE '%jwt%expir%'
                AND deleted_at IS NULL
            )
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if jwt_configured {
            checks.push(SecurityCheck {
                name: "jwt_expiry".to_string(),
                status: "pass".to_string(),
                details: "JWT expiry is configured".to_string(),
            });
        } else {
            checks.push(SecurityCheck {
                name: "jwt_expiry".to_string(),
                status: "warning".to_string(),
                details: "JWT expiry configuration not found, using defaults".to_string(),
            });
        }

        // Check password policy configuration
        let password_policy_configured: bool = sqlx::query_scalar(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM sys_config
                WHERE config_key LIKE '%password%policy%'
                AND deleted_at IS NULL
            )
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if password_policy_configured {
            checks.push(SecurityCheck {
                name: "password_policy".to_string(),
                status: "pass".to_string(),
                details: "Password policy is configured".to_string(),
            });
        } else {
            checks.push(SecurityCheck {
                name: "password_policy".to_string(),
                status: "warning".to_string(),
                details: "Password policy configuration not found, using defaults".to_string(),
            });
        }

        // Check audit logging configuration
        let audit_log_exists: bool = sqlx::query_scalar(
            "SELECT EXISTS(SELECT 1 FROM sys_audit_log LIMIT 1)",
        )
        .fetch_one(&self.pool)
        .await?;

        if audit_log_exists {
            checks.push(SecurityCheck {
                name: "audit_logging".to_string(),
                status: "pass".to_string(),
                details: "Audit logging is active".to_string(),
            });
        } else {
            checks.push(SecurityCheck {
                name: "audit_logging".to_string(),
                status: "warning".to_string(),
                details: "No audit log data available yet".to_string(),
            });
        }

        Ok(checks)
    }

    /// Scan audit events for security issues
    pub async fn scan_audit_events(&self) -> Result<Vec<SecurityCheck>, sqlx::Error> {
        let mut checks = Vec::new();

        // Check for permission denied events
        let permission_denied_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_audit_log
            WHERE event_type = 'permission_denied'
            AND created_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if permission_denied_count > 10 {
            checks.push(SecurityCheck {
                name: "permission_denied_events".to_string(),
                status: "warning".to_string(),
                details: format!("{} permission denied events in 24 hours", permission_denied_count),
            });
        } else if permission_denied_count > 0 {
            checks.push(SecurityCheck {
                name: "permission_denied_events".to_string(),
                status: "pass".to_string(),
                details: format!("{} permission denied events in 24 hours (normal)", permission_denied_count),
            });
        } else {
            checks.push(SecurityCheck {
                name: "permission_denied_events".to_string(),
                status: "pass".to_string(),
                details: "No permission denied events in 24 hours".to_string(),
            });
        }

        // Check for suspicious input patterns (SQL injection, XSS attempts)
        let suspicious_input_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_audit_log
            WHERE event_type IN ('sql_injection_attempt', 'xss_attempt', 'invalid_input')
            AND created_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if suspicious_input_count > 0 {
            checks.push(SecurityCheck {
                name: "suspicious_input".to_string(),
                status: "warning".to_string(),
                details: format!("{} suspicious input attempt(s) detected in 24 hours", suspicious_input_count),
            });
        } else {
            checks.push(SecurityCheck {
                name: "suspicious_input".to_string(),
                status: "pass".to_string(),
                details: "No suspicious input patterns detected".to_string(),
            });
        }

        // Check for privilege escalation attempts
        let privilege_escalation_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_audit_log
            WHERE event_type IN ('privilege_escalation', 'unauthorized_access', 'role_change')
            AND created_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        if privilege_escalation_count > 0 {
            checks.push(SecurityCheck {
                name: "privilege_escalation".to_string(),
                status: "fail".to_string(),
                details: format!("{} privilege escalation attempt(s) detected", privilege_escalation_count),
            });
        } else {
            checks.push(SecurityCheck {
                name: "privilege_escalation".to_string(),
                status: "pass".to_string(),
                details: "No privilege escalation attempts detected".to_string(),
            });
        }

        Ok(checks)
    }

    /// Get password health for a specific user
    pub async fn get_password_health(&self, user_id: Uuid) -> Result<Option<PasswordHealth>, sqlx::Error> {
        let result = sqlx::query_as::<_, (
            Uuid,
            String,
            Option<DateTime<Utc>>,
            Option<DateTime<Utc>>,
            Option<i64>,
        )>(
            r#"
            SELECT id, username, last_password_changed_at, password_expires_at, password_expiry_days
            FROM sys_user
            WHERE id = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|(id, username, _last_changed, expires_at, _expiry_days)| {
            let now = Utc::now();
            let (status, days_remaining) = if let Some(expires) = expires_at {
                let days_left = (expires - now).num_days();
                if days_left < 0 {
                    ("expired".to_string(), Some(days_left))
                } else if days_left <= 7 {
                    ("expiring_soon".to_string(), Some(days_left))
                } else {
                    ("valid".to_string(), Some(days_left))
                }
            } else {
                ("force_change".to_string(), None)
            };

            PasswordHealth {
                user_id: id,
                username,
                status,
                days_remaining,
                expires_at: expires_at.map(|dt| dt.to_rfc3339()),
            }
        }))
    }

    /// Get all currently locked accounts
    pub async fn get_locked_accounts(&self) -> Result<Vec<LockedAccount>, sqlx::Error> {
        let results = sqlx::query_as::<_, (Uuid, String, DateTime<Utc>, Option<String>)>(
            r#"
            SELECT id, username, locked_until, lockout_reason
            FROM sys_user
            WHERE deleted_at IS NULL
            AND locked_until IS NOT NULL
            AND locked_until > NOW()
            ORDER BY locked_until DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|(id, username, locked_until, reason)| LockedAccount {
                user_id: id,
                username,
                locked_until: locked_until.to_rfc3339(),
                reason,
            })
            .collect())
    }

    /// Get failed login attempts summary
    pub async fn get_failed_login_attempts(&self, limit: i64) -> Result<Vec<FailedLoginAttempt>, sqlx::Error> {
        let results = sqlx::query_as::<_, (Option<Uuid>, String, String, i64, DateTime<Utc>)>(
            r#"
            SELECT user_id, username, ip_address, COUNT(*) as attempt_count, MAX(login_time) as last_attempt
            FROM sys_login_log
            WHERE status = 1
            AND login_time > NOW() - INTERVAL '24 hours'
            GROUP BY user_id, username, ip_address
            ORDER BY attempt_count DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|(user_id, username, ip, count, last)| FailedLoginAttempt {
                user_id,
                username,
                ip_address: ip,
                attempt_count: count,
                last_attempt: last.to_rfc3339(),
            })
            .collect())
    }

    /// Get suspicious input patterns from audit log
    pub async fn get_suspicious_inputs(&self, limit: i64) -> Result<Vec<SuspiciousInput>, sqlx::Error> {
        let results = sqlx::query_as::<_, (Uuid, Option<String>, String, String, serde_json::Value, DateTime<Utc>)>(
            r#"
            SELECT id, username, ip_address, event_type, details, created_at
            FROM sys_audit_log
            WHERE event_type IN ('sql_injection_attempt', 'xss_attempt', 'invalid_input', 'suspicious_input')
            ORDER BY created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|(id, username, ip, event_type, details, created)| SuspiciousInput {
                id,
                username,
                ip_address: ip,
                event_type,
                details,
                created_at: created.to_rfc3339(),
            })
            .collect())
    }

    /// Get permission denied events
    pub async fn get_permission_denied_events(&self, limit: i64) -> Result<Vec<PermissionDeniedEvent>, sqlx::Error> {
        let results = sqlx::query_as::<_, (Uuid, Option<Uuid>, Option<String>, String, String, serde_json::Value, DateTime<Utc>)>(
            r#"
            SELECT id, user_id, username, ip_address, event_type, details, created_at
            FROM sys_audit_log
            WHERE event_type = 'permission_denied'
            ORDER BY created_at DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|(id, user_id, username, ip, event_type, details, created)| PermissionDeniedEvent {
                id,
                user_id,
                username,
                ip_address: ip,
                event_type,
                details,
                created_at: created.to_rfc3339(),
            })
            .collect())
    }

    /// Get brute force detection results
    pub async fn get_brute_force_detection(&self) -> Result<Vec<BruteForceDetection>, sqlx::Error> {
        let results = sqlx::query_as::<_, (String, i64)>(
            r#"
            SELECT ip_address, COUNT(*) as attempt_count
            FROM sys_login_log
            WHERE status = 1
            AND login_time > NOW() - INTERVAL '1 hour'
            GROUP BY ip_address
            HAVING COUNT(*) >= 5
            ORDER BY attempt_count DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(results
            .into_iter()
            .map(|(ip, count)| BruteForceDetection {
                ip_address: ip,
                attempt_count: count,
                is_blocked: count >= 10,
                blocked_until: if count >= 10 {
                    Some((Utc::now() + Duration::minutes(15)).to_rfc3339())
                } else {
                    None
                },
            })
            .collect())
    }

    /// Get security event summary
    pub async fn get_security_event_summary(&self) -> Result<SecurityEventSummary, sqlx::Error> {
        let total_events: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sys_audit_log WHERE created_at > NOW() - INTERVAL '24 hours'",
        )
        .fetch_one(&self.pool)
        .await?;

        let permission_denied_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_audit_log
            WHERE event_type = 'permission_denied'
            AND created_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        let suspicious_input_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_audit_log
            WHERE event_type IN ('sql_injection_attempt', 'xss_attempt', 'invalid_input', 'suspicious_input')
            AND created_at > NOW() - INTERVAL '24 hours'
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        let brute_force_attempts: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM sys_login_log
            WHERE status = 1
            AND login_time > NOW() - INTERVAL '24 hours'
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(SecurityEventSummary {
            total_events,
            permission_denied_count,
            suspicious_input_count,
            brute_force_attempts,
        })
    }
}
