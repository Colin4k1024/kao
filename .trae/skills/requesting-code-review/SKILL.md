---
name: "requesting-code-review"
description: "Reviews code against plan for correctness, bugs, security issues, and adherence to standards. Reports issues by severity. Activates between tasks or before merging."
---

# Requesting Code Review

The requesting-code-review skill ensures code quality by systematically reviewing changes against specifications, identifying issues, and reporting them by severity.

## When to Use

Activate requesting-code-review when:
- Completing a logical group of tasks
- Before merging any branch
- Between implementation phases
- User requests review
- Want to catch issues early

## Pre-Review Checklist

Before starting review:

- [ ] All planned changes are committed
- [ ] Tests pass locally
- [ ] Code compiles/builds
- [ ] No uncommitted changes
- [ ] Review scope is clear

## The Review Process

### 1. Understand Context

Review the plan and design:
- What was the goal?
- What tasks were completed?
- What are acceptance criteria?
- What are non-goals?

### 2. Scope the Changes

```bash
# See what changed
git diff main...feature-branch --stat

# List modified files
git diff main...feature-branch --name-only

# Review commit history
git log main..feature-branch --oneline
```

Identify:
- Files changed
- Lines added/removed
- New dependencies
- Configuration changes

### 3. Systematic Review

#### A. Correctness Review

**Check:**
- Does the code do what it claims?
- Are edge cases handled?
- Is error handling present?
- Are there obvious bugs?

**Look for:**
- Logic errors
- Incorrect assumptions
- Missing null checks
- Off-by-one errors
- Race conditions

**Example command:**
```bash
# Review specific file
git diff main...feature-branch -- src/feature/model.rs
```

#### B. Security Review

**Check:**
- SQL injection vulnerabilities
- XSS vulnerabilities
- Authentication/authorization issues
- Sensitive data exposure
- Input validation

**Look for:**
- Direct SQL concatenation (use parameterized queries)
- Unsanitized user input
- Missing auth checks
- Secrets in code (use env vars)
- Improper error messages

**Example:**
```bash
# Check for TODO comments that might indicate missing security
git diff main...feature-branch | grep -i TODO
```

#### C. Performance Review

**Check:**
- N+1 query problems
- Missing indexes
- Inefficient algorithms
- Memory leaks
- Unnecessary allocations

**Look for:**
- Loops in queries
- Missing pagination
- Large data loading
- Unbounded recursion

#### D. Code Quality Review

**Check:**
- Follows project conventions
- Code is readable
- Functions are reasonably sized
- No code duplication
- Tests are adequate

**Look for:**
- Long functions (split them)
- Deep nesting (extract methods)
- Magic numbers (use constants)
- Comments explaining "why" not "what"
- Missing tests

### 4. Report by Severity

Group issues by severity:

#### Critical Issues ❌

Block progress. Must fix before merging.

Examples:
- Security vulnerabilities
- Data loss risks
- Complete functionality broken
- Breaking existing features

```markdown
### ❌ Critical Issues

**Issue:** SQL Injection vulnerability in user search
**File:** `src/api/user/handlers.rs:45`
**Problem:** Direct string concatenation in SQL query
**Impact:** Attacker can execute arbitrary SQL

**Issue:** Authentication bypass
**File:** `src/middleware/auth.rs:23`
**Problem:** Missing auth check allows unauthorized access
**Impact:** Security breach

Must fix before proceeding.
```

#### Major Issues ⚠️

Should fix, but doesn't block.

Examples:
- Significant bugs
- Major code smells
- Missing error handling
- Poor performance in hot paths

```markdown
### ⚠️ Major Issues

**Issue:** No error handling for database failures
**File:** `src/services/user.rs:67`
**Problem:** Database errors propagate as unhandled exceptions
**Suggestion:** Add try-catch and user-friendly error messages

**Issue:** N+1 query problem
**File:** `src/services/report.rs:34`
**Problem:** Loading users then loading each user's posts separately
**Suggestion:** Use JOIN or batch query
```

#### Minor Issues 💡

Nice to fix, low priority.

Examples:
- Code style inconsistencies
- Minor duplication
- Suboptimal naming
- Missing comments

```markdown
### 💡 Minor Issues

**Issue:** Function name unclear
**File:** `src/utils/helper.rs:12`
**Problem:** `fn x()` doesn't convey purpose
**Suggestion:** Rename to `calculate_user_score()`

**Issue:** Code duplication
**Files:** `src/api/user/handlers.rs`, `src/api/admin/handlers.rs`
**Problem:** Identical validation logic duplicated
**Suggestion:** Extract to shared `validate_email()` function
```

### 5. Provide Recommendations

Don't just report problems—suggest solutions:

**Good:**
```markdown
**Suggestion:** Extract validation logic:

```rust
// Current (duplicated in 3 places)
if email.contains('@') && email.contains('.') {
    // validation
}

// Better: Extract to shared function
fn validate_email(email: &str) -> Result<(), ValidationError> {
    // single source of truth
}
```
```

**Explain why:**
```markdown
**Why:** Duplicated code is harder to maintain. If we need to add email domain validation, we'd have to update 3 places.
```

### 6. Summary and Decision

Provide clear summary:

```markdown
## Review Summary

**Files reviewed:** 12
**Lines changed:** +245, -89
**Tests reviewed:** 8

**Issues found:** 4
- Critical: 1 (SQL injection - MUST FIX)
- Major: 2 (error handling, N+1 query)
- Minor: 1 (naming)

**Recommendation:** Fix critical issue, address major issues before merge. Minor issues can be tracked separately.

**Ready for merge if:** Critical issue is fixed
```

## Review Techniques

### Top-Down Review

1. Start with high-level architecture
2. Drill down to module structure
3. Examine specific implementations
4. Check tests last

### Bottom-Up Review

1. Start with test files
2. Understand expected behavior
3. Review implementation to match tests
4. Check integration points

### Security-First Review

1. Identify attack surfaces
2. Check authentication/authorization
3. Validate input handling
4. Review data storage
5. Check error handling

## Anti-Patterns to Avoid

❌ **Don't nitpick style** - Focus on correctness and maintainability

❌ **Don't miss context** - Understand why code was written this way

❌ **Don't skip tests** - Always review test coverage

❌ **Don't give vague feedback** - Be specific with file/line numbers

❌ **Don't ignore severity** - Critical issues must block progress

❌ **Don't skip positives** - Acknowledge what's done well

## Review Tools

### Command-Line Review

```bash
# Review with syntax highlighting
git diff main...feature | less -R

# Review specific file
git show branch:file | diff - file

# Blame for context
git blame src/file.rs
```

### Automated Checks

Run before manual review:
```bash
# Static analysis
cargo clippy

# Formatting
cargo fmt --check

# Security
cargo audit

# Tests
cargo test
```

### Code Review Tools

Consider using:
- GitHub pull request reviews
- GitLab merge request reviews
- Code review platforms (Review Board, Phabricator)

## Success Criteria

Review is successful when:
- ✅ All critical issues identified
- ✅ Issues reported with clear severity
- ✅ Specific file/line locations provided
- ✅ Actionable recommendations given
- ✅ User understands what needs fixing
- ✅ Clear go/no-go decision made

## Follow-Up

After review:

1. **Track issues** - Create issues or comments for each problem
2. **Assign fixes** - Who will fix what
3. **Re-review** - Check fixes before merging
4. **Document lessons** - Learn from issues found

## Example Review Session

```markdown
## Code Review: User Notifications Feature

### Changes Overview
- 8 files modified
- +312 lines, -45 lines
- 5 new tests

### Critical Issues ❌

**1. SQL Injection in notification query**
- File: `src/api/notifications/handlers.rs:67`
- Problem: `format!("SELECT * FROM notifications WHERE user_id = {}", user_id)`
- Impact: Attacker can inject arbitrary SQL
- Fix: Use parameterized query with SQLx

### Major Issues ⚠️

**2. Missing error handling**
- File: `src/services/notification.rs:89`
- Problem: No handling for failed push notification
- Impact: Silent failures, no retry
- Fix: Add error handling and retry queue

**3. No rate limiting**
- File: `src/api/notifications/handlers.rs`
- Problem: No limits on notification frequency
- Impact: Potential DoS from spam
- Fix: Add rate limiting middleware

### Minor Issues 💡

**4. Duplicate validation logic**
- Files: `src/api/notifications/handlers.rs`, `src/api/users/handlers.rs`
- Problem: Email validation duplicated
- Fix: Extract to `validate_email()` function

### What's Good ✅

- Comprehensive test coverage
- Clean separation of concerns
- Proper use of async/await
- Good error message structure

### Recommendation

**Must fix before merge:**
- Issue #1 (SQL injection) - security risk
- Issue #2 (error handling) - reliability risk

**Should fix before merge:**
- Issue #3 (rate limiting) - prevent abuse

**Can track separately:**
- Issue #4 (duplication) - refactor later

**Ready for merge after fixing critical and major issues.**
```
