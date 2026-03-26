use axum::{
    routing::{get, post},
    Router,
};

use crate::features::monitoring::online_user::OnlineUserController;

pub fn online_user_router() -> Router {
    Router::new()
        .route("/", get(OnlineUserController::get_online_users_handler))
        .route(
            "/force-logout",
            post(OnlineUserController::force_logout_handler),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_online_user_router() {
        // Basic test to ensure router compiles
        let router = online_user_router();
        assert!(router.is_classified());
    }
}
