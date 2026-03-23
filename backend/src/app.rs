use crate::db::create_db_pool;
use crate::handlers::{auth_routes, user_routes};
use crate::middleware::auth::auth_middleware;
use crate::services::user_service::UserService;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

pub fn create_app() -> axum::Router {
    // 创建数据库连接池
    let pool = tokio::runtime::Handle::current()
        .block_on(create_db_pool())
        .expect("Failed to create database pool");

    let user_service = Arc::new(UserService::new(pool));

    // CORS 配置
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any());

    // 路由
    let api = axum::Router::new()
        .merge(auth_routes(user_service.clone()))
        .merge(user_routes(user_service.clone()));

    // 添加认证中间件的路由
    let protected_api = axum::Router::new()
        .nest("/api/v1", api);

    axum::Router::new()
        .route("/", axum::routing::get(root))
        .merge(protected_api)
        .layer(cors)
        .layer(ServiceBuilder::new().layer(axum::middleware::from_fn(auth_middleware)))
}

async fn root() -> &'static str {
    "AI Coding Backend - Rust + Axum\nAPI Docs: /api/docs"
}
