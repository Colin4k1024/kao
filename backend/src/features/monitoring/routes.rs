use axum::{Router, routing::get};

use super::health;
use super::login_log;
use super::metrics;
use super::online_user;
use super::operation_log;

pub fn monitoring_router() -> Router {
    Router::new()
        .route("/metrics", get(metrics::get_metrics))
        .route("/health", get(health::check_health))
        .route("/alerts", get(get_alerts))
        .nest("/oper/logs", operation_log::routes::operation_log_router())
        .nest("/login/logs", login_log::routes::login_log_router())
        .nest("/online/users", online_user::routes::online_user_router())
        // Security endpoints
        .nest("/security", security_router())
}

// Alert endpoint
async fn get_alerts() -> &'static str {
    "Alerting endpoint - configuration required"
}

// Security router
pub fn security_router() -> Router {
    Router::new()
        .route("/scan", get(security_scan))
        .route("/scan/configuration", get(config_scan))
        .route("/scan/input-validation", get(input_scan))
        .route("/scan/authentication", get(auth_scan))
        .route("/scan/authorization", get(authz_scan))
        .route("/events", get(security_events))
        .route("/password-health/{user_id}", get(password_health))
}

// Security scan
async fn security_scan() -> &'static str {
    "Run full security scan"
}

// Configuration scan
async fn config_scan() -> &'static str {
    "Run configuration scan"
}

// Input validation scan
async fn input_scan() -> &'static str {
    "Run input validation scan"
}

// Authentication scan
async fn auth_scan() -> &'static str {
    "Run authentication scan"
}

// Authorization scan
async fn authz_scan() -> &'static str {
    "Run authorization scan"
}

// Security events
async fn security_events() -> &'static str {
    "Get security events"
}

// Password health
async fn password_health() -> &'static str {
    "Get password health"
}
