---
name: "systematic-debugging"
description: "Applies 4-phase root cause analysis: gather data, form hypothesis, test hypothesis, solve and verify. Includes root-cause-tracing, defense-in-depth, and condition-based-waiting techniques. Activates for any bug."
---

# Systematic Debugging

The systematic-debugging skill applies a disciplined approach to finding and fixing bugs through structured root cause analysis, preventing guesswork and ensuring fixes actually solve the problem.

## When to Use

Activate systematic-debugging when:
- Bug is reported
- Test is failing
- Feature not working as expected
- Error occurs in production
- Something breaks unexpectedly

## The 4-Phase Debugging Process

### Phase 1: Gather Data

**Goal:** Understand the problem fully before attempting fixes.

**Questions to answer:**
- What is the expected behavior?
- What is the actual behavior?
- When does it happen? (time, frequency)
- Where does it happen? (code location, environment)
- Who does it affect? (users, systems)
- How can it be reproduced?

**How to gather data:**

**1. Read error messages carefully:**
```
❌ Database connection failed: "Connection refused"

✅ What's the full error?
✅ What database? Where is it hosted?
✅ What credentials are being used?
✅ Has it ever worked?
✅ What changed recently?
```

**2. Check logs:**
```bash
# System logs
journalctl -u myservice --since "1 hour ago"

# Application logs
tail -f logs/app.log

# Error logs
grep -i error logs/app.log | tail -50
```

**3. Reproduce the bug:**
```bash
# Try to reproduce
./myapp --input problematic_data

# With debugging
RUST_BACKTRACE=1 cargo run
```

**4. Gather context:**
- User report details
- System state at time of failure
- Recent changes (commits, deployments)
- Environment differences
- Related issues

**Document what you know:**
```markdown
## Bug: Login fails for some users

**Symptoms:**
- Users with email containing "+" cannot login
- Example: "user+tag@example.com" fails
- Example: "user@example.com" works

**Reproduction:**
1. Create user with email "user+tag@example.com"
2. Attempt login
3. Result: 401 Unauthorized

**Error:**
[Error message here]

**When it started:**
- Last 24 hours
- After deployment of commit abc123

**Affected users:**
- ~5% of users (those with "+" in email)
```
---

### Phase 2: Form Hypothesis

**Goal:** Develop testable theories about the root cause.

**Don't assume - hypothesize:**
```
❌ "It's probably a database issue"
❌ "The server must be overloaded"
❌ "This always happens"

✅ "If the bug is in the email parsing, then I should see it fail at that exact line"
✅ "If it's a connection timeout, then increasing the timeout should help"
```

**How to form good hypotheses:**

**1. Consider the "NUTS" framework:**
- **N**ormal - What should happen?
- **U**nder what conditions - When does it break?
- **T**heory - What could cause this?
- **S**pecific - Where exactly does it fail?

**2. List possible causes:**
```markdown
## Possible Causes

1. Email parsing fails on "+" character
2. Database doesn't accept "+" in email field
3. Auth service has regex that excludes "+"
4. Frontend form validation blocks "+"
5. Email verification logic rejects "+"
```

**3. Rank by likelihood:**
```markdown
## Most Likely (Based on Error Message)

1. **Auth regex excludes "+"** - Error mentions "invalid format" - likelihood: HIGH
2. Email parsing fails - likelihood: MEDIUM  
3. Database rejects - likelihood: LOW (other emails work)

## Tests to Verify:

Test 1: Check auth regex
Test 2: Test email parsing function  
Test 3: Check database schema
```

**4. Form specific hypothesis:**
```markdown
## Hypothesis

"If the bug is in the email validation regex (cause #1), 
then:
- The regex should be rejecting emails with '+'
- Fixing the regex should allow '+' emails
- The fix should not break valid email formats"
```

### Phase 3: Test Hypothesis

**Goal:** Verify or disprove your hypothesis through controlled experiments.

**Design experiments:**

**1. Isolate the variable:**
```bash
# Instead of:
./app --all-features --with-database --with-cache

# Try:
./app --only-email-validation --test-email "user+tag@example.com"
```

**2. Test the hypothesis directly:**
```rust
#[test]
fn test_email_with_plus_sign() {
    let email = "user+tag@example.com";
    let result = validate_email(email);
    assert!(result.is_ok(), "Email with + should be valid");
}
```

**3. Binary search:**
```markdown
## Binary Search Strategy

If bug appeared in commit range 100-200:
- Test commit 150
- Works? → Bug is in 150-200
- Broken? → Bug is in 100-150
- Continue halving until you isolate the exact commit
```

**4. Compare working vs. broken:**
```bash
# Working user
echo "user@example.com" | ./validate

# Broken user  
echo "user+tag@example.com" | ./validate

# Compare output
```

**Test systematically:**

**Step 1: Reproduce the bug**
```bash
cargo test test_email_with_plus_sign
# Output: FAILED
```

**Step 2: Narrow down location**
```bash
# Add logging
println!("Email before parsing: {}", email);
let parsed = parse_email(&email);
println!("Email after parsing: {:?}", parsed);
```

**Step 3: Test the specific theory**
```bash
# Check if it's the regex
grep -n "email.*regex" src/auth/validation.rs
# Line 45: let email_regex = Regex::new(r"^[\w]+@[\w]+\.[\w]+").unwrap();
# NOTE: Missing + in character class!
```

**Step 4: Verify with test**
```bash
# Test your fix
# Change regex to: r"^[\w+]+@[\w]+\.[\w]+"

cargo test test_email_with_plus_sign  
# Output: PASSED ✅
```

### Phase 4: Solve and Verify

**Goal:** Implement the fix and confirm it solves the problem without breaking anything.

**1. Implement the fix:**
```rust
// Before
let email_regex = Regex::new(r"^[\w]+@[\w]+\.[\w]+").unwrap();

// After  
let email_regex = Regex::new(r"^[\w.+-]+@[\w.-]+\.[\w]+$").unwrap();
```

**2. Verify the fix works:**
```bash
# Test the specific case
cargo test test_email_with_plus_sign
# Output: PASSED ✅

# Test edge cases
cargo test test_email_validation
# Output: All tests pass ✅
```

**3. Run regression tests:**
```bash
# All existing tests
cargo test
# Output: All tests pass ✅

# Integration tests
cargo test --test '*_integration'
# Output: All integration tests pass ✅
```

**4. Test in environment:**
```bash
# Local
./myapp --test-login user+tag@example.com
# Output: Login successful ✅

# Staging (if available)
# Deploy and test
```

**5. Add regression test:**
```rust
#[test]
fn test_email_validation_allows_plus_sign() {
    assert!(validate_email("user+tag@example.com").is_ok());
    assert!(validate_email("user+test+tag@example.com").is_ok());
}

#[test]  
fn test_email_validation_rejects_invalid() {
    assert!(validate_email("invalid-email").is_err());
}
```

**6. Document the fix:**
```markdown
## Fix: Email validation now allows "+" character

**Problem:** Emails containing "+" (e.g., "user+tag@example.com") were rejected.

**Root Cause:** Email regex `^[\w]+@[\w]+\.[\w]+` didn't include "+" character.

**Solution:** Updated regex to `^[\w.+-]+@[\w.-]+\.[\w]+$`

**Files changed:**
- src/auth/validation.rs (line 45)

**Tests added:**
- test_email_validation_allows_plus_sign
- test_email_validation_rejects_invalid

**Verified:**
- Unit tests pass
- Integration tests pass
- Manually tested in staging
```

## Debugging Techniques

### Root Cause Tracing

See companion skill: `root-cause-tracing` for detailed techniques.

**Key techniques:**
- 5 Whys (ask "why" until you reach root cause)
- Fishbone diagrams (categorize causes)
- Fault tree analysis (boolean logic of failures)

### Defense in Depth

See companion skill: `defense-in-depth` for detailed techniques.

**Key principles:**
- Multiple layers of protection
- Fail-safe defaults
- Validate at boundaries
- Log for forensics

### Condition-Based Waiting

See companion skill: `condition-based-waiting` for detailed techniques.

**For timing/race condition issues:**
- Wait for specific conditions, not arbitrary timeouts
- Use polling with exponential backoff
- Implement timeouts with clear failure modes

## Common Debugging Mistakes

❌ **Fixing symptoms, not causes**
```
❌ "User gets error" → "Hide error message"
✅ "User gets error" → "Fix the actual problem"
```

❌ **Making random changes**
```
❌ Change A, test, change B, test, change C, test
✅ Form hypothesis, test hypothesis, verify
```

❌ **Not testing the fix**
```
❌ "Seems to work now"
✅ "All tests pass, manually verified, regression test added"
```

❌ **Skipping documentation**
```
❌ Fixed it, moving on
✅ Document root cause, fix, and verification
```

❌ **Not adding regression tests**
```
❌ Fixed bug, never to return
✅ Fixed bug, added test to prevent return
```

## Debugging Tools

### Command Line

```bash
# strace (Linux) - trace system calls
strace -e trace=read,write ./myapp

# ltrace (Linux) - trace library calls
ltrace ./myapp

# dtrace (macOS) - dynamic tracing
sudo dtrace -n 'syscall::read:entry { @[execname] = count(); }'
```

### Language-Specific

**Rust:**
```bash
# Backtrace
RUST_BACKTRACE=1 cargo run

# Debug symbols
RUSTFLAGS="-C debug-assertions=y" cargo build

# Memory leaks
cargo install cargo-valgrind
cargo valgrind test
```

### Logging

```rust
// Add structured logging
tracing::info!("Processing email: {}", email);
tracing::debug!("Parsed result: {:?}", result);
tracing::error!("Failed: {:?}", error);
```

### Debuggers

**Rust (lldb/gdb):**
```bash
rust-lldb target/debug/myapp
# (lldb) breakpoint set --name main
# (lldb) run
```

## Systematic Debugging Checklist

- [ ] **Gather Data**
  - [ ] Can you reproduce the bug?
  - [ ] What are exact error messages?
  - [ ] What logs show?
  - [ ] When did it start?
  - [ ] What changed recently?
  - [ ] Who does it affect?

- [ ] **Form Hypothesis**
  - [ ] Listed all possible causes?
  - [ ] Ranked by likelihood?
  - [ ] Formed specific, testable theory?
  - [ ] Identified where to look?

- [ ] **Test Hypothesis**
  - [ ] Designed controlled experiment?
  - [ ] Isolated the variable?
  - [ ] Ran the test?
  - [ ] Confirmed or disproved hypothesis?

- [ ] **Solve and Verify**
  - [ ] Implemented the fix?
  - [ ] Verified fix works?
  - [ ] Ran all tests?
  - [ ] Added regression test?
  - [ ] Documented the fix?
  - [ ] Tested in environment?

## Success Criteria

Debugging is successful when:
- ✅ Root cause identified (not just symptoms)
- ✅ Fix actually solves the problem
- ✅ No new bugs introduced
- ✅ Regression test added
- ✅ Documentation complete
- ✅ Problem won't recur

## Remember

**Phase 1:** Understand the problem completely  
**Phase 2:** Form a testable hypothesis  
**Phase 3:** Prove or disprove it  
**Phase 4:** Fix it right and verify

**Don't:**
- Guess
- Fix randomly
- Skip testing
- Forget to document
- Leave tests broken

**Do:**
- Be systematic
- Test assumptions
- Verify thoroughly
- Learn and prevent
