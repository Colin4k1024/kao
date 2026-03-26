---
name: "writing-plans"
description: "Breaks approved designs into actionable implementation tasks. Each task is 2-5 minutes, has exact file paths, complete code, and verification steps. Activates after design approval."
---

# Writing Plans

The writing-plans skill transforms approved designs into concrete implementation tasks. It enforces a specific format that makes work unambiguous and verifiable.

## When to Use

Activate writing-plans when:
- User has approved a design from brainstorming
- You have a SPEC.md or design document to implement
- Starting new features, bug fixes, or refactoring
- Breaking down complex work into manageable chunks

## The Process

### 1. Review the Design

Before writing tasks, ensure you understand:
- The problem being solved
- The proposed solution
- Success criteria and acceptance tests
- Constraints and non-goals
- Dependencies and prerequisites

### 2. Identify Entry Point

Find the natural starting point:
- What file/module does this touch first?
- Are there prerequisites (new dependencies, migrations)?
- What's the minimum viable change?

### 3. Break Down into 2-5 Minute Tasks

Each task should be:
- **Completable in 2-5 minutes** by an enthusiastic junior engineer
- **Atomic** - does one thing, can be committed independently
- **Verifiable** - has clear success criteria
- **Independent** - can be understood without context from other tasks

**Task structure:**
```markdown
### [TASK-NUMBER]: Brief description

**Files to modify:**
- `path/to/file1.ext`
- `path/to/file2.ext`

**What to do:**
1. Specific action
2. Specific action
3. Specific action

**Verification:**
- How to verify this works
- Expected outcome

**Notes:**
- Any gotchas or considerations
- Related decisions from design
```

### 4. Order Tasks Logically

Arrange tasks to:
- Minimize merge conflicts
- Build on each other naturally
- Allow early testing of core functionality
- Surface blockers early

**Recommended order:**
1. Prerequisites (dependencies, migrations, types)
2. Core data models and interfaces
3. Implementation logic
4. Integration points
5. Tests
6. Documentation

### 5. Verify Completeness

Check that the plan:
- ✅ Covers all aspects of the design
- ✅ Has no circular dependencies between tasks
- ✅ Includes rollback/revert strategy if needed
- ✅ Accounts for testing at multiple levels

## Task Examples

### Good Task (2-5 minutes)

```markdown
### 3: Add user validation to login endpoint

**Files to modify:**
- `backend/src/api/auth/handlers.rs`
- `backend/src/features/auth/model.rs`

**What to do:**
1. Add `email` field validation to `LoginRequest` model:
   ```rust
   #[validate(email)]
   pub email: String,
   ```
2. Update `login` handler to return 400 with specific error if email is invalid
3. Add test case for invalid email format

**Verification:**
- Run `cargo test auth_login::invalid_email`
- Should see test pass with 400 status
- Manual: POST /api/auth/login with `{"email": "not-an-email"}` returns 400

**Notes:**
- Follow existing error response format from auth service
```

### Bad Task (Too Large)

❌ "Implement user authentication system"

**Why it's bad:**
- Takes hours, not minutes
- Can't verify incrementally
- Hard to commit safely
- Blocks other work

### Bad Task (Too Vague)

❌ "Fix the login bug"

**Why it's bad:**
- Doesn't say what to change
- Doesn't explain how to verify
- Requires additional investigation

### Good Task (Specific)

✅ "Return proper error message when password is wrong in login"

## Anti-Patterns to Avoid

❌ **Don't write 1-hour tasks** - If a task takes more than 5 minutes, split it.

❌ **Don't skip testing tasks** - Every feature needs tests.

❌ **Don't ignore edge cases** - Include tasks for error handling and boundary conditions.

❌ **Don't skip review tasks** - Plan for code review between logical groups.

❌ **Don't forget rollback** - If destructive, include revert strategy.

## Plan Format

Save plans as: `docs/plans/[date]-[brief-name]-implementation.md`

```markdown
# Implementation Plan: [Feature Name]

## Overview
Brief description of what we're building and why.

## Prerequisites
- [ ] List any setup tasks, migrations, dependencies
- [ ] Note any blocking items

## Tasks

### 1: [Task name]
[Task details]

### 2: [Task name]
[Task details]

## Verification
How to verify the entire feature works end-to-end.

## Rollback Plan
How to revert if something goes wrong.

## Notes
Additional context, decisions, gotchas.
```

## Success Criteria

The plan is successful when:
- ✅ Every task is 2-5 minutes
- ✅ Every task has exact file paths
- ✅ Every task has verifiable completion criteria
- ✅ Tasks are in logical order
- ✅ Someone could follow the plan without asking questions
- ✅ User has approved the plan before implementation begins
