use axum::{Router, routing::get};
use crate::AppState;

use super::health;
use super::login_log;
use super::metrics;
use super::online_user;
use super::operation_log;
use super::security;

pub fn monitoring_router() -> Router<AppState> {
    Router::new()
        .route("/metrics", get(metrics::get_metrics))
        .route("/health", get(health::check_health))
        .route("/alerts", get(get_alerts))
        .nest("/oper/logs", operation_log::routes::operation_log_router())
        .nest("/login/logs", login_log::routes::login_log_router())
        .nest("/online/users", online_user::routes::online_user_router())
        // Security endpoints - use the new security module router
        .nest("/security", security::routes::security_router())
}

// Alert endpoint
async fn get_alerts() -> &'static str {
    "Alerting endpoint - configuration required"
}
