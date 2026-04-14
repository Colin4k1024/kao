//! Unit tests for Claims structure and JWT claims handling.
//!
//! Tests cover:
//! - Claims creation with various parameters
//! - Claims serialization/deserialization
//! - Claims field access
//! - Token version handling

use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Reimplementation of Claims for testing since we can't import binary crate
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,
    username: String,
    exp: usize,
    iat: usize,
    permissions: Vec<String>,
    dept_id: Option<String>,
    roles: Vec<String>,
    #[serde(default)]
    token_version: usize,
}

impl Claims {
    fn new(
        user_id: Uuid,
        username: String,
        permissions: Vec<String>,
        dept_id: Option<Uuid>,
        roles: Vec<String>,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as usize;

        Self {
            sub: user_id.to_string(),
            username,
            exp: now + 24 * 60 * 60,
            iat: now,
            permissions,
            dept_id: dept_id.map(|id| id.to_string()),
            roles,
            token_version: 0,
        }
    }

    fn with_version(&self, version: usize) -> Self {
        Claims {
            token_version: version,
            ..self.clone()
        }
    }
}

/// Tests for Claims creation.
mod claims_creation_tests {
    use super::*;

    #[test]
    fn test_claims_new_with_all_fields() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(
            user_id,
            "testuser".to_string(),
            vec!["read".to_string(), "write".to_string()],
            Some(Uuid::new_v4()),
            vec!["admin".to_string()],
        );

        assert_eq!(claims.sub, user_id.to_string());
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.permissions, vec!["read", "write"]);
        assert!(claims.dept_id.is_some());
        assert_eq!(claims.roles, vec!["admin"]);
        assert_eq!(claims.token_version, 0);
    }

    #[test]
    fn test_claims_new_without_dept_id() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(
            user_id,
            "testuser".to_string(),
            vec!["read".to_string()],
            None,
            vec!["user".to_string()],
        );

        assert_eq!(claims.sub, user_id.to_string());
        assert!(claims.dept_id.is_none());
    }

    #[test]
    fn test_claims_expiration_in_future() {
        let user_id = Uuid::new_v4();
        let before = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        
        let claims = Claims::new(
            user_id,
            "testuser".to_string(),
            vec![],
            None,
            vec![],
        );

        let after = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // exp should be iat + 24 hours
        assert!(claims.exp >= before + 24 * 60 * 60 - 1);
        assert!(claims.exp <= after + 24 * 60 * 60 + 1);
        assert_eq!(claims.iat, claims.exp - 24 * 60 * 60);
    }

    #[test]
    fn test_claims_empty_permissions_and_roles() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(
            user_id,
            "testuser".to_string(),
            vec![],
            None,
            vec![],
        );

        assert!(claims.permissions.is_empty());
        assert!(claims.roles.is_empty());
    }
}

/// Tests for Claims serialization.
mod claims_serialization_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_claims_json_serialization() {
        let user_id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
        let claims = Claims::new(
            user_id,
            "testuser".to_string(),
            vec!["read".to_string()],
            None,
            vec!["admin".to_string()],
        );

        let json = serde_json::to_string(&claims).unwrap();
        
        assert!(json.contains("\"sub\":\"550e8400-e29b-41d4-a716-446655440000\""));
        assert!(json.contains("\"username\":\"testuser\""));
        assert!(json.contains("\"permissions\":[\"read\"]"));
        assert!(json.contains("\"roles\":[\"admin\"]"));
    }

    #[test]
    fn test_claims_json_deserialization() {
        let json = r#"{
            "sub": "550e8400-e29b-41d4-a716-446655440000",
            "username": "testuser",
            "exp": 1700000000,
            "iat": 1699900000,
            "permissions": ["read", "write"],
            "dept_id": null,
            "roles": ["user"],
            "token_version": 1
        }"#;

        let claims: Claims = serde_json::from_str(json).unwrap();
        
        assert_eq!(claims.sub, "550e8400-e29b-41d4-a716-446655440000");
        assert_eq!(claims.username, "testuser");
        assert_eq!(claims.permissions, vec!["read", "write"]);
        assert!(claims.dept_id.is_none());
        assert_eq!(claims.roles, vec!["user"]);
        assert_eq!(claims.token_version, 1);
    }

    #[test]
    fn test_claims_json_with_dept_id() {
        let json = r#"{
            "sub": "550e8400-e29b-41d4-a716-446655440000",
            "username": "testuser",
            "exp": 1700000000,
            "iat": 1699900000,
            "permissions": [],
            "dept_id": "660e8400-e29b-41d4-a716-446655440000",
            "roles": []
        }"#;

        let claims: Claims = serde_json::from_str(json).unwrap();
        
        assert!(claims.dept_id.is_some());
        assert_eq!(claims.dept_id.unwrap(), "660e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn test_claims_deserialization_missing_optional_fields() {
        let json = r#"{
            "sub": "550e8400-e29b-41d4-a716-446655440000",
            "username": "testuser",
            "exp": 1700000000,
            "iat": 1699900000,
            "permissions": [],
            "roles": []
        }"#;

        let claims: Claims = serde_json::from_str(json).unwrap();
        
        assert!(claims.dept_id.is_none());
        assert_eq!(claims.token_version, 0); // Default value
    }
}

/// Tests for Claims field access.
mod claims_field_tests {
    use super::*;

    #[test]
    fn test_claims_with_multiple_permissions() {
        let user_id = Uuid::new_v4();
        let permissions = vec![
            "read".to_string(),
            "write".to_string(),
            "delete".to_string(),
            "admin".to_string(),
        ];
        let claims = Claims::new(
            user_id,
            "admin".to_string(),
            permissions.clone(),
            None,
            vec![],
        );

        assert_eq!(claims.permissions.len(), 4);
        assert!(claims.permissions.contains(&"admin".to_string()));
    }

    #[test]
    fn test_claims_with_multiple_roles() {
        let user_id = Uuid::new_v4();
        let roles = vec![
            "superadmin".to_string(),
            "manager".to_string(),
            "user".to_string(),
        ];
        let claims = Claims::new(
            user_id,
            "multirole".to_string(),
            vec![],
            None,
            roles.clone(),
        );

        assert_eq!(claims.roles.len(), 3);
        assert!(claims.roles.contains(&"superadmin".to_string()));
    }

    #[test]
    fn test_claims_user_id_as_string() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(
            user_id,
            "testuser".to_string(),
            vec![],
            None,
            vec![],
        );

        // The sub field should be the string representation of UUID
        assert_eq!(claims.sub, user_id.to_string());
        assert!(claims.sub.contains("-"));
    }
}

/// Tests for token version handling.
mod token_version_tests {
    use super::*;

    #[test]
    fn test_claims_with_version() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(
            user_id,
            "testuser".to_string(),
            vec![],
            None,
            vec![],
        );

        let v1_claims = claims.with_version(1);
        assert_eq!(v1_claims.token_version, 1);
        
        // Original claims should be unchanged
        assert_eq!(claims.token_version, 0);
    }

    #[test]
    fn test_claims_with_different_versions() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(
            user_id,
            "testuser".to_string(),
            vec![],
            None,
            vec![],
        );

        let v5_claims = claims.with_version(5);
        let v10_claims = claims.with_version(10);

        assert_eq!(v5_claims.token_version, 5);
        assert_eq!(v10_claims.token_version, 10);
        
        // All should share same base claims
        assert_eq!(v5_claims.sub, v10_claims.sub);
        assert_eq!(v5_claims.username, v10_claims.username);
    }

    #[test]
    fn test_claims_clone_preserves_version() {
        let user_id = Uuid::new_v4();
        let claims = Claims::new(
            user_id,
            "testuser".to_string(),
            vec![],
            None,
            vec![],
        );

        let v2_claims = claims.with_version(2);
        let cloned = v2_claims.clone();

        assert_eq!(cloned.token_version, 2);
        assert_eq!(cloned.sub, v2_claims.sub);
        assert_eq!(cloned.username, v2_claims.username);
    }
}
