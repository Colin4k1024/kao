use axum::Router;

use super::health;
use super::login_log;
use super::metrics;
use super::online_user;
use super::operation_log;

pub fn monitoring_router() -> Router {
    Router::new()
        .route("/metrics", axum::routing::get(metrics::get_metrics))
        .route("/health", axum::routing::get(health::check_health))
        .nest("/oper/logs", operation_log::routes::operation_log_router())
        .nest("/login/logs", login_log::routes::login_log_router())
        .nest("/online/users", online_user::routes::online_user_router())
}
