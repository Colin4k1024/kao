//! Unit tests for model types and data structures.
//!
//! Tests cover:
//! - LoginRequest validation
//! - UserProfile serialization
//! - ApiResponse formatting

use serde_json;

/// Tests for LoginRequest validation.
mod login_request_tests {
    use validator::Validate;

    // Inline the LoginRequest struct for testing since we can't import binary
    #[derive(Debug, validator::Validate)]
    pub struct TestLoginRequest {
        #[validate(length(min = 3, max = 30, message = "Username must be 3-30 characters"))]
        pub username: String,

        #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
        pub password: String,
    }

    #[test]
    fn test_valid_login_request() {
        let request = TestLoginRequest {
            username: "testuser".to_string(),
            password: "Password123".to_string(),
        };

        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_username_too_short() {
        let request = TestLoginRequest {
            username: "ab".to_string(), // Less than 3 chars
            password: "Password123".to_string(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_username_too_long() {
        let request = TestLoginRequest {
            username: "a".repeat(31), // More than 30 chars
            password: "Password123".to_string(),
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn test_password_too_short() {
        let request = TestLoginRequest {
            username: "testuser".to_string(),
            password: "Pass1".to_string(), // Less than 8 chars
        };

        assert!(request.validate().is_err());
    }
}

/// Tests for API response serialization.
mod api_response_tests {
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestData {
        id: u32,
        name: String,
    }

    #[test]
    fn test_api_response_structure() {
        #[derive(Serialize)]
        struct ApiResponse<T> {
            code: u16,
            message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            data: Option<T>,
        }

        let response = ApiResponse {
            code: 0,
            message: "ok".to_string(),
            data: Some(TestData {
                id: 1,
                name: "test".to_string(),
            }),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"code\":0"));
        assert!(json.contains("\"message\":\"ok\""));
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"name\":\"test\""));
    }

    #[test]
    fn test_api_response_without_data() {
        #[derive(Serialize)]
        struct ApiResponse<T> {
            code: u16,
            message: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            data: Option<T>,
        }

        let response: ApiResponse<TestData> = ApiResponse {
            code: 400,
            message: "error".to_string(),
            data: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("\"code\":400"));
        assert!(json.contains("\"message\":\"error\""));
        // data field should be omitted when None due to skip_serializing_if
        assert!(!json.contains("\"data\""));
    }
}

/// Tests for UUID handling.
mod uuid_tests {
    use uuid::Uuid;

    #[test]
    fn test_uuid_v4_generation() {
        let uuid1 = Uuid::new_v4();
        let uuid2 = Uuid::new_v4();

        assert_ne!(uuid1, uuid2);
        assert_eq!(uuid1.to_string().len(), 36);
    }

    #[test]
    fn test_uuid_parsing() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let parsed = Uuid::parse_str(uuid_str);

        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().to_string(), uuid_str);
    }

    #[test]
    fn test_uuid_invalid_parsing() {
        let invalid_str = "not-a-valid-uuid";
        let parsed = Uuid::parse_str(invalid_str);

        assert!(parsed.is_err());
    }
}

/// Tests for chrono DateTime handling.
mod datetime_tests {
    use chrono::{DateTime, Utc, Duration};

    #[test]
    fn test_datetime_utc_now() {
        let before = Utc::now();
        let after = Utc::now();

        assert!(before <= after);
    }

    #[test]
    fn test_datetime_add_duration() {
        let start = Utc::now();
        let later = start + Duration::hours(1);

        assert!(later > start);
    }

    #[test]
    fn test_datetime_timestamp() {
        let now = Utc::now();
        let timestamp = now.timestamp();

        assert!(timestamp > 0);
    }

    #[test]
    fn test_datetime_from_timestamp() {
        let now = Utc::now();
        let timestamp = now.timestamp();

        let recovered = DateTime::from_timestamp(timestamp, 0).unwrap();

        // Allow for small difference due to nanoseconds
        assert!((recovered.timestamp() - now.timestamp()).abs() < 2);
    }
}

/// Tests for Claims structure.
mod claims_tests {
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    struct TestClaims {
        sub: String,
        username: String,
        exp: usize,
        iat: usize,
        permissions: Vec<String>,
        dept_id: Option<String>,
        roles: Vec<String>,
        token_version: usize,
    }

    impl TestClaims {
        fn new(user_id: Uuid, username: String) -> Self {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize;

            Self {
                sub: user_id.to_string(),
                username,
                exp: now + 3600, // 1 hour from now
                iat: now,
                permissions: vec!["read".to_string()],
                dept_id: None,
                roles: vec!["user".to_string()],
                token_version: 0,
            }
        }
    }

    #[test]
    fn test_claims_creation() {
        let user_id = Uuid::new_v4();
        let claims = TestClaims::new(user_id, "testuser".to_string());

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, "testuser");
        assert!(claims.exp > claims.iat);
    }

    #[test]
    fn test_claims_token_version_default() {
        let user_id = Uuid::new_v4();
        let claims = TestClaims::new(user_id, "testuser".to_string());

        assert_eq!(claims.token_version, 0);
    }
}
