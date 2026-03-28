//! Account lockout mechanism to prevent brute force attacks.
//!
//! This module provides account lockout functionality that:
//! - Tracks failed login attempts per user
//! - Locks account after 5 consecutive failed attempts
//! - Auto-unlocks after 15 minutes
//! - Provides lockout status checking

use chrono::{DateTime, Duration, Utc};
use sqlx::PgPool;
use thiserror::Error;

/// Lockout errors
#[derive(Error, Debug)]
pub enum LockoutError {
    #[error("Account is locked until {0}")]
    AccountLocked(DateTime<Utc>),
    #[error("Account is locked: {0}")]
    AccountLockedWithReason(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

/// Account lockout configuration
#[derive(Debug, Clone)]
pub struct LockoutConfig {
    /// Maximum failed attempts before lockout
    pub max_attempts: i32,
    /// Lockout duration in minutes
    pub lockout_duration_mins: i64,
    /// Reason for lockout
    pub lockout_reason: String,
}

impl Default for LockoutConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            lockout_duration_mins: 15,
            lockout_reason: "Too many failed login attempts".to_string(),
        }
    }
}

/// Account lockout service
pub struct AccountLockoutService {
    config: LockoutConfig,
}

impl AccountLockoutService {
    pub fn new(config: LockoutConfig) -> Self {
        Self { config }
    }

    /// Check if an account is currently locked
    pub async fn is_locked(&self, pool: &PgPool, username: &str) -> Result<bool, LockoutError> {
        let result = sqlx::query_as::<_, (Option<DateTime<Utc>>,)>(
            r#"
            SELECT locked_until
            FROM sys_user
            WHERE username = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        if let Some((locked_until,)) = result {
            if let Some(locked) = locked_until {
                if locked > Utc::now() {
                    return Ok(true);
                }
                // Lock has expired, auto-unlock
                self.unlock(pool, username).await?;
                return Ok(false);
            }
        }

        Ok(false)
    }

    /// Get lockout status with details
    pub async fn get_lockout_status(&self, pool: &PgPool, username: &str) -> Result<LockoutStatus, LockoutError> {
        let result = sqlx::query_as::<_, (i32, Option<DateTime<Utc>>, Option<String>)>(
            r#"
            SELECT failed_login_attempts, locked_until, lockout_reason
            FROM sys_user
            WHERE username = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        match result {
            Some((attempts, locked_until, reason)) => {
                let is_locked = locked_until.map(|l| l > Utc::now()).unwrap_or(false);
                Ok(LockoutStatus {
                    username: username.to_string(),
                    failed_attempts: attempts,
                    is_locked,
                    locked_until,
                    lockout_reason: reason,
                })
            }
            None => Ok(LockoutStatus {
                username: username.to_string(),
                failed_attempts: 0,
                is_locked: false,
                locked_until: None,
                lockout_reason: None,
            }),
        }
    }

    /// Record a failed login attempt
    pub async fn record_failed_attempt(&self, pool: &PgPool, username: &str) -> Result<(), LockoutError> {
        // First get current attempts
        let current = sqlx::query_as::<_, (i32,)>(
            r#"
            SELECT COALESCE(failed_login_attempts, 0)
            FROM sys_user
            WHERE username = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(username)
        .fetch_optional(pool)
        .await?;

        let attempts = current.map(|(a,)| a).unwrap_or(0) + 1;

        // Check if we should lock
        if attempts >= self.config.max_attempts {
            let locked_until = Utc::now() + Duration::minutes(self.config.lockout_duration_mins);
            sqlx::query(
                r#"
                UPDATE sys_user
                SET failed_login_attempts = $1,
                    locked_until = $2,
                    lockout_reason = $3,
                    updated_at = NOW()
                WHERE username = $4 AND deleted_at IS NULL
                "#,
            )
            .bind(attempts)
            .bind(locked_until)
            .bind(&self.config.lockout_reason)
            .bind(username)
            .execute(pool)
            .await?;
        } else {
            sqlx::query(
                r#"
                UPDATE sys_user
                SET failed_login_attempts = $1,
                    updated_at = NOW()
                WHERE username = $2 AND deleted_at IS NULL
                "#,
            )
            .bind(attempts)
            .bind(username)
            .execute(pool)
            .await?;
        }

        Ok(())
    }

    /// Reset failed login attempts on successful login
    pub async fn reset_attempts(&self, pool: &PgPool, username: &str) -> Result<(), LockoutError> {
        sqlx::query(
            r#"
            UPDATE sys_user
            SET failed_login_attempts = 0,
                locked_until = NULL,
                lockout_reason = NULL,
                updated_at = NOW()
            WHERE username = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(username)
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Manually unlock an account
    pub async fn unlock(&self, pool: &PgPool, username: &str) -> Result<(), LockoutError> {
        sqlx::query(
            r#"
            UPDATE sys_user
            SET failed_login_attempts = 0,
                locked_until = NULL,
                lockout_reason = NULL,
                updated_at = NOW()
            WHERE username = $1 AND deleted_at IS NULL
            "#,
        )
        .bind(username)
        .execute(pool)
        .await?;

        Ok(())
    }
}

/// Lockout status information
#[derive(Debug, Clone)]
pub struct LockoutStatus {
    pub username: String,
    pub failed_attempts: i32,
    pub is_locked: bool,
    pub locked_until: Option<DateTime<Utc>>,
    pub lockout_reason: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lockout_config_defaults() {
        let config = LockoutConfig::default();
        assert_eq!(config.max_attempts, 5);
        assert_eq!(config.lockout_duration_mins, 15);
    }
}
