use crate::common::security::validate_password;
use crate::common::security::PasswordPolicy;

#[test]
fn test_password_policy_default() {
    let policy = PasswordPolicy::default();
    assert_eq!(policy.min_length, 8);
    assert_eq!(policy.max_length, 128);
    assert_eq!(policy.min_lowercase, 1);
    assert_eq!(policy.min_uppercase, 1);
    assert_eq!(policy.min_digits, 1);
    assert_eq!(policy.min_special, 1);
    assert!(policy.no_whitespace);
}

#[test]
fn test_validate_password_valid() {
    let policy = PasswordPolicy::default();
    assert!(validate_password("Password1!", &policy).is_ok());
}

#[test]
fn test_validate_password_too_short() {
    let policy = PasswordPolicy::default();
    let result = validate_password("Pass!", &policy);
    assert!(result.is_err());
}

#[test]
fn test_validate_password_common() {
    let policy = PasswordPolicy::default();
    let result = validate_password("password", &policy);
    assert!(result.is_err());
}

#[test]
fn test_validate_password_no_whitespace() {
    let policy = PasswordPolicy::default();
    let result = validate_password("Pass word1!", &policy);
    assert!(result.is_err());
}
