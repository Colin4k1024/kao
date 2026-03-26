---
name: "subagent-driven-development"
description: "Dispatches parallel subagents for concurrent task execution with two-stage review (spec compliance + code quality). Activates for large plans (10+ tasks) to maximize throughput."
---

# Subagent-Driven Development

The subagent-driven-development skill scales implementation by dispatching parallel subagents to work on independent tasks simultaneously. It maintains quality through rigorous review at each stage.

## When to Use

Activate subagent-driven development when:
- Implementation plan has 10+ tasks
- Tasks are largely independent
- Multiple features need building in parallel
- Time-sensitive delivery required
- User says "go" on a large plan

## Prerequisites

Before starting:
- ✅ Approved implementation plan exists
- ✅ Tasks are broken into 2-5 minute chunks
- ✅ Task dependencies are minimal or clearly marked
- ✅ Clean working directory
- ✅ Tests pass baseline

## The Process

### 1. Analyze Task Graph

Review the plan and identify:
- **Independent tasks** - Can run in parallel
- **Sequential tasks** - Must run in order
- **Bottlenecks** - Tasks that others depend on
- **Logical groups** - Tasks that should be reviewed together

### 2. Create Subagent Batches

Group tasks into batches based on:
- Independence (no shared files)
- Logical cohesion (same feature area)
- Size (3-5 tasks per subagent works well)

**Example batch assignment:**

```
Batch A (3 tasks):
- Task 1: Add User model
- Task 2: Add User repository
- Task 3: Add User service

Batch B (3 tasks):
- Task 4: Add Role model
- Task 5: Add Role repository
- Task 6: Add Role service

Batch C (4 tasks):
- Task 7: Add User-Role integration
- Task 8: Add tests
- Task 9: Add API routes
- Task 10: Update docs
```

### 3. Dispatch Subagents

For each batch, launch a subagent with:

**Prompt structure:**
```
You are implementing tasks [X-Y] from the [Feature Name] implementation plan.

## Context
[Brief description of the feature and why we're building it]

## Your Tasks
[Numbered list of specific tasks with file paths and exact instructions]

## Instructions
1. Follow RED-GREEN-REFACTOR for each task
2. Write failing tests first, then minimal code to pass
3. Review your code for quality before completing
4. Commit after each task with clear message
5. Report completion with:
   - Tasks completed
   - Files modified
   - Tests added/passing
   - Any issues encountered

## Verification
[How to verify the batch works end-to-end]
```

### 4. Two-Stage Review

When subagent reports completion, review in two stages:

**Stage 1: Spec Compliance Review**
- Did they complete all assigned tasks?
- Does each task match the plan specification?
- Are all acceptance criteria met?
- Are edge cases handled?

**Stage 2: Code Quality Review**
- Is the code readable and well-organized?
- Are there obvious bugs, security issues, or performance problems?
- Is error handling appropriate?
- Does it follow project conventions?
- Are tests adequate?

**Review commands:**
```bash
# Check what changed
git diff [files]

# Run tests
cargo test

# Review specific changes
git show [commit-hash]

# Check for issues
cargo clippy
cargo fmt --check
```

### 5. Handle Issues

**If issues found:**
- Document specific problems
- Ask subagent to fix (or fix yourself)
- Re-review after changes
- Don't merge until quality is acceptable

**If subagent is blocked:**
- Provide clarification or context
- Adjust task scope if needed
- Consider redistributing tasks

### 6. Sequential Bottlenecks

For tasks that must run in order:
- Complete them yourself OR
- Run a single-threaded subagent
- Don't parallelize dependent tasks

### 7. Integration

After all parallel batches complete:
1. Merge all changes
2. Resolve any conflicts
3. Run full test suite
4. Verify end-to-end functionality
5. Create integration commit

## Subagent Batching Strategy

### Good Batching

**Independent tasks, shared domain:**
```
Subagent 1: User CRUD
- Create user model
- Create user repository
- Create user service
- Add user tests

Subagent 2: Role CRUD
- Create role model
- Create role repository
- Create role service
- Add role tests

Subagent 3: Integration
- User-Role relationships
- Permission checks
- Integration tests
```

### Bad Batching

**Overlapping files, conflicts inevitable:**
```
Subagent 1: Add field to User model
Subagent 2: Refactor User repository
Subagent 3: Update User API routes
# All three touch User model - will have conflicts
```

## Coordination Protocol

### Communication

**At start:**
"Starting subagent-driven development for [Feature]. 
I'll dispatch [N] subagents to work on [X] tasks in parallel.
Expect checkpoint after each batch completes."

**After each batch:**
"Batch [N] complete: [tasks]. 
Reviewing code quality...
[Issues found] → [Fixed/Assigned]
Ready for next batch: [Y-Z]"

**At end:**
"All batches complete. Running integration...
Merging changes, resolving conflicts...
Full test suite: [X/Y] passing.
[Feature] implementation complete."

### Error Handling

**Subagent failure:**
1. Diagnose the issue
2. Either fix directly or relaunch with better context
3. Document the problem for learning

**Merge conflicts:**
1. Analyze conflicts carefully
2. Resolve in favor of correct implementation
3. Verify tests still pass
4. Document resolution rationale

**Quality issues:**
1. Be specific about problems
2. Request fixes before accepting
3. Never accept substandard code

## Anti-Patterns to Avoid

❌ **Don't parallelize dependent tasks** - This creates confusion and conflicts.

❌ **Don't skip review stages** - Quality will suffer.

❌ **Don't assign too many tasks per subagent** - Context gets lost.

❌ **Don't merge without running full tests** - Integration issues happen.

❌ **Don't ignore conflicts** - Address them immediately.

❌ **Don't skip commits** - Each subagent should commit their work.

## Success Criteria

Subagent-driven development is successful when:
- ✅ All tasks completed correctly
- ✅ No spec violations
- ✅ High code quality maintained
- ✅ Conflicts minimized and resolved properly
- ✅ Tests passing end-to-end
- ✅ User informed throughout

## Performance Tips

- **Start with 2-3 subagents** - Add more if no conflicts
- **Batch by domain** - Reduces merge conflicts
- **Keep tasks small** - Easier to verify and review
- **Review frequently** - Catch issues early
- **Communicate often** - User stays informed

## When to Stop

Exit subagent-driven mode when:
- All tasks complete
- User requests review
- Issues are blocking
- Quality is degrading
- User says "stop"
