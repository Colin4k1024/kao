use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Password expiration configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordExpiration {
    /// Number of days after which password expires (default: 90)
    pub expiry_days: i64,
    /// Grace period in days before expiration (default: 7)
    pub grace_period_days: i64,
    /// Maximum age in days (default: 180)
    pub max_age_days: i64,
}

impl Default for PasswordExpiration {
    fn default() -> Self {
        Self {
            expiry_days: 90,
            grace_period_days: 7,
            max_age_days: 180,
        }
    }
}

/// Check if password is expired based on last change date
pub fn is_password_expired(last_changed: DateTime<Utc>, config: &PasswordExpiration) -> bool {
    let now = Utc::now();
    let days_since_change = days_since_last_change(last_changed, now);

    // Password is expired if it's past the expiry date + grace period
    days_since_change > config.expiry_days + config.grace_period_days
}

/// Calculate when the password will expire
pub fn password_expires_on(
    last_changed: DateTime<Utc>,
    config: &PasswordExpiration,
) -> DateTime<Utc> {
    last_changed + chrono::Duration::days(config.expiry_days)
}

/// Calculate days since last password change
pub fn days_since_last_change(last_changed: DateTime<Utc>, now: DateTime<Utc>) -> i64 {
    let duration = now - last_changed;
    duration.num_days()
}

/// Check password status and return warning level
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PasswordStatus {
    /// Password is valid with plenty of time remaining
    Valid { days_remaining: i64 },
    /// Password is in grace period, user should change soon
    ExpiringSoon { days_remaining: i64 },
    /// Password is expired, user must change
    Expired,
    /// Password is beyond max age, forced expiration
    ForceChange { days_over_max: i64 },
}

/// Check password status and return warning level
pub fn check_password_status(
    last_changed: DateTime<Utc>,
    now: DateTime<Utc>,
    config: &PasswordExpiration,
) -> PasswordStatus {
    let days_since_change = days_since_last_change(last_changed, now);
    let expires_on = password_expires_on(last_changed, config);
    let days_remaining = (expires_on - now).num_days();

    if days_since_change > config.max_age_days {
        PasswordStatus::ForceChange {
            days_over_max: days_since_change - config.max_age_days,
        }
    } else if days_remaining <= config.grace_period_days {
        PasswordStatus::Expired
    } else if days_remaining <= config.grace_period_days * 2 {
        PasswordStatus::ExpiringSoon { days_remaining }
    } else {
        PasswordStatus::Valid { days_remaining }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_not_expired() {
        let config = PasswordExpiration::default();
        let last_changed = Utc::now() - chrono::Duration::days(30);
        assert!(!is_password_expired(last_changed, &config));
    }

    #[test]
    fn test_password_expired() {
        let config = PasswordExpiration::default();
        let last_changed = Utc::now() - chrono::Duration::days(120);
        assert!(is_password_expired(last_changed, &config));
    }

    #[test]
    fn test_password_expires_on() {
        let config = PasswordExpiration::default();
        let last_changed = Utc::now();
        let expires = password_expires_on(last_changed, &config);
        let expected = last_changed + chrono::Duration::days(90);
        assert_eq!(expires, expected);
    }

    #[test]
    fn test_days_since_last_change() {
        let last_changed = Utc::now() - chrono::Duration::days(45);
        let now = Utc::now();
        assert_eq!(days_since_last_change(last_changed, now), 45);
    }

    #[test]
    fn test_password_status_valid() {
        let config = PasswordExpiration::default();
        let last_changed = Utc::now() - chrono::Duration::days(30);
        let now = Utc::now();

        match check_password_status(last_changed, now, &config) {
            PasswordStatus::Valid { days_remaining } => {
                // Allow for timing edge cases - should be around 60
                assert!(days_remaining >= 59 && days_remaining <= 60);
            }
            _ => panic!("Expected Valid status"),
        }
    }

    #[test]
    fn test_password_status_expired() {
        let config = PasswordExpiration::default();
        let last_changed = Utc::now() - chrono::Duration::days(120);
        let now = Utc::now();

        match check_password_status(last_changed, now, &config) {
            PasswordStatus::Expired => {}
            _ => panic!("Expected Expired status"),
        }
    }
}
