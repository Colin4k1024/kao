//! Unit tests for LoginRequest validation and auth model functions.
//!
//! Tests cover:
//! - LoginRequest validation with validator crate
//! - Password hashing with bcrypt
//! - Password verification
//! - Username format validation
//! - Password complexity validation

use validator::Validate;

/// Re-implement the validation functions from auth model for testing
/// since we can't directly test private functions
fn validate_username_format(username: &str) -> Result<(), validator::ValidationError> {
    if username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        Ok(())
    } else {
        Err(validator::ValidationError::new("username_format"))
    }
}

fn validate_password_complexity(password: &str) -> Result<(), validator::ValidationError> {
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());

    if has_uppercase && has_lowercase && has_digit {
        Ok(())
    } else {
        Err(validator::ValidationError::new("password_complexity"))
    }
}

/// Test LoginRequest structure matching the actual model
#[derive(Debug, Validate)]
struct TestLoginRequest {
    #[validate(length(min = 3, max = 30, message = "Username must be 3-30 characters"))]
    #[validate(custom(function = "validate_username_format", message = "Username must be alphanumeric only"))]
    username: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    #[validate(custom(function = "validate_password_complexity", message = "Password must contain uppercase, lowercase, and digit"))]
    password: String,
}

/// Tests for LoginRequest validation.
mod login_request_validation_tests {
    use super::*;

    #[test]
    fn test_valid_login_request() {
        let request = TestLoginRequest {
            username: "testuser".to_string(),
            password: "Password123".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_valid_username_with_underscore() {
        let request = TestLoginRequest {
            username: "test_user_123".to_string(),
            password: "Password123".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_username_too_short_min() {
        let request = TestLoginRequest {
            username: "ab".to_string(),
            password: "Password123".to_string(),
        };
        let result = request.validate();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(!request.validate().is_ok());
    }

    #[test]
    fn test_username_too_long() {
        let request = TestLoginRequest {
            username: "a".repeat(31),
            password: "Password123".to_string(),
        };
        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_username_with_special_chars() {
        let request = TestLoginRequest {
            username: "user@domain.com".to_string(),
            password: "Password123".to_string(),
        };
        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_username_with_space() {
        let request = TestLoginRequest {
            username: "user name".to_string(),
            password: "Password123".to_string(),
        };
        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_username_with_dash() {
        let request = TestLoginRequest {
            username: "user-name".to_string(),
            password: "Password123".to_string(),
        };
        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_username_with_dot() {
        let request = TestLoginRequest {
            username: "user.name".to_string(),
            password: "Password123".to_string(),
        };
        let result = request.validate();
        assert!(result.is_err());
    }

    #[test]
    fn test_username_exactly_3_chars() {
        let request = TestLoginRequest {
            username: "abc".to_string(),
            password: "Password123".to_string(),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn test_username_exactly_30_chars() {
        let request = TestLoginRequest {
            username: "a".repeat(30),
            password: "Password123".to_string(),
        };
        assert!(request.validate().is_ok());
    }
}

/// Tests for password complexity validation.
mod password_complexity_tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let result = validate_password_complexity("Password123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_without_uppercase() {
        let result = validate_password_complexity("password123");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_without_lowercase() {
        let result = validate_password_complexity("PASSWORD123");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_without_digit() {
        let result = validate_password_complexity("PasswordABC");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_with_special_chars_only() {
        // Special chars don't count as uppercase or lowercase
        let result = validate_password_complexity("!!!!!!!!!1");
        assert!(result.is_err());
    }

    #[test]
    fn test_password_all_types() {
        let result = validate_password_complexity("Passw0rd!");
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_single_char_each_type() {
        // Single char of each type should pass
        let result = validate_password_complexity("Aa1!");
        // Note: This passes the complexity check but might fail length check
        // The complexity function doesn't check length
        assert!(result.is_ok());
    }
}

/// Tests for username format validation.
mod username_format_tests {
    use super::*;

    #[test]
    fn test_valid_alphanumeric_username() {
        let result = validate_username_format("testuser123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_username_with_underscore() {
        let result = validate_username_format("test_user");
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_username_all_numbers() {
        let result = validate_username_format("12345");
        assert!(result.is_ok());
    }

    #[test]
    fn test_valid_username_all_underscores() {
        let result = validate_username_format("___");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_username_with_at() {
        let result = validate_username_format("user@domain");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_username_with_space() {
        let result = validate_username_format("user name");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_username_with_dash() {
        let result = validate_username_format("user-name");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_username_with_dot() {
        let result = validate_username_format("user.name");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_username_with_hash() {
        let result = validate_username_format("user#name");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_username_with_emoji() {
        let result = validate_username_format("user😀");
        assert!(result.is_err());
    }
}

/// Tests for password hashing with bcrypt.
mod password_hashing_tests {
    #[test]
    fn test_bcrypt_hash_produces_valid_hash() {
        let password = "TestPassword123";
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        
        // bcrypt hashes start with $2a$, $2b$, or $2y$
        assert!(hash.starts_with("$2") || hash.starts_with("$2b$") || hash.starts_with("$2y$"));
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

    #[test]
    fn test_bcrypt_different_costs_produce_different_hashes() {
        let password = "TestPassword123";
        let hash_low = bcrypt::hash(password, 4).unwrap();  // Low cost
        let hash_high = bcrypt::hash(password, 12).unwrap(); // High cost
        
        // Different costs should produce hashes with different prefixes indicating cost
        // Cost is encoded in the hash as $2a$04$... or $2a$12$...
        let cost_low = &hash_low[4..6];
        let cost_high = &hash_high[4..6];
        assert_ne!(cost_low, cost_high);
    }

    #[test]
    fn test_bcrypt_verify_empty_password_fails() {
        let password = "";
        let hash = bcrypt::hash("non-empty", bcrypt::DEFAULT_COST).unwrap();
        
        let is_valid = bcrypt::verify(password, &hash).unwrap();
        assert!(!is_valid);
    }

    #[test]
    fn test_bcrypt_verify_unicode_password() {
        let password = "密码Password123";
        let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST).unwrap();
        
        let is_valid = bcrypt::verify(password, &hash).unwrap();
        assert!(is_valid);
    }

    #[test]
    fn test_bcrypt_verify_long_password() {
        let password = "a".repeat(72); // bcrypt max is 72 bytes
        let hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST).unwrap();
        
        let is_valid = bcrypt::verify(&password, &hash).unwrap();
        assert!(is_valid);
    }
}
