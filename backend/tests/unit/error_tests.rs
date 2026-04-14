//! Unit tests for error handling.
//!
//! Tests cover:
//! - AppError creation and Display implementation
//! - ErrorResponseData structure
//! - AppError IntoResponse conversion
//! - Error type conversions

use kao_backend::common::error::{AppError, ErrorResponseData};
use axum::response::IntoResponse;
use std::str;

/// Tests for AppError creation and display.
mod app_error_tests {
    use super::*;

    #[test]
    fn test_authentication_error_display() {
        let err = AppError::Authentication("Invalid credentials".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Authentication error"));
        assert!(display.contains("Invalid credentials"));
    }

    #[test]
    fn test_authorization_error_display() {
        let err = AppError::Authorization("Access denied".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Authorization error"));
        assert!(display.contains("Access denied"));
    }

    #[test]
    fn test_validation_error_display() {
        let err = AppError::Validation {
            field: "username".to_string(),
            message: "Username is required".to_string(),
        };
        let display = format!("{}", err);
        assert!(display.contains("Validation error"));
        assert!(display.contains("username"));
        assert!(display.contains("Username is required"));
    }

    #[test]
    fn test_not_found_error_display() {
        let err = AppError::NotFound("User not found".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Not found"));
        assert!(display.contains("User not found"));
    }

    #[test]
    fn test_rate_limit_error_display() {
        let err = AppError::RateLimit { retry_after: 300 };
        let display = format!("{}", err);
        assert!(display.contains("Rate limit exceeded"));
        assert!(display.contains("300"));
    }

    #[test]
    fn test_database_error_display() {
        let err = AppError::Database("Connection failed".to_string());
        let display = format!("{}", err);
        assert!(display.contains("Database error"));
        assert!(display.contains("Connection failed"));
    }

    #[test]
    fn test_internal_error_with_message() {
        let err = AppError::Internal(Some("Something went wrong".to_string()));
        let display = format!("{}", err);
        assert!(display.contains("Internal error"));
        assert!(display.contains("Something went wrong"));
    }

    #[test]
    fn test_internal_error_without_message() {
        let err = AppError::Internal(None);
        let display = format!("{}", err);
        assert!(display.contains("Internal error"));
        assert!(display.contains("Unknown error"));
    }
}

/// Tests for ErrorResponseData structure.
mod error_response_data_tests {
    use super::*;

    #[test]
    fn test_error_response_data_new() {
        let response = ErrorResponseData::new("TestError");
        assert_eq!(response.error_type, "TestError");
        assert!(response.field.is_none());
        assert!(response.details.is_none());
        assert!(response.retry_after.is_none());
    }

    #[test]
    fn test_error_response_data_with_field() {
        let response = ErrorResponseData::new("ValidationError")
            .with_field("username");
        assert_eq!(response.error_type, "ValidationError");
        assert_eq!(response.field, Some("username".to_string()));
    }

    #[test]
    fn test_error_response_data_with_details() {
        let response = ErrorResponseData::new("ValidationError")
            .with_details("Username is too short");
        assert_eq!(response.details, Some("Username is too short".to_string()));
    }

    #[test]
    fn test_error_response_data_with_retry_after() {
        let response = ErrorResponseData::new("RateLimitError")
            .with_retry_after(600);
        assert_eq!(response.retry_after, Some(600));
    }

    #[test]
    fn test_error_response_data_chaining() {
        let response = ErrorResponseData::new("ValidationError")
            .with_field("password")
            .with_details("Password must be at least 8 characters")
            .with_retry_after(0);
        
        assert_eq!(response.error_type, "ValidationError");
        assert_eq!(response.field, Some("password".to_string()));
        assert!(response.details.is_some());
        assert_eq!(response.retry_after, Some(0));
    }
}

/// Helper function to extract body string from response
async fn extract_body(response: axum::response::Response) -> String {
    let (_parts, body) = response.into_parts();
    let bytes = axum::body::to_bytes(body, 1024).await.unwrap();
    str::from_utf8(&bytes).unwrap().to_string()
}

/// Tests for AppError to Response conversion.
mod error_into_response_tests {
    use super::*;

    #[tokio::test]
    async fn test_authentication_error_into_response() {
        let err = AppError::Authentication("Invalid token".to_string());
        let response = err.into_response();
        let body = extract_body(response).await;
        
        assert!(body.contains("AuthenticationError"));
        assert!(body.contains("Invalid token"));
    }

    #[tokio::test]
    async fn test_authorization_error_into_response() {
        let err = AppError::Authorization("Forbidden".to_string());
        let response = err.into_response();
        let body = extract_body(response).await;
        
        assert!(body.contains("AuthorizationError"));
    }

    #[tokio::test]
    async fn test_validation_error_into_response() {
        let err = AppError::Validation {
            field: "email".to_string(),
            message: "Invalid email format".to_string(),
        };
        let response = err.into_response();
        let body = extract_body(response).await;
        
        assert!(body.contains("ValidationError"));
        assert!(body.contains("email"));
        assert!(body.contains("Invalid email format"));
    }

    #[tokio::test]
    async fn test_not_found_error_into_response() {
        let err = AppError::NotFound("Resource not found".to_string());
        let response = err.into_response();
        let body = extract_body(response).await;
        
        assert!(body.contains("NotFoundError"));
    }

    #[tokio::test]
    async fn test_rate_limit_error_into_response() {
        let err = AppError::RateLimit { retry_after: 300 };
        let response = err.into_response();
        let body = extract_body(response).await;
        
        assert!(body.contains("RateLimitError"));
    }

    #[tokio::test]
    async fn test_database_error_into_response() {
        let err = AppError::Database("Connection refused".to_string());
        let response = err.into_response();
        let body = extract_body(response).await;
        
        assert!(body.contains("DatabaseError"));
    }

    #[tokio::test]
    async fn test_internal_error_into_response() {
        let err = AppError::Internal(Some("Server error".to_string()));
        let response = err.into_response();
        let body = extract_body(response).await;
        
        assert!(body.contains("InternalError"));
    }
}

/// Tests for error conversions.
mod error_conversions_tests {
    use super::*;

    #[test]
    fn test_app_error_from_jwt_error() {
        use jsonwebtoken::errors::ErrorKind;
        let jwt_err = jsonwebtoken::errors::Error::from(ErrorKind::ExpiredSignature);
        let app_err: AppError = jwt_err.into();
        
        match app_err {
            AppError::Authentication(msg) => {
                assert!(msg.contains("JWT error"));
            },
            _ => panic!("Expected Authentication error"),
        }
    }

    #[test]
    fn test_app_error_from_str() {
        let app_err: AppError = "Some error string".into();
        
        match app_err {
            AppError::Internal(Some(msg)) => {
                assert_eq!(msg, "Some error string");
            },
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_app_error_from_uuid_error() {
        // Create a uuid::Error by parsing an invalid UUID
        let result = uuid::Uuid::parse_str("not-a-valid-uuid");
        assert!(result.is_err());
        let uuid_err = result.unwrap_err();
        let app_err: AppError = uuid_err.into();
        
        match app_err {
            AppError::Validation { field, message } => {
                assert_eq!(field, "uuid");
                assert!(message.contains("Invalid UUID format"));
            },
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_app_error_from_serde_json_error() {
        let json_err = serde_json::from_str::<String>("not json").unwrap_err();
        let app_err: AppError = json_err.into();
        
        match app_err {
            AppError::Internal(Some(msg)) => {
                assert!(msg.contains("JSON serialization error"));
            },
            _ => panic!("Expected Internal error"),
        }
    }
}
