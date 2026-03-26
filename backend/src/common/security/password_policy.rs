use serde::{Deserialize, Serialize};

/// Password policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordPolicy {
    pub min_length: usize,
    pub max_length: usize,
    pub min_lowercase: usize,
    pub min_uppercase: usize,
    pub min_digits: usize,
    pub min_special: usize,
    pub no_whitespace: bool,
    pub common_passwords: Vec<String>,
}

impl Default for PasswordPolicy {
    fn default() -> Self {
        Self {
            min_length: 8,
            max_length: 128,
            min_lowercase: 1,
            min_uppercase: 1,
            min_digits: 1,
            min_special: 1,
            no_whitespace: true,
            common_passwords: vec![
                "password".to_string(),
                "123456".to_string(),
                "qwerty".to_string(),
                "abc123".to_string(),
                "password123".to_string(),
                "admin".to_string(),
                "letmein".to_string(),
                "welcome".to_string(),
                "monkey".to_string(),
                "dragon".to_string(),
                "master".to_string(),
                "sunshine".to_string(),
                "ashley".to_string(),
                "bailey".to_string(),
                "shadow".to_string(),
                "123456789".to_string(),
                "12345678".to_string(),
                "1234567".to_string(),
                "1234567890".to_string(),
                "12345".to_string(),
                "password1".to_string(),
                "qwerty123".to_string(),
                "letmein1".to_string(),
                "admin123".to_string(),
                "root".to_string(),
                "toor".to_string(),
                "pass".to_string(),
                "test".to_string(),
                "user".to_string(),
                "guest".to_string(),
                "hello".to_string(),
                "world".to_string(),
                "default".to_string(),
                "secure".to_string(),
                "Password1".to_string(),
                "Password123".to_string(),
                "P@ssw0rd".to_string(),
            ],
        }
    }
}

/// Password validation error
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PasswordValidationError {
    TooShort(usize),
    TooLong(usize),
    MissingLowercase(usize),
    MissingUppercase(usize),
    MissingDigit(usize),
    MissingSpecial(usize),
    ContainsWhitespace,
    CommonPassword,
}

impl std::fmt::Display for PasswordValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordValidationError::TooShort(min) => {
                write!(f, "Password must be at least {} characters", min)
            }
            PasswordValidationError::TooLong(max) => {
                write!(f, "Password must be at most {} characters", max)
            }
            PasswordValidationError::MissingLowercase(count) => {
                write!(
                    f,
                    "Password must contain at least {} lowercase letter(s)",
                    count
                )
            }
            PasswordValidationError::MissingUppercase(count) => {
                write!(
                    f,
                    "Password must contain at least {} uppercase letter(s)",
                    count
                )
            }
            PasswordValidationError::MissingDigit(count) => {
                write!(f, "Password must contain at least {} digit(s)", count)
            }
            PasswordValidationError::MissingSpecial(count) => {
                write!(
                    f,
                    "Password must contain at least {} special character(s)",
                    count
                )
            }
            PasswordValidationError::ContainsWhitespace => {
                write!(f, "Password must not contain whitespace")
            }
            PasswordValidationError::CommonPassword => {
                write!(f, "Password is too common and easily guessable")
            }
        }
    }
}

/// Result type for password validation
pub type PasswordValidationResult = Result<(), PasswordValidationError>;

/// Validate a password against the policy
pub fn validate_password(password: &str, policy: &PasswordPolicy) -> PasswordValidationResult {
    // Check length
    if password.len() < policy.min_length {
        return Err(PasswordValidationError::TooShort(policy.min_length));
    }
    if password.len() > policy.max_length {
        return Err(PasswordValidationError::TooLong(policy.max_length));
    }

    // Count character types
    let mut lowercase_count = 0;
    let mut uppercase_count = 0;
    let mut digit_count = 0;
    let mut special_count = 0;

    for c in password.chars() {
        if c.is_ascii_lowercase() {
            lowercase_count += 1;
        } else if c.is_ascii_uppercase() {
            uppercase_count += 1;
        } else if c.is_ascii_digit() {
            digit_count += 1;
        } else if !c.is_whitespace() {
            special_count += 1;
        }

        if c.is_whitespace() && policy.no_whitespace {
            return Err(PasswordValidationError::ContainsWhitespace);
        }
    }

    // Check lowercase requirement
    if lowercase_count < policy.min_lowercase {
        return Err(PasswordValidationError::MissingLowercase(
            policy.min_lowercase,
        ));
    }

    // Check uppercase requirement
    if uppercase_count < policy.min_uppercase {
        return Err(PasswordValidationError::MissingUppercase(
            policy.min_uppercase,
        ));
    }

    // Check digit requirement
    if digit_count < policy.min_digits {
        return Err(PasswordValidationError::MissingDigit(policy.min_digits));
    }

    // Check special character requirement
    if special_count < policy.min_special {
        return Err(PasswordValidationError::MissingSpecial(policy.min_special));
    }

    // Check if password is in common passwords list
    if policy.common_passwords.contains(&password.to_string()) {
        return Err(PasswordValidationError::CommonPassword);
    }

    Ok(())
}

/// Check if password contains the username or parts of it
pub fn check_username_in_password(password: &str, username: &str) -> PasswordValidationResult {
    if password.to_lowercase().contains(&username.to_lowercase()) {
        Err(PasswordValidationError::CommonPassword)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let policy = PasswordPolicy::default();
        assert!(validate_password("Passw0rd!", &policy).is_ok());
    }

    #[test]
    fn test_too_short_password() {
        let policy = PasswordPolicy::default();
        let result = validate_password("Short1!", &policy);
        assert!(matches!(result, Err(PasswordValidationError::TooShort(_))));
    }

    #[test]
    fn test_missing_lowercase() {
        let policy = PasswordPolicy::default();
        let result = validate_password("PASSWORD1!", &policy);
        assert!(matches!(
            result,
            Err(PasswordValidationError::MissingLowercase(_))
        ));
    }

    #[test]
    fn test_missing_uppercase() {
        let policy = PasswordPolicy::default();
        let result = validate_password("password1!", &policy);
        assert!(matches!(
            result,
            Err(PasswordValidationError::MissingUppercase(_))
        ));
    }

    #[test]
    fn test_missing_digit() {
        let policy = PasswordPolicy::default();
        let result = validate_password("Password!", &policy);
        assert!(matches!(
            result,
            Err(PasswordValidationError::MissingDigit(_))
        ));
    }

    #[test]
    fn test_missing_special() {
        let policy = PasswordPolicy::default();
        let result = validate_password("Password1", &policy);
        assert!(matches!(
            result,
            Err(PasswordValidationError::MissingSpecial(_))
        ));
    }

    #[test]
    fn test_common_password() {
        let policy = PasswordPolicy::default();
        let result = validate_password("password", &policy);
        assert!(matches!(
            result,
            Err(PasswordValidationError::CommonPassword)
        ));
    }

    #[test]
    fn test_whitespace_in_password() {
        let policy = PasswordPolicy {
            no_whitespace: true,
            ..PasswordPolicy::default()
        };
        let result = validate_password("Pass word1!", &policy);
        assert!(matches!(
            result,
            Err(PasswordValidationError::ContainsWhitespace)
        ));
    }

    #[test]
    fn test_username_in_password() {
        let result = check_username_in_password("MyUsername123!", "myusername");
        assert!(matches!(
            result,
            Err(PasswordValidationError::CommonPassword)
        ));
    }
}
