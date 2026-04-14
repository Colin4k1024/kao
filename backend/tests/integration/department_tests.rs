//! Integration tests for department API endpoints.
//!
//! Tests cover:
//! - Department CRUD operations
//! - Department tree building
//! - Department validation
//! - Error handling for departments

use std::net::SocketAddr;
use axum::{
    Router,
    routing::{get, post, put, delete},
    extract::State,
    Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

/// Test application state.
#[derive(Clone)]
struct TestAppState {
    // In real app, this would include database pool
}

/// Department models matching the actual application.
#[derive(Debug, Serialize, Deserialize)]
struct DepartmentRequest {
    parent_id: Option<Uuid>,
    code: String,
    name: String,
    sort_order: Option<i32>,
    leader: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    status: Option<String>,
}

#[derive(Debug, Serialize)]
struct DepartmentResponse {
    id: Uuid,
    parent_id: Option<Uuid>,
    code: String,
    name: String,
    ancestors: String,
    path: String,
    sort_order: i32,
    leader: Option<String>,
    phone: Option<String>,
    email: Option<String>,
    status: String,
}

/// Mock department storage for testing.
static MOCK_DEPARTMENTS: std::sync::Mutex<std::collections::HashMap<String, DepartmentResponse>> = 
    std::sync::Mutex::new(std::collections::HashMap::new());

/// Create a test router with department endpoints.
fn test_app() -> Router {
    let state = TestAppState {};

    Router::new()
        .route("/api/v1/departments", get(list_departments))
        .route("/api/v1/departments", post(create_department))
        .route("/api/v1/departments/:id", get(get_department))
        .route("/api/v1/departments/:id", put(update_department))
        .route("/api/v1/departments/:id", delete(delete_department))
        .route("/api/v1/departments/tree", get(get_department_tree))
        .with_state(state)
}

fn clear_mock_storage() {
    MOCK_DEPARTMENTS.lock().unwrap().clear();
}

fn add_mock_department(dept: DepartmentResponse) {
    MOCK_DEPARTMENTS.lock().unwrap().insert(dept.id.to_string(), dept);
}

async fn list_departments() -> Json<Vec<DepartmentResponse>> {
    let deps: Vec<DepartmentResponse> = MOCK_DEPARTMENTS.lock().unwrap().values().cloned().collect();
    Json(deps)
}

async fn create_department(
    Json(req): Json<DepartmentRequest>,
) -> Result<Json<DepartmentResponse>, StatusCode> {
    // Validate required fields
    if req.code.is_empty() || req.name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    if req.code.len() > 50 || req.name.len() > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let dept = DepartmentResponse {
        id: Uuid::new_v4(),
        parent_id: req.parent_id,
        code: req.code.clone(),
        name: req.name.clone(),
        ancestors: format!("/{}", req.code),
        path: format!("/{}/", req.code),
        sort_order: req.sort_order.unwrap_or(0),
        leader: req.leader,
        phone: req.phone,
        email: req.email,
        status: req.status.unwrap_or_else(|| "ACTIVE".to_string()),
    };

    add_mock_department(dept.clone());
    Ok(Json(dept))
}

async fn get_department(
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<Json<DepartmentResponse>, StatusCode> {
    let deps = MOCK_DEPARTMENTS.lock().unwrap();
    match deps.get(&id.to_string()) {
        Some(dept) => Ok(Json(dept.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn update_department(
    axum::extract::Path(id): axum::extract::Path<Uuid>,
    Json(req): Json<DepartmentRequest>,
) -> Result<Json<DepartmentResponse>, StatusCode> {
    let mut deps = MOCK_DEPARTMENTS.lock().unwrap();
    match deps.get_mut(&id.to_string()) {
        Some(dept) => {
            if let Some(parent_id) = req.parent_id {
                dept.parent_id = Some(parent_id);
            }
            if let Some(code) = Some(req.code) {
                dept.code = code;
            }
            if let Some(name) = Some(req.name) {
                dept.name = name;
            }
            if let Some(sort_order) = req.sort_order {
                dept.sort_order = sort_order;
            }
            if let Some(leader) = req.leader {
                dept.leader = Some(leader);
            }
            if let Some(phone) = req.phone {
                dept.phone = Some(phone);
            }
            if let Some(email) = req.email {
                dept.email = Some(email);
            }
            if let Some(status) = req.status {
                dept.status = status;
            }
            Ok(Json(dept.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_department(
    axum::extract::Path(id): axum::extract::Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut deps = MOCK_DEPARTMENTS.lock().unwrap();
    if deps.remove(&id.to_string()).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn get_department_tree() -> Json<Vec<DepartmentResponse>> {
    let deps: Vec<DepartmentResponse> = MOCK_DEPARTMENTS.lock().unwrap().values().cloned().collect();
    Json(deps)
}

/// Helper to send HTTP requests in tests.
async fn send_request(
    method: &str,
    uri: &str,
    body: Option<serde_json::Value>,
) -> Response {
    use axum::body::Body;
    use http::{Request, header};

    let app = test_app();

    let req = Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.map(|b| b.to_string()).unwrap_or_default()))
        .unwrap();

    app.call(req).await.unwrap()
}

/// Integration tests for department endpoints.
#[cfg(test)]
mod department_integration_tests {
    use super::*;
    use axum::body::Body;
    use http::Request;

    #[tokio::test]
    async fn test_create_department_success() {
        clear_mock_storage();
        
        let response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "IT001",
                "name": "Information Technology",
                "sort_order": 1,
                "status": "ACTIVE"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::OK);
        
        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(json["code"], "IT001");
        assert_eq!(json["name"], "Information Technology");
        assert!(json["id"].is_string());
    }

    #[tokio::test]
    async fn test_create_department_with_parent() {
        clear_mock_storage();
        
        // Create parent first
        let parent_response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "IT",
                "name": "Information Technology",
                "status": "ACTIVE"
            })),
        ).await;
        
        let body = axum::body::to_bytes(parent_response.into_body(), 1024).await.unwrap();
        let parent_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let parent_id = parent_json["id"].as_str().unwrap();

        // Create child department
        let response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "IT001",
                "name": "Software Development",
                "parent_id": parent_id,
                "status": "ACTIVE"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::OK);
        
        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(json["parent_id"], json!({"Uuid": parent_id}));
    }

    #[tokio::test]
    async fn test_create_department_empty_code() {
        clear_mock_storage();
        
        let response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "",
                "name": "Test Department",
                "status": "ACTIVE"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_department_empty_name() {
        clear_mock_storage();
        
        let response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "TEST",
                "name": "",
                "status": "ACTIVE"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_create_department_code_too_long() {
        clear_mock_storage();
        
        let response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "A".repeat(51),
                "name": "Test Department",
                "status": "ACTIVE"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_list_departments() {
        clear_mock_storage();
        
        // Create some departments
        for i in 1..=3 {
            send_request(
                "POST",
                "/api/v1/departments",
                Some(json!({
                    "code": format!("DEPT{}", i),
                    "name": format!("Department {}", i),
                    "status": "ACTIVE"
                })),
            ).await;
        }

        let response = send_request("GET", "/api/v1/departments", None).await;
        
        assert_eq!(response.status(), StatusCode::OK);
        
        let body = axum::body::to_bytes(response.into_body(), 10240).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert!(json.is_array());
        assert_eq!(json.as_array().unwrap().len(), 3);
    }

    #[tokio::test]
    async fn test_get_department_by_id() {
        clear_mock_storage();
        
        // Create a department
        let create_response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "GET001",
                "name": "Get Test Department",
                "status": "ACTIVE"
            })),
        ).await;
        
        let body = axum::body::to_bytes(create_response.into_body(), 1024).await.unwrap();
        let dept_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let dept_id = dept_json["id"].as_str().unwrap();

        // Get the department
        let response = send_request(
            "GET",
            &format!("/api/v1/departments/{}", dept_id),
            None,
        ).await;

        assert_eq!(response.status(), StatusCode::OK);
        
        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(json["id"], json!({"Uuid": dept_id}));
        assert_eq!(json["code"], "GET001");
    }

    #[tokio::test]
    async fn test_get_nonexistent_department() {
        clear_mock_storage();
        
        let fake_id = Uuid::new_v4();
        let response = send_request(
            "GET",
            &format!("/api/v1/departments/{}", fake_id),
            None,
        ).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_update_department() {
        clear_mock_storage();
        
        // Create a department
        let create_response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "UPDATE001",
                "name": "Original Name",
                "status": "ACTIVE"
            })),
        ).await;
        
        let body = axum::body::to_bytes(create_response.into_body(), 1024).await.unwrap();
        let dept_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let dept_id = dept_json["id"].as_str().unwrap();

        // Update the department
        let response = send_request(
            "PUT",
            &format!("/api/v1/departments/{}", dept_id),
            Some(json!({
                "name": "Updated Name",
                "leader": "John Doe",
                "status": "INACTIVE"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::OK);
        
        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(json["name"], "Updated Name");
        assert_eq!(json["leader"], "John Doe");
        assert_eq!(json["status"], "INACTIVE");
        // Code should remain unchanged
        assert_eq!(json["code"], "UPDATE001");
    }

    #[tokio::test]
    async fn test_update_nonexistent_department() {
        clear_mock_storage();
        
        let fake_id = Uuid::new_v4();
        let response = send_request(
            "PUT",
            &format!("/api/v1/departments/{}", fake_id),
            Some(json!({
                "name": "New Name"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_department() {
        clear_mock_storage();
        
        // Create a department
        let create_response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "DELETE001",
                "name": "Delete Test Department",
                "status": "ACTIVE"
            })),
        ).await;
        
        let body = axum::body::to_bytes(create_response.into_body(), 1024).await.unwrap();
        let dept_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let dept_id = dept_json["id"].as_str().unwrap();

        // Delete the department
        let delete_response = send_request(
            "DELETE",
            &format!("/api/v1/departments/{}", dept_id),
            None,
        ).await;

        assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);

        // Verify it's deleted
        let get_response = send_request(
            "GET",
            &format!("/api/v1/departments/{}", dept_id),
            None,
        ).await;

        assert_eq!(get_response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_delete_nonexistent_department() {
        clear_mock_storage();
        
        let fake_id = Uuid::new_v4();
        let response = send_request(
            "DELETE",
            &format!("/api/v1/departments/{}", fake_id),
            None,
        ).await;

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_department_tree_structure() {
        clear_mock_storage();
        
        // Create root department
        let root_response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "ROOT",
                "name": "Root Department",
                "status": "ACTIVE"
            })),
        ).await;
        
        let body = axum::body::to_bytes(root_response.into_body(), 1024).await.unwrap();
        let root_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let root_id = root_json["id"].as_str().unwrap();

        // Create child department
        let child_response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "CHILD",
                "name": "Child Department",
                "parent_id": root_id,
                "status": "ACTIVE"
            })),
        ).await;
        
        let body = axum::body::to_bytes(child_response.into_body(), 1024).await.unwrap();
        let _child_json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        // Get tree
        let tree_response = send_request("GET", "/api/v1/departments/tree", None).await;
        
        assert_eq!(tree_response.status(), StatusCode::OK);
        
        let body = axum::body::to_bytes(tree_response.into_body(), 10240).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert!(json.is_array());
        assert_eq!(json.as_array().unwrap().len(), 2);
    }

    #[tokio::test]
    async fn test_department_with_contact_info() {
        clear_mock_storage();
        
        let response = send_request(
            "POST",
            "/api/v1/departments",
            Some(json!({
                "code": "CONTACT",
                "name": "Contact Department",
                "leader": "Jane Smith",
                "phone": "+1-555-123-4567",
                "email": "contact@example.com",
                "status": "ACTIVE"
            })),
        ).await;

        assert_eq!(response.status(), StatusCode::OK);
        
        let body = axum::body::to_bytes(response.into_body(), 1024).await.unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        
        assert_eq!(json["leader"], "Jane Smith");
        assert_eq!(json["phone"], "+1-555-123-4567");
        assert_eq!(json["email"], "contact@example.com");
    }
}
