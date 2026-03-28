//! Unit tests for authentication logic.
//!
//! Tests cover:
//! - Password hashing with bcrypt
//! - Password verification
//! - JWT token generation and validation

use kao_backend::common::auth::jwt::{generate_jwt, validate_jwt};
use kao_backend::common::auth::claims::Claims;
use uuid::Uuid;

/// Tests for password hashing using bcrypt.
mod password_tests {
    use super::*;

    #[test]
    fn test_hash_password_produces_bcrypt_hash() {
        // The hash_password function in the model module produces bcrypt hashes
        // that start with "$2a$" or "$2b$"
        // Since we can't directly test hash_password without the full module,
        // we test the underlying bcrypt behavior
        let password = "TestPassword123";
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

        assert!(hash.starts_with("$2a$") || hash.starts_with("$2b$"));
        assert_ne!(hash, password);
    }

    #[test]
    fn test_bcrypt_verify_correct_password() {
        let password = "MySecurePassword123";
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

        let is_valid = bcrypt::verify(password, &hash).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_bcrypt_verify_wrong_password() {
        let password = "MySecurePassword123";
        let wrong_password = "WrongPassword456";
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

        let is_valid = bcrypt::verify(wrong_password, &hash).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_bcrypt_different_hashes_for_same_password() {
        let password = "SamePassword123";
        let hash1 = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        let hash2 = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();

        // Same password should produce different hashes due to random salt
        assert_ne!(hash1, hash2);

        // But both should verify correctly
        assert!(bcrypt::verify(password, &hash1).unwrap());
        assert!(bcrypt::verify(password, &hash2).unwrap());
    }
}

/// Tests for JWT token generation and validation.
mod jwt_tests {
    use super::*;

    const TEST_SECRET: &str = "test-secret-key-for-unit-tests-only";

    fn create_test_claims() -> Claims {
        Claims::new(
            Uuid::new_v4(),
            "testuser".to_string(),
            vec!["read".to_string(), "write".to_string()],
            None,
            vec!["admin".to_string()],
        )
    }

    #[test]
    fn test_generate_jwt_token_creates_valid_token() {
        let claims = create_test_claims();
        let token = generate_jwt(claims.clone(), TEST_SECRET);

        assert!(token.is_ok());
        let token_str = token.unwrap();
        assert!(!token_str.is_empty());
        // JWT tokens have three parts separated by dots
        assert_eq!(token_str.split('.').count(), 3);
    }

    #[test]
    fn test_validate_jwt_token_returns_claims() {
        let original_claims = create_test_claims();
        let token = generate_jwt(original_claims.clone(), TEST_SECRET).unwrap();

        let validated_claims = validate_jwt(&token, TEST_SECRET);

        assert!(validated_claims.is_ok());
        let claims = validated_claims.unwrap();
        assert_eq!(claims.username, original_claims.username);
        assert_eq!(claims.sub, original_claims.sub);
        assert_eq!(claims.permissions, original_claims.permissions);
        assert_eq!(claims.roles, original_claims.roles);
    }

    #[test]
    fn test_validate_jwt_rejects_invalid_token() {
        let result = validate_jwt("invalid.token.here", TEST_SECRET);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_jwt_rejects_wrong_secret() {
        let claims = create_test_claims();
        let token = generate_jwt(claims, TEST_SECRET).unwrap();

        let result = validate_jwt(&token, "wrong-secret");
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_jwt_token_with_custom_expiration() {
        use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Serialize, Deserialize, Clone)]
        struct TestClaims {
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

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        // Create an already-expired token
        let expired_claims = TestClaims {
            sub: "user-123".to_string(),
            username: "testuser".to_string(),
            exp: now - 3600, // Expired 1 hour ago
            iat: now - 7200,
            permissions: vec![],
            dept_id: None,
            roles: vec![],
            token_version: 0,
        };

        let token = encode(
            &Header::default(),
            &expired_claims,
            &EncodingKey::from_secret(TEST_SECRET.as_bytes()),
        ).unwrap();

        // Validation should fail for expired token
        let result = decode::<TestClaims>(
            &token,
            &DecodingKey::from_secret(TEST_SECRET.as_bytes()),
            &Validation::default(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_claims_struct_clone() {
        let claims = create_test_claims();
        let cloned = claims.clone();

        assert_eq!(claims.sub, cloned.sub);
        assert_eq!(claims.username, cloned.username);
    }
}
