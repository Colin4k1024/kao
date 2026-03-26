---
name: "executing-plans"
description: "Executes implementation plans task-by-task with two-stage review (spec compliance, then code quality). Enforces TDD cycle: RED-GREEN-REFACTOR. Activates when user says 'go' or approves plan."
---

# Executing Plans

The executing-plans skill turns approved plans into committed code. It enforces a disciplined, test-driven approach with regular review checkpoints.

## When to Use

Activate executing-plans when:
- User has approved an implementation plan
- User says "go", "do it", "start implementing", or similar
- Ready to begin work after brainstorming + planning

## The Process

### 1. Verify Preconditions

Before starting:
- ✅ Clean working directory (no uncommitted changes)
- ✅ Tests pass baseline
- ✅ Plan is approved by user
- ✅ Environment is ready (dependencies installed, services running)

If not clean, ask: "Should I clean up first, or work around the existing state?"

### 2. Choose Execution Mode

**Option A: Batch Execution (Default)**
Process tasks in batches of 3-5, then pause for user checkpoint.

**Option B: Subagent-Driven Development**
For large plans (10+ tasks), dispatch subagents to work in parallel.

Choose based on:
- Plan size (small = batch, large = subagents)
- Task independence (independent = parallel, sequential = batch)
- User preference (ask if unsure)

### 3. Implement Each Task

For every task, follow the RED-GREEN-REFACTOR cycle:

**RED - Write the failing test**
```bash
# Write test that describes expected behavior
# Run it, confirm it fails with clear error message
cargo test [test_name]
# Expected: test fails ❌
```

**GREEN - Write minimal code to pass**
```bash
# Write ONLY the code needed to make the test pass
# No optimizations, no "improvements"
# Get to green as fast as possible
cargo test [test_name]
# Expected: test passes ✅
```

**REFACTOR - Clean up (optional in early stages)**
```bash
# Improve code quality if needed
# Ensure tests still pass
cargo test
# Expected: all tests pass ✅
```

### 4. Two-Stage Review

After each task, review against two criteria:

**Stage 1: Spec Compliance**
- Does this complete what the task asked?
- Does it match the design document?
- Are all acceptance criteria met?
- Are edge cases handled?

**Stage 2: Code Quality**
- Is the code clear and readable?
- Are there obvious bugs or security issues?
- Is error handling appropriate?
- Are there unnecessary abstractions?
- Does it follow project conventions?

If issues found: fix them before proceeding.

### 5. Commit Strategy

After each task or logical group:
```bash
# Commit after each successful task
git add [files]
git commit -m "feat: [brief description]

- What changed
- Why it was needed
- How to verify"
```

Keep commits:
- **Small** - one task or logical unit
- **Verifiable** - tests pass
- **Messageful** - explains the "why"

### 6. Checkpoint Protocol

After every 3-5 tasks or logical grouping:

**Stop and report:**
```
Completed: [X/Y] tasks
Last 3 tasks:
- ✅ Task 3: Added validation
- ✅ Task 4: Implemented handler
- ✅ Task 5: Added tests

Current state:
- All tests passing
- No uncommitted changes
- [Any issues/observations]

Should I continue with tasks 6-10, or do you want to review the current state?
```

**User may:**
- Review current code
- Modify remaining tasks
- Change direction
- Take a break
- Say "continue"

## Anti-Patterns to Avoid

❌ **Don't skip the RED phase** - Always write failing tests first.

❌ **Don't write code before tests** - This defeats the purpose of TDD.

❌ **Don't skip commits** - Commit after each task for safety.

❌ **Don't skip reviews** - Two-stage review catches most issues.

❌ **Don't skip checkpoints** - Pause every 3-5 tasks for user input.

❌ **Don't optimize prematurely** - GREEN phase is about minimal code.

## Handling Issues

### Task Takes Longer Than Expected

If a task takes more than 5-10 minutes:
1. Stop and assess
2. Break into smaller subtasks if needed
3. Ask for help if blocked
4. Note the issue in your progress

### Test Won't Pass

1. Verify test is correct (does it match spec?)
2. Check for environmental issues
3. Try minimal implementation
4. Ask for clarification if spec is unclear

### Discovered Issue in Spec

If you find the spec is wrong or incomplete:
1. Don't guess - stop and ask
2. Propose correction
3. Get approval before proceeding

### Merge Conflicts

If working in parallel (subagent mode):
1. Pull latest changes
2. Resolve conflicts carefully
3. Ensure tests still pass
4. Verify no functionality broken

## Success Criteria

Execution is successful when:
- ✅ All tasks completed
- ✅ All tests passing
- ✅ Code matches spec
- ✅ Regular commits made
- ✅ Checkpoints honored
- ✅ User informed of progress

## Subagent-Driven Mode

For large plans, dispatch subagents:

1. Break plan into independent chunks
2. Launch subagent for each chunk with:
   - Specific tasks to complete
   - Relevant context from design
   - Instructions to follow RED-GREEN-REFACTOR
3. Review each subagent's work before merging
4. Handle integration and conflicts

See `dispatching-parallel-agents` skill for details.
