//! Unit tests for API response structures.
//!
//! Tests cover:
//! - ApiResponse::success() conversion
//! - ApiResponse::success_no_data() conversion
//! - ApiResponse::error() conversion
//! - Serialization to JSON

use kao_backend::common::response::ApiResponse;
use axum::response::IntoResponse;
use serde::Serialize;
use std::str;

/// Test data structure for serialization tests.
#[derive(Serialize)]
struct TestData {
    id: u32,
    name: String,
    active: bool,
}

/// Helper function to extract body string from response
fn extract_body(response: axum::response::Response) -> String {
    let (_parts, body) = response.into_parts();
    // Use blocking call since we're in test context
    let bytes = tokio::runtime::Runtime::new().unwrap().block_on(
        axum::body::to_bytes(body, 1024)
    ).unwrap();
    str::from_utf8(&bytes).unwrap().to_string()
}

/// Tests for ApiResponse::success() method.
mod success_response_tests {
    use super::*;

    #[test]
    fn test_success_response_with_data() {
        let data = TestData {
            id: 1,
            name: "test".to_string(),
            active: true,
        };
        let response = ApiResponse::success(data);
        let body = extract_body(response.into_response());
        
        assert!(body.contains("\"code\":200"));
        assert!(body.contains("\"message\":\"ok\""));
        assert!(body.contains("\"id\":1"));
        assert!(body.contains("\"name\":\"test\""));
        assert!(body.contains("\"active\":true"));
    }

    #[test]
    fn test_success_response_with_nested_data() {
        #[derive(Serialize)]
        struct NestedData {
            user: UserData,
            count: i32,
        }
        #[derive(Serialize)]
        struct UserData {
            id: u64,
            username: String,
        }
        
        let data = NestedData {
            user: UserData {
                id: 42,
                username: "testuser".to_string(),
            },
            count: 10,
        };
        let response = ApiResponse::success(data);
        let body = extract_body(response.into_response());
        
        assert!(body.contains("\"id\":42"));
        assert!(body.contains("\"username\":\"testuser\""));
        assert!(body.contains("\"count\":10"));
    }

    #[test]
    fn test_success_response_with_empty_vec() {
        #[derive(Serialize)]
        struct DataWithVec {
            items: Vec<String>,
        }
        
        let data = DataWithVec {
            items: vec![],
        };
        let response = ApiResponse::success(data);
        let body = extract_body(response.into_response());
        
        assert!(body.contains("\"items\":[]"));
    }
}

/// Tests for ApiResponse::success_no_data() method.
mod success_no_data_tests {
    use super::*;

    #[test]
    fn test_success_no_data_response() {
        let response = ApiResponse::success_no_data();
        let body = extract_body(response.into_response());
        
        assert!(body.contains("\"code\":200"));
        assert!(body.contains("\"message\":\"ok\""));
    }

    #[test]
    fn test_success_no_data_with_request_id() {
        let response = ApiResponse::success_no_data_with_request_id("req-123".to_string());
        let body = extract_body(response.into_response());
        
        assert!(body.contains("\"request_id\":\"req-123\""));
    }
}

/// Tests for ApiResponse::error() method.
mod error_response_tests {
    use super::*;

    #[test]
    fn test_error_response() {
        let response = ApiResponse::error(400, "Bad request".to_string());
        let body = extract_body(response.into_response());
        
        // Note: ApiResponse::error embeds code in body
        assert!(body.contains("\"code\":400"));
        assert!(body.contains("\"message\":\"Bad request\""));
    }

    #[test]
    fn test_error_response_with_request_id() {
        let response = ApiResponse::error_with_request_id(
            404,
            "Not found".to_string(),
            "req-456".to_string(),
        );
        let body = extract_body(response.into_response());
        
        assert!(body.contains("\"code\":404"));
        assert!(body.contains("\"message\":\"Not found\""));
        assert!(body.contains("\"request_id\":\"req-456\""));
    }

    #[test]
    fn test_error_various_codes() {
        let error_codes = vec![400, 401, 403, 404, 500];
        
        for code in error_codes {
            let response = ApiResponse::error(code, "Test".to_string());
            let body = extract_body(response.into_response());
            assert!(body.contains(&format!("\"code\":{}", code)));
        }
    }
}

/// Tests for ApiResponse with different data types.
mod response_serialization_tests {
    use super::*;

    #[test]
    fn test_response_with_optional_fields_some() {
        #[derive(Serialize)]
        struct DataWithOptional {
            required: String,
            optional: Option<String>,
        }
        
        let data = DataWithOptional {
            required: "test".to_string(),
            optional: Some("value".to_string()),
        };
        let response = ApiResponse::success(data);
        let body = extract_body(response.into_response());
        assert!(body.contains("\"optional\":\"value\""));
    }

    #[test]
    fn test_response_with_vector() {
        #[derive(Serialize)]
        struct ListData {
            items: Vec<i32>,
            total: usize,
        }
        
        let data = ListData {
            items: vec![1, 2, 3, 4, 5],
            total: 5,
        };
        let response = ApiResponse::success(data);
        let body = extract_body(response.into_response());
        
        assert!(body.contains("\"items\":[1,2,3,4,5]"));
        assert!(body.contains("\"total\":5"));
    }
}
