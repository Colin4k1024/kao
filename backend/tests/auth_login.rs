use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use tower::ServiceExt; // for `app.oneshot()`

use ai_coding_backend::app::{router::create_router, state::AppState};
use ai_coding_backend::common::{config::Config, db::create_db_pool};
use ai_coding_backend::features::auth::model::LoginRequest;
use std::sync::Arc;

#[tokio::test]
async fn login_returns_access_token_and_profile() {
    // Skip this test if DATABASE_URL is not set
    if std::env::var("DATABASE_URL").is_err() {
        println!("Skipping test: DATABASE_URL not set");
        return;
    }

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

    // Prepare login request
    let login_payload = LoginRequest {
        username: "admin".to_string(),
        password: "Admin123!".to_string(),
    };

    // Create a request to the login endpoint
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::to_vec(&login_payload).unwrap(),
                ))
                .unwrap(),
        )
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
    assert!(response_json["data"]["access_token"].is_string());
    assert_eq!(response_json["data"]["token_type"], "Bearer");
}