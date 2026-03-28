use axum::{
    routing::{delete, get, post},
    Router,
};
use crate::AppState;

use crate::features::monitoring::operation_log::OperationLogController;

pub fn operation_log_router() -> axum::Router<AppState> {
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
