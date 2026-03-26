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
}

// Alert endpoint
async fn get_alerts() -> &'static str {
    "Alerting endpoint - configuration required"
}
