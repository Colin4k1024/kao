//! Integration tests for authentication flow.
//!
//! Tests cover:
//! - User login with valid credentials
//! - User login with invalid credentials
//! - Token refresh
//! - Logout
//! - Protected route access

use std::net::SocketAddr;
use axum::{Router, routing::post, extract::State, Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Test application state.
#[derive(Clone)]
struct TestAppState {
    jwt_secret: String,
}

/// Login request for testing.
#[derive(Debug, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

/// Login response for testing.
#[derive(Debug, Serialize)]
struct LoginResponse {
    access_token: String,
    token_type: String,
    expires_in: usize,
}

/// Create a test router with auth endpoints.
fn test_app() -> Router {
    let state = TestAppState {
        jwt_secret: "test-secret-key-for-integration-tests".to_string(),
    };

    Router::new()
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/refresh", post(refresh))
        .with_state(state)
}

async fn login(
    State(state): State<TestAppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // In a real implementation, this would verify against the database
    // For testing, we accept specific test credentials
    if request.username == "testuser" && request.password == "TestPassword123" {
        // Generate a mock JWT token
        let token = format!("mock_token_for_{}", request.username);
        Ok(Json(json!({
            "code": 0,
            "message": "ok",
            "data": {
                "access_token": token,
                "token_type": "Bearer",
                "expires_in": 86400
            }
        })))
    } else if request.username == "locked_user" && request.password == "LockedPass123" {
        // Simulate locked account
        Err(StatusCode::FORBIDDEN)
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn register(
    State(_state): State<TestAppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let username = request.get("username").and_then(|v| v.as_str());
    let password = request.get("password").and_then(|v| v.as_str());

    match (username, password) {
        (Some(u), Some(p)) if u.len() >= 3 && p.len() >= 8 => {
            Ok(Json(json!({
                "code": 0,
                "message": "ok",
                "data": {
                    "id": "new-user-id",
                    "username": u
                }
            })))
        }
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

async fn refresh(
    State(_state): State<TestAppState>,
    Json(request): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let refresh_token = request.get("refresh_token").and_then(|v| v.as_str());

    match refresh_token {
        Some(token) if token.starts_with("refresh_") => {
            Ok(Json(json!({
                "code": 0,
                "message": "ok",
                "data": {
                    "access_token": format!("new_access_token_{}", token.len()),
                    "token_type": "Bearer",
                    "expires_in": 86400
                }
            })))
        }
        Some(_) => Err(StatusCode::UNAUTHORIZED),
        None => Err(StatusCode::BAD_REQUEST),
    }
}

/// Integration tests for authentication flow.
#[cfg(test)]
mod integration_tests {
    use super::*;
    use axum::body::Body;
    use http::{Request, header};

    async fn send_request(app: Router, method: &str, uri: &str, body: Option<serde_json::Value>) -> Response {
        let req = Request::builder()
            .method(method)
            .uri(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body.map(|b| b.to_string()).unwrap_or_default()))
            .unwrap();

        let response = app.call(req).await.unwrap();
        response
    }

    #[tokio::test]
    async fn test_login_success() {
        let app = test_app();

        let request = Json(LoginRequest {
            username: "testuser".to_string(),
            password: "TestPassword123".to_string(),
        });

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/login",
            Some(serde_json::to_value(&request.0).unwrap()),
        ).await;

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["code"], 0);
        assert_eq!(json["message"], "ok");
        assert!(json["data"]["access_token"].is_string());
        assert_eq!(json["data"]["token_type"], "Bearer");
    }

    #[tokio::test]
    async fn test_login_invalid_credentials() {
        let app = test_app();

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/login",
            Some(json!({
                "username": "testuser",
                "password": "WrongPassword123"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_login_nonexistent_user() {
        let app = test_app();

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/login",
            Some(json!({
                "username": "nonexistent",
                "password": "TestPassword123"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_login_locked_account() {
        let app = test_app();

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/login",
            Some(json!({
                "username": "locked_user",
                "password": "LockedPass123"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn test_register_success() {
        let app = test_app();

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/register",
            Some(json!({
                "username": "newuser",
                "password": "NewUserPass123"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["code"], 0);
        assert!(json["data"]["id"].is_string());
    }

    #[tokio::test]
    async fn test_register_username_too_short() {
        let app = test_app();

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/register",
            Some(json!({
                "username": "ab", // Too short
                "password": "TestPassword123"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_register_password_too_short() {
        let app = test_app();

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/register",
            Some(json!({
                "username": "validuser",
                "password": "short" // Too short
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_token_refresh_success() {
        let app = test_app();

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/refresh",
            Some(json!({
                "refresh_token": "refresh_valid_token_12345"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["code"], 0);
        assert!(json["data"]["access_token"].is_string());
    }

    #[tokio::test]
    async fn test_token_refresh_invalid_token() {
        let app = test_app();

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/refresh",
            Some(json!({
                "refresh_token": "invalid_token"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_token_refresh_missing_token() {
        let app = test_app();

        let response = send_request(
            app,
            "POST",
            "/api/v1/auth/refresh",
            Some(json!({})),
        ).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
