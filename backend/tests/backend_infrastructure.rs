use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt; // for `app.oneshot()`

use ai_coding_backend::app::{router::create_router, state::AppState};
use ai_coding_backend::common::{config::Config, db::create_db_pool};
use std::sync::Arc;

#[tokio::test]
async fn health_route_returns_ok() {
    // Create a mock config for testing
    let config = Arc::new(Config {
        port: 3000,
        database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests"),
        jwt_secret: "test_secret_for_testing".to_string(),
    });

    // Create a test database pool
    let db_pool = create_db_pool(&config.database_url).await.unwrap();

    // Create app state
    let app_state = AppState {
        db: db_pool,
        config: config.clone(),
    };

    // Create the app router
    let app = create_router(app_state);

    // Create a request to the health endpoint
    let response = app
        .oneshot(Request::builder().uri("/api/v1/health").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Assert the response status
    assert_eq!(response.status(), StatusCode::OK);

    // Read the response body
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    
    // Parse the response as JSON and verify it contains expected content
    let response_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    assert_eq!(response_json["code"], 0);
    assert_eq!(response_json["message"], "ok");
    assert_eq!(response_json["data"]["status"], "ok");
}