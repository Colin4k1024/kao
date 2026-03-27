# Testing Patterns

**Analysis Date:** 2026-03-27

## Test Framework

**Runner:**
- Cargo test (built-in)
- Config: None (default behavior)

**Assertion library:** Standard assert macro

**Run Commands:**
```bash
cargo test          # Run all tests
cargo test -- --nocapture  # Show output
cargo test -- --test-threads=1  # Run tests sequentially
cargo test --lib    # Run only library tests
cargo test --test integration  # Run integration tests
```

## Test File Organization

**Location:**
- Co-located with source files: `{module}/mod.rs` contains tests
- Integration tests: `tests/` directory
- No separate test directories per module

**Naming:**
- Test modules: `#[cfg(test)] mod tests { ... }`
- Test functions: `test_{description}` or `test_{function_under_test}`

**Structure:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Arrange
        let service = Service::new();

        // Act
        let result = service.method().await;

        // Assert
        assert!(result.is_ok());
    }
}
```

**Fixtures and Factories:**
```rust
fn create_test_user() -> CreateUserRequest {
    CreateUserRequest {
        username: "test_user".to_string(),
        email: "test@example.com".to_string(),
        // ...
    }
}
```

**Location:** Define in test modules or `src/utils/test_utils.rs`

## Coverage

**Requirements:** None enforced currently

**View Coverage:** None configured

## Test Types

**Unit Tests:**
- Scope: Single function/method
- Approach: Direct function calls with mocked dependencies

**Integration Tests:**
- Scope: Multiple components working together
- Approach: Full application with test database

**E2E Tests:** Not used currently

## Common Patterns

**Async Testing:**
```rust
#[tokio::test]
async fn test_async() {
    let result = async_function().await;
    assert!(result.is_ok());
}
```

**Error Testing:**
```rust
#[test]
fn test_error_handling() {
    let result = function_that_fails();
    assert!(result.is_err());
    assert_eq!(format!("{}", result.unwrap_err()), "expected error");
}
```

---

*Testing analysis: 2026-03-27*
