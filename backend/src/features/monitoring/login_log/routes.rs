use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::features::monitoring::login_log::LoginLogController;

pub fn login_log_router() -> axum::Router<PgPool> {
    Router::new()
        .route("/", post(LoginLogController::create_login_log_handler))
        .route("/", get(LoginLogController::get_login_logs_handler))
        .route("/:id", get(LoginLogController::get_login_log_handler))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_log_router() {
        // Basic test to ensure router compiles
        let router = login_log_router();
        assert!(router.is_classified());
    }
}
