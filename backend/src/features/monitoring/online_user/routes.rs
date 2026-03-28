use axum::{
    routing::{get, post},
    Router,
};
use crate::AppState;

use crate::features::monitoring::online_user::OnlineUserController;

pub fn online_user_router() -> axum::Router<AppState> {
    Router::new()
        .route("/", get(OnlineUserController::get_online_users_handler))
        .route(
            "/force-logout",
            post(OnlineUserController::force_logout_handler),
        )
}
