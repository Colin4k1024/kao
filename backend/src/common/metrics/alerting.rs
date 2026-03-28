// Alerting rules configuration
//
// This module provides alerting rules for monitoring system metrics
// and triggering alerts when thresholds are breached.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// Alert severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlertSeverity {
    Critical,
    Warning,
    Info,
}

impl std::fmt::Display for AlertSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertSeverity::Critical => write!(f, "CRITICAL"),
            AlertSeverity::Warning => write!(f, "WARNING"),
            AlertSeverity::Info => write!(f, "INFO"),
        }
    }
}

// Alert status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AlertStatus {
    Pending,
    Firing,
    Resolved,
}

// Single alert rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: AlertSeverity,
    pub threshold: f64,
    pub window: Duration, // Time window for evaluation
    pub cooldown: Duration, // Cooldown after alert fires
    pub metric_name: String,
    pub operator: AlertOperator,
    pub enabled: bool,
}

// Alert operator for threshold comparison
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AlertOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

impl AlertOperator {
    pub fn evaluate(&self, value: f64, threshold: f64) -> bool {
        match self {
            AlertOperator::GreaterThan => value > threshold,
            AlertOperator::GreaterThanOrEqual => value >= threshold,
            AlertOperator::LessThan => value < threshold,
            AlertOperator::LessThanOrEqual => value <= threshold,
            AlertOperator::Equal => (value - threshold).abs() < f64::EPSILON,
            AlertOperator::NotEqual => (value - threshold).abs() >= f64::EPSILON,
        }
    }
}

// Alert instance (fired alert)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: String,
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub severity: AlertSeverity,
    pub value: f64,
    pub status: AlertStatus,
    pub started_at: chrono::NaiveDateTime,
    pub triggered_at: Option<chrono::NaiveDateTime>,
    pub resolved_at: Option<chrono::NaiveDateTime>,
    pub labels: HashMap<String, String>,
    pub annotations: HashMap<String, String>,
}

// Alert manager for managing rules and alerts
#[derive(Clone)]
pub struct AlertManager {
    rules: Vec<AlertRule>,
    active_alerts: Vec<Alert>,
    last_evaluation: Option<chrono::NaiveDateTime>,
}

impl AlertManager {
    pub fn new() -> Self {
        AlertManager {
            rules: Vec::new(),
            active_alerts: Vec::new(),
            last_evaluation: None,
        }
    }

    // Add a rule to the manager
    pub fn add_rule(&mut self, rule: AlertRule) {
        self.rules.push(rule);
    }

    // Add multiple rules
    pub fn add_rules(&mut self, rules: Vec<AlertRule>) {
        for rule in rules {
            self.add_rule(rule);
        }
    }

    // Check all rules against provided metric values
    pub fn check_thresholds(&mut self, metrics: &HashMap<String, f64>) {
        let now = chrono::Utc::now().naive_utc();

        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            // Get metric value
            let metric_value = match metrics.get(&rule.metric_name) {
                Some(&value) => value,
                None => continue, // Skip if metric not available
            };

            // Check if threshold is breached
            let threshold_breached = rule.operator.evaluate(metric_value, rule.threshold);

            // Check if rule is already firing
            let existing_alert = self
                .active_alerts
                .iter()
                .find(|a| a.rule_id == rule.id);

            match existing_alert {
                Some(alert) if alert.status == AlertStatus::Firing => {
                    // Alert is already firing, check cooldown
                    if let Some(triggered_at) = alert.triggered_at {
                        let elapsed = (now - triggered_at).to_std().unwrap_or(Duration::ZERO);
                        if elapsed < rule.cooldown {
                            continue; // Still in cooldown
                        }
                    }
                }
                None => {
                    // No existing alert, check if we should create one
                    if threshold_breached {
                        let new_alert = Alert {
                            id: format!("{}-{}", rule.id, now.and_utc().timestamp()),
                            rule_id: rule.id.clone(),
                            name: rule.name.clone(),
                            description: rule.description.clone(),
                            severity: rule.severity,
                            value: metric_value,
                            status: AlertStatus::Firing,
                            started_at: now,
                            triggered_at: Some(now),
                            resolved_at: None,
                            labels: HashMap::new(),
                            annotations: HashMap::new(),
                        };
                        self.active_alerts.push(new_alert);
                    }
                }
                _ => {}
            }
        }

        self.last_evaluation = Some(now);
    }

    // Resolve alerts that are no longer breached
    pub fn resolve_alerts(&mut self, metrics: &HashMap<String, f64>) {
        let now = chrono::Utc::now().naive_utc();

        for alert in &mut self.active_alerts {
            if alert.status == AlertStatus::Resolved {
                continue;
            }

            // Get rule
            let rule = match self.rules.iter().find(|r| r.id == alert.rule_id) {
                Some(r) => r,
                None => continue,
            };

            // Get metric value
            let metric_value = match metrics.get(&rule.metric_name) {
                Some(&value) => value,
                None => continue,
            };

            // Check if threshold is NOT breached
            let threshold_not_breached = !rule.operator.evaluate(metric_value, rule.threshold);

            if threshold_not_breached {
                alert.status = AlertStatus::Resolved;
                alert.resolved_at = Some(now);
            }
        }
    }

    // Get all active alerts
    pub fn get_active_alerts(&self) -> Vec<&Alert> {
        self.active_alerts
            .iter()
            .filter(|a| a.status == AlertStatus::Firing)
            .collect()
    }

    // Get alerts by severity
    pub fn get_alerts_by_severity(&self, severity: AlertSeverity) -> Vec<&Alert> {
        self.active_alerts
            .iter()
            .filter(|a| a.status == AlertStatus::Firing && a.severity == severity)
            .collect()
    }

    // Clear resolved alerts
    pub fn clear_resolved_alerts(&mut self) {
        self.active_alerts.retain(|a| a.status != AlertStatus::Resolved);
    }

    // Format alert as message
    pub fn format_alert(&self, alert: &Alert) -> String {
        format!(
            "[{}] {} - {} (value: {:.3}, threshold: {:.3})",
            alert.severity,
            alert.name,
            alert.description,
            alert.value,
            self.rules
                .iter()
                .find(|r| r.id == alert.rule_id)
                .map(|r| r.threshold)
                .unwrap_or(0.0),
        )
    }
}

impl Default for AlertManager {
    fn default() -> Self {
        Self::new()
    }
}

// Configure all alert rules
pub fn configure_alerts() -> Vec<AlertRule> {
    vec![
        AlertRule {
            id: "high_error_rate".to_string(),
            name: "High Error Rate".to_string(),
            description: "Error rate exceeds 5% in the last 5 minutes".to_string(),
            severity: AlertSeverity::Critical,
            threshold: 0.05,
            window: Duration::from_secs(300), // 5 minutes
            cooldown: Duration::from_secs(600), // 10 minutes
            metric_name: "http_error_rate".to_string(),
            operator: AlertOperator::GreaterThan,
            enabled: true,
        },
        AlertRule {
            id: "high_latency".to_string(),
            name: "High Latency".to_string(),
            description: "Response time exceeds 500ms for p95".to_string(),
            severity: AlertSeverity::Warning,
            threshold: 0.5, // 500ms
            window: Duration::from_secs(300), // 5 minutes
            cooldown: Duration::from_secs(600), // 10 minutes
            metric_name: "http_request_duration_p95".to_string(),
            operator: AlertOperator::GreaterThan,
            enabled: true,
        },
        AlertRule {
            id: "db_connection_pool_exhausted".to_string(),
            name: "Database Connection Pool Exhausted".to_string(),
            description: "Database connection pool is exhausted".to_string(),
            severity: AlertSeverity::Critical,
            threshold: 20.0,
            window: Duration::from_secs(60), // 1 minute
            cooldown: Duration::from_secs(300), // 5 minutes
            metric_name: "database_connections_active".to_string(),
            operator: AlertOperator::Equal,
            enabled: true,
        },
        AlertRule {
            id: "high_cache_miss_rate".to_string(),
            name: "High Cache Miss Rate".to_string(),
            description: "Cache miss rate exceeds 90%".to_string(),
            severity: AlertSeverity::Warning,
            threshold: 0.9,
            window: Duration::from_secs(300), // 5 minutes
            cooldown: Duration::from_secs(600), // 10 minutes
            metric_name: "cache_miss_rate".to_string(),
            operator: AlertOperator::GreaterThan,
            enabled: true,
        },
        AlertRule {
            id: "password_validation_failure_spike".to_string(),
            name: "Password Validation Failure Spike".to_string(),
            description: "Password validation failures detected".to_string(),
            severity: AlertSeverity::Warning,
            threshold: 100.0,
            window: Duration::from_secs(300), // 5 minutes
            cooldown: Duration::from_secs(300), // 5 minutes
            metric_name: "password_validation_failures_total".to_string(),
            operator: AlertOperator::GreaterThan,
            enabled: true,
        },
        AlertRule {
            id: "audit_log_write_failure".to_string(),
            name: "Audit Log Write Failure".to_string(),
            description: "Audit log write latency exceeds threshold".to_string(),
            severity: AlertSeverity::Info,
            threshold: 1.0, // 1 second
            window: Duration::from_secs(60), // 1 minute
            cooldown: Duration::from_secs(300), // 5 minutes
            metric_name: "audit_log_write_latency_p95".to_string(),
            operator: AlertOperator::GreaterThan,
            enabled: true,
        },
        AlertRule {
            id: "high_memory_usage".to_string(),
            name: "High Memory Usage".to_string(),
            description: "Memory usage exceeds 80%".to_string(),
            severity: AlertSeverity::Warning,
            threshold: 0.8,
            window: Duration::from_secs(60), // 1 minute
            cooldown: Duration::from_secs(300), // 5 minutes
            metric_name: "memory_usage_percent".to_string(),
            operator: AlertOperator::GreaterThan,
            enabled: true,
        },
        AlertRule {
            id: "high_cpu_usage".to_string(),
            name: "High CPU Usage".to_string(),
            description: "CPU usage exceeds 90%".to_string(),
            severity: AlertSeverity::Critical,
            threshold: 0.9,
            window: Duration::from_secs(60), // 1 minute
            cooldown: Duration::from_secs(300), // 5 minutes
            metric_name: "cpu_usage_percent".to_string(),
            operator: AlertOperator::GreaterThan,
            enabled: true,
        },
    ]
}

// Trigger alert via webhook
pub async fn trigger_alert(_alert: &Alert, _webhook_url: &str) -> Result<(), String> {
    // In production, this would make a POST request to the webhook URL
    // For now, just log the alert
    println!("Alert triggered: {}", _alert.name);
    println!("  Severity: {}", _alert.severity);
    println!("  Value: {}", _alert.value);
    println!("  Threshold: {}", _alert.value);
    println!("  URL: {}", _webhook_url);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_operator_evaluate() {
        assert!(AlertOperator::GreaterThan.evaluate(5.0, 3.0));
        assert!(!AlertOperator::GreaterThan.evaluate(3.0, 5.0));

        assert!(AlertOperator::LessThan.evaluate(3.0, 5.0));
        assert!(!AlertOperator::LessThan.evaluate(5.0, 3.0));

        assert!(AlertOperator::Equal.evaluate(5.0, 5.0));
        assert!(!AlertOperator::Equal.evaluate(5.0, 3.0));
    }

    #[test]
    fn test_threshold_checking() {
        let mut manager = AlertManager::new();
        manager.add_rules(configure_alerts());

        let mut metrics = HashMap::new();
        metrics.insert("http_error_rate".to_string(), 0.1);

        manager.check_thresholds(&metrics);

        assert!(!manager.get_active_alerts().is_empty());
    }
}
