---
name: "verification-before-completion"
description: "Ensures bug fixes and features are truly working before marking as complete. Verifies fix doesn't break other functionality. Activates after any fix or implementation."
---

# Verification Before Completion

The verification-before-completion skill ensures that fixes and features are genuinely working before considering them done. It prevents "it's probably fixed" syndrome and catches regressions early.

## When to Use

Activate verification-before-completion when:
- You think you've fixed a bug
- Implementation is complete
- User reports "it's still broken"
- Before marking any task as done
- Before merging any branch

## The Problem

"Working in my machine" isn't verification:
- ❌ "I ran it once and it didn't crash"
- ❌ "The test passed"
- ❌ "User said it works now"
- ❌ "Looks good to me"

## The Verification Process

### 1. Automated Verification

**Run the full test suite:**
```bash
# All unit tests
cargo test

# Integration tests
cargo test --test '*_integration'

# Specific feature tests
cargo test feature_name

# With coverage
cargo tarpaulin
```

**Verify no regressions:**
```bash
# Run all existing tests
# Expected: ALL PASS ✅

# Check test count hasn't decreased
cargo test --message-format=short | grep "test result"
# Expected: Same or more tests than before
```

### 2. Manual Verification

**Test the specific fix:**
```bash
# Reproduce original bug
./myapp --scenario that_broke_before

# Verify it's fixed
# Expected: Works correctly now
```

**Test edge cases:**
```bash
# Test boundary conditions
./myapp --test boundary_1
./myapp --test boundary_2
./myapp --test empty_input
./myapp --test max_value
```

**Test related functionality:**
```bash
# Test features that might be affected
./myapp --test login_flow
./myapp --test data_persistence
./myapp --test error_recovery
```

### 3. User-Facing Verification

**Simulate real user behavior:**
```bash
# Complete user journey
1. Login
2. Create resource
3. Modify it
4. Delete it
5. Logout

# Alternative flows
1. Login with invalid credentials
2. Try to access without auth
3. Handle network error mid-operation
```

**Verify the fix matches requirements:**
```markdown
## Requirements Check

Original requirement:
"Users with '+' in email must be able to login"

Verification:
✅ Email "user+tag@example.com" can login
✅ Email "user@example.com" still works
✅ Invalid email still rejected
✅ All auth flows work
```

### 4. Environment Verification

**Test in target environment:**
```bash
# Development
cargo run -- --test all

# Staging (if available)
curl -X POST https://staging.example.com/api/test

# Production check (if safe)
# Monitor error rates
# Check user reports
```

**Verify dependencies:**
```bash
# Check external services
curl https://api.example.com/health

# Check database
psql -c "SELECT 1"

# Check cache
redis-cli ping
```

## Verification Checklist

### For Bug Fixes

- [ ] **Reproduce the bug** - Can you still trigger it?
  - [ ] Original scenario works now
  - [ ] Edge cases handled
  
- [ ] **Verify the fix** - Does code change work?
  - [ ] Unit tests pass
  - [ ] Integration tests pass
  - [ ] Manual test succeeds
  
- [ ] **Check for side effects** - Did fix break anything?
  - [ ] All existing tests pass
  - [ ] Related features work
  - [ ] No new errors in logs
  
- [ ] **Add regression test** - Will it stay fixed?
  - [ ] Test that reproduces bug added
  - [ ] Test passes now
  - [ ] Test will catch if bug returns
  
- [ ] **Document** - Is the fix understood?
  - [ ] Root cause documented
  - [ ] Fix described
  - [ ] Verification steps recorded

### For Features

- [ ] **Requirements met** - Does it do what it should?
  - [ ] All acceptance criteria satisfied
  - [ ] User stories completed
  - [ ] Edge cases handled
  
- [ ] **Code quality** - Is it well-written?
  - [ ] Tests written (TDD)
  - [ ] Code follows conventions
  - [ ] No obvious bugs
  
- [ ] **Integration** - Does it work with rest of system?
  - [ ] Integration tests pass
  - [ ] No breaking changes
  - [ ] API contracts maintained
  
- [ ] **Performance** - Is it fast enough?
  - [ ] Response times acceptable
  - [ ] No obvious bottlenecks
  - [ ] Scales appropriately
  
- [ ] **Documentation** - Is it understandable?
  - [ ] Code is self-documenting
  - [ ] README updated if needed
  - [ ] API docs complete

## Verification Levels

### Level 1: Code Verification

**Checks:**
- Code compiles
- Type checks pass
- Linter passes

**Commands:**
```bash
cargo build
cargo check
cargo clippy
cargo fmt --check
```

### Level 2: Test Verification

**Checks:**
- All unit tests pass
- All integration tests pass
- Coverage maintained or improved

**Commands:**
```bash
cargo test
cargo test --test '*_integration'
cargo tarpaulin  # coverage
```

### Level 3: Functional Verification

**Checks:**
- Feature works as intended
- Bug is actually fixed
- Edge cases handled

**Commands:**
```bash
# Manual testing
./myapp --test scenario

# Automated smoke tests
cargo test --test smoke_tests
```

### Level 4: System Verification

**Checks:**
- All system components work together
- No regressions in other features
- Performance acceptable

**Commands:**
```bash
# End-to-end tests
cargo test --test e2e_tests

# Load tests (if applicable)
cargo test --test load_tests

# Staging deployment
./deploy.sh staging
```

### Level 5: Production Verification

**Checks:**
- Works in production
- No increase in errors
- User reports positive

**Commands:**
```bash
# Monitor metrics
curl -X GET /metrics

# Check error rates
tail -100 logs/error.log | grep -c ERROR

# User feedback
# (direct feedback from users)
```

## Verification Anti-Patterns

❌ **"I tested it once and it worked"**
- Test multiple times
- Test edge cases
- Test related functionality

❌ **"The unit test passes"**
- Run all tests, not just related ones
- Check for regressions

❌ **"It works on my machine"**
- Test in target environment
- Test in CI/CD pipeline

❌ **"User confirmed it works"**
- Verify independently
- Check for placebo effect

❌ **"Looks fine"**
- Verify objectively
- Check metrics/logs

❌ **"Time pressure"**
- Quick verification is better than none
- Document any shortcuts taken

## Documentation Template

```markdown
## Verification Report: [Bug Fix / Feature]

**Date:** [Date]
**Verifier:** [Who]

### Summary
Brief description of what was verified.

### Verification Steps

#### 1. Automated Tests
```
[Command and output]
Result: ✅ PASSED / ❌ FAILED
```

#### 2. Manual Testing
```
[Steps taken]
Result: ✅ PASSED / ❌ FAILED
```

#### 3. Edge Cases
```
[Edge cases tested]
Result: ✅ PASSED / ❌ FAILED
```

#### 4. Regression Testing
```
[Existing features tested]
Result: ✅ PASSED / ❌ FAILED
```

### Issues Found
[Any problems encountered]

### Verification Status
- ✅ **VERIFIED** - Ready for production
- ⚠️ **PARTIAL** - Known limitations documented
- ❌ **FAILED** - Needs more work

### Sign-off
[Approver signature/approval]
```

## Verification Techniques

### Smoke Tests

Quick tests of core functionality:
```bash
#!/bin/bash
# smoke_test.sh

echo "Running smoke tests..."

cargo test --test smoke_tests
if [ $? -eq 0 ]; then
    echo "✅ Smoke tests passed"
    exit 0
else
    echo "❌ Smoke tests failed"
    exit 1
fi
```

### Sanity Checks

Quick sanity verifications:
```bash
# Is it running?
pgrep -f myapp

# Are ports open?
netstat -tulpn | grep 8080

# Are there recent errors?
tail -100 logs/app.log | grep ERROR
```

### Health Checks

System health verification:
```bash
# Service health
curl http://localhost:8080/health

# Database connection
psql -c "SELECT 1" database

# External dependencies
curl http://api.external.com/health
```

## Verification Reporting

### Report Format

**Before claiming "done":**
```markdown
## Verification Report: Fix for Issue #123

**Date:** 2024-01-15
**Tester:** Agent

### Bug Description
Users with "+" in email could not login.

### Fix Applied
Updated email validation regex to allow "+" character.

### Verification Steps

#### 1. Regression Tests
```bash
$ cargo test
   Compiling myapp v0.1.0
    Finished test [unoptimized]
  test result: all 47 tests passed ✅
```

#### 2. Manual Verification
```bash
$ ./myapp login user+tag@example.com
Login successful ✅
```

#### 3. Edge Case Testing
```bash
$ ./myapp login "user+test+multiple@example.com"
Login successful ✅

$ ./myapp login "invalid+email"
Validation error ✅ (expected)
```

#### 4. Production Metrics
```
Error rate: 0% (was 0.5%)
Login success rate: 100% (was 99.5%)
User complaints: 0 (was 3)
```

### Verification Status

✅ **VERIFIED AND COMPLETE**
- All tests pass
- Manual verification successful
- No regressions detected
- Metrics improved

**Ready for:** Deployment
```

## Success Criteria

Verification is successful when:
- ✅ All automated tests pass
- ✅ Manual tests succeed
- ✅ Edge cases handled
- ✅ No regressions
- ✅ Metrics improved
- ✅ Documentation complete
- ✅ Someone else can verify your work

## Remember

**Verification is not optional:**
- "It's probably fixed" is not verification
- "Tests pass" is not complete verification
- "User says it works" is not independent verification

**Verification has levels:**
- Code compiles
- Tests pass
- Feature works
- System integrates
- Production succeeds

**When in doubt, verify:**
- Better to find problems now than in production
- Extra verification time saves debugging time later
- Document your verification for others
