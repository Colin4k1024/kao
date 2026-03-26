---
name: "test-driven-development"
description: "Enforces RED-GREEN-REFACTOR cycle: write failing test first, watch it fail, write minimal code to pass, then refactor. Deletes code written before tests. Activates during implementation."
---

# Test-Driven Development (TDD)

The test-driven-development skill enforces the RED-GREEN-REFACTOR cycle, ensuring tests are written before code and quality is maintained throughout development.

## When to Use

Activate test-driven-development when:
- Writing any new feature
- Fixing bugs
- Adding functionality
- Refactoring code
- During implementation phase

## The RED-GREEN-REFACTOR Cycle

### RED - Write the Failing Test

**Principle:** Write a test that describes the behavior you want before writing any implementation code.

**Why RED first?**
- Forces you to think about the interface before implementation
- Catches bugs before they happen
- Ensures testability
- Prevents "just ship it" mentality

**How to RED:**
```bash
# 1. Write a test that describes expected behavior
cat > tests/my_feature_test.rs << 'EOF'
#[test]
fn test_feature_returns_expected_value() {
    let result = my_function(input);
    assert_eq!(result, expected_value);
}
EOF

# 2. Run the test - it should COMPILE but FAIL
cargo test test_feature_returns_expected_value
# Expected output:
# test result: FAILED. 1 failed, 0 passed.
```

**What to test:**
- Happy path (normal operation)
- Edge cases (empty, null, zero, max values)
- Error conditions (invalid input, network failure)
- Boundary conditions

**Example RED:**
```rust
// Before writing any code:
#[test]
fn test_user_can_login_with_valid_credentials() {
    let credentials = Credentials {
        email: "user@example.com",
        password: "correct_password"
    };
    
    let result = login(credentials);
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap().email, "user@example.com");
}
```

### GREEN - Write Minimal Code to Pass

**Principle:** Write the minimum code needed to make the test pass. No optimizations, no "improvements", no extra features.

**Why GREEN?**
- Focuses on what's needed, not what might be needed
- Prevents over-engineering
- Gets to working code fast
- Forces simplicity

**How to GREEN:**
```bash
# 1. Write minimal implementation
cat > src/my_feature.rs << 'EOF'
pub fn my_function(input: Type) -> Output {
    // Minimum code to pass test
    // Hardcode if needed
}
EOF

# 2. Run the test - it should PASS
cargo test test_feature_returns_expected_value
# Expected output:
# test result: ok. 1 passed, 0 failed.
```

**GREEN rules:**
- ✅ Do the simplest thing that could work
- ✅ Hardcode values if needed
- ✅ Copy-paste before abstracting
- ❌ Don't optimize (yet)
- ❌ Don't add features not in tests
- ❌ Don't refactor (yet)

**Example GREEN:**
```rust
// Minimal implementation that passes the test:
pub fn login(credentials: Credentials) -> Result<User, AuthError> {
    // Simple hardcoded check for now
    if credentials.email == "user@example.com" 
       && credentials.password == "correct_password" {
        Ok(User { 
            email: credentials.email,
            id: 1 
        })
    } else {
        Err(AuthError::InvalidCredentials)
    }
}
```

### REFACTOR - Clean Up (Optional)

**Principle:** Now that tests pass, improve code quality without changing behavior.

**Why REFACTOR?**
- Cleans up "quick and dirty" code
- Removes duplication
- Improves readability
- Maintains design integrity

**When to REFACTOR:**
- When code is duplicated
- When names are unclear
- When logic is convoluted
- When you're about to add new tests (clean slate)

**When NOT to REFACTOR:**
- When tests are failing
- When under time pressure
- When in production emergency
- When code is working and stable

**REFACTOR rules:**
- ✅ Make small, incremental changes
- ✅ Run tests after each change
- ✅ Keep tests passing throughout
- ❌ Don't add new functionality
- ❌ Don't change behavior
- ❌ Don't skip tests

**Example REFACTOR:**
```rust
// Before refactor (works but messy):
pub fn login(credentials: Credentials) -> Result<User, AuthError> {
    if credentials.email == "user@example.com" 
       && credentials.password == "correct_password" {
        Ok(User { 
            email: credentials.email,
            id: 1 
        })
    } else {
        Err(AuthError::InvalidCredentials)
    }
}

// After refactor (cleaner):
const ADMIN_EMAIL: &str = "user@example.com";
const ADMIN_PASSWORD: &str = "correct_password";
const ADMIN_ID: i32 = 1;

pub fn login(credentials: Credentials) -> Result<User, AuthError> {
    if credentials.email == ADMIN_EMAIL 
       && credentials.password == ADMIN_PASSWORD {
        Ok(User { 
            email: credentials.email,
            id: ADMIN_ID 
        })
    } else {
        Err(AuthError::InvalidCredentials)
    }
}
```

## Test Structure

### Anatomy of a Good Test

```rust
#[test]
fn test_name_describes_behavior() {
    // Arrange - set up test data
    let input = create_test_data();
    let expected = expected_output();
    
    // Act - execute the function
    let result = function_under_test(input);
    
    // Assert - verify the result
    assert_eq!(result, expected);
}
```

### AAA Pattern (Arrange-Act-Assert)

**Arrange:** Set up everything needed for the test
- Create test objects
- Set up mock/stub data
- Configure environment

**Act:** Execute the code being tested
- Call the function
- Perform the operation

**Assert:** Verify the results
- Check return values
- Verify state changes
- Ensure side effects

### Test Naming

Good test names describe behavior:
```
✅ test_user_cannot_login_with_wrong_password
✅ test_login_returns_user_id_on_success
✅ test_empty_list_returns_zero_items

❌ test_login
❌ test_function
❌ test_case1
```

## What to Test

### Test Boundaries

**Test from the outside:**
- Public API (integration tests)
- User-facing behavior
- System interactions

**Don't test internals:**
- Private functions
- Internal implementation details
- Code that might change

### What Makes a Good Test?

**Tests behavior, not implementation:**
```rust
// ✅ Good - tests behavior
#[test]
fn test_notification_appears_in_user_inbox() {
    send_notification(user_id, message);
    
    let inbox = get_user_inbox(user_id);
    assert!(inbox.contains(message));
}

// ❌ Bad - tests implementation
#[test]
fn test_notification_saved_to_database() {
    // This tests HOW, not WHAT
    let count = database.count("notifications");
    assert_eq!(count, 1);
}
```

**Tests are independent:**
```rust
// ✅ Good - no dependencies
#[test]
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

// ❌ Bad - depends on order
#[test]
fn test_second_addition() {
    assert_eq!(3 + 3, 6); // What if test 1 failed?
}
```

**Tests are deterministic:**
```rust
// ✅ Good - always same result
#[test]
fn test_user_id_is_sequential() {
    let user1 = create_user("Alice");
    let user2 = create_user("Bob");
    
    assert!(user2.id > user1.id);
}

// ❌ Bad - flaky test
#[test]
fn test_response_time_under_100ms() {
    let time = measure_request();
    assert!(time < 100); // Flaky - depends on system load
}
```

## Testing Anti-Patterns

See companion skill: `testing-anti-patterns` for detailed examples of what NOT to do.

Common mistakes to avoid:

❌ **Testing implementation, not behavior**
❌ **Tightly coupled tests**
❌ **Testing private functions**
❌ **Assertion roulette (unclear failures)**
❌ **Sensitive tests (depends on environment)**
❌ **Skip tests when in a hurry**

## Test Levels

### Unit Tests

**Purpose:** Test individual functions/modules in isolation

**Characteristics:**
- Fast (milliseconds)
- No external dependencies
- In-memory
- Highly specific

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_total() {
        let items = vec![1.0, 2.0, 3.0];
        assert_eq!(calculate_total(items), 6.0);
    }
}
```

### Integration Tests

**Purpose:** Test how components work together

**Characteristics:**
- Slower than unit tests
- Real dependencies
- Test database
- Test external services

**Example:**
```rust
#[tokio::test]
async fn test_user_registration_flow() {
    let db = setup_test_database().await;
    let user_service = UserService::new(db);
    
    let result = user_service.register("user@test.com").await;
    
    assert!(result.is_ok());
    assert!(user_exists_in_db("user@test.com").await);
}
```

### End-to-End Tests

**Purpose:** Test complete user journeys

**Characteristics:**
- Slowest
- Full system
- Real environment
- Simulates user

**Example:**
```rust
#[tokio::test]
async fn test_user_can_login_and_view_dashboard() {
    let client = TestClient::new();
    
    // Register
    client.register("user@test.com", "password").await;
    
    // Login
    client.login("user@test.com", "password").await;
    
    // View dashboard
    let response = client.get("/dashboard").await;
    assert!(response.contains("Welcome"));
}
```

## Running Tests

### Run All Tests
```bash
cargo test
```

### Run Specific Test
```bash
cargo test test_user_can_login
```

### Run Tests with Output
```bash
cargo test -- --nocapture
```

### Run Tests by Pattern
```bash
cargo test user
```

### Run Tests by Type
```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test '*_integration'
```

### Run Tests with Coverage
```bash
cargo tarpaulin
```

## Test-Driven Bug Fixes

When fixing bugs:

1. **Write a test that reproduces the bug**
   ```rust
   #[test]
   fn test_login_fails_with_wrong_password() {
       let credentials = Credentials {
           email: "user@test.com",
           password: "wrong_password"
       };
       
       let result = login(credentials);
       assert!(result.is_err());
   }
   ```

2. **Run test - verify it FAILS**

3. **Fix the bug**

4. **Run test - verify it PASSES**

5. **Don't remove the test!** - It's a regression test now

## Success Criteria

TDD is successful when:
- ✅ Tests written before code (RED first)
- ✅ Minimal code to pass tests (GREEN)
- ✅ Code cleaned up safely (REFACTOR)
- ✅ All tests passing
- ✅ Tests are deterministic
- ✅ Tests are maintainable
- ✅ Code behavior matches tests

## Remember

**RED** - Think before you code
**GREEN** - Focus on what's needed
**REFACTOR** - Clean without breaking

The cycle is not:
- Write code, then tests
- Write tests, then all features
- Skip tests because "we're in a hurry"
- Leave tests failing
- Refactor without tests
