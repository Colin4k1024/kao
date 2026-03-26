use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::features::monitoring::operation_log::OperationLogController;

pub fn operation_log_router() -> Router {
    Router::new()
        .route(
            "/",
            post(OperationLogController::create_operation_log_handler),
        )
        .route("/", get(OperationLogController::get_operation_logs_handler))
        .route(
            "/:id",
            get(OperationLogController::get_operation_log_handler),
        )
        .route(
            "/:id",
            delete(OperationLogController::delete_operation_log_handler),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operation_log_router() {
        // Basic test to ensure router compiles
        let router = operation_log_router();
        assert!(router.is_classified());
    }
}
