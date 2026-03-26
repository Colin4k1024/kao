---
name: "dispatching-parallel-agents"
description: "Manages concurrent subagent workflows with task distribution, progress tracking, and result aggregation. Use when multiple independent tasks can run simultaneously."
---

# Dispatching Parallel Agents

The dispatching-parallel-agents skill coordinates multiple subagents working simultaneously on different tasks. It handles task distribution, conflict prevention, and result aggregation.

## When to Use

Use this skill when:
- Multiple independent tasks exist that can run in parallel
- You want to accelerate implementation
- Tasks are well-defined and isolated
- You have a clear plan to follow
- User approves parallel execution

## Core Concepts

### Task Independence

Tasks are independent if they:
- Don't modify the same files
- Don't depend on each other's output
- Can be verified standalone
- Have no shared state

### Conflict Prevention

Before dispatching agents:
- Map file dependencies
- Group tasks by shared files
- Assign each file group to one agent
- Prevent multiple agents from touching same files

## The Process

### 1. Analyze Dependencies

Create a dependency matrix:
```
Task A: files: [model.rs, repo.rs]
Task B: files: [service.rs, handlers.rs]
Task C: files: [model.rs, service.rs]  ← CONFLICT with A and B
```

Reorganize to avoid conflicts:
```
Agent 1: Tasks A, C  (shared files model.rs, service.rs)
Agent 2: Task B      (separate files)
```

### 2. Create Agent Manifest

Document what each agent will do:

```markdown
## Agent 1: User Module
- Tasks: 1, 3, 5
- Files: user/model.rs, user/repo.rs, user/service.rs
- Context: Building user CRUD operations

## Agent 2: Auth Module
- Tasks: 2, 4, 6
- Files: auth/handlers.rs, auth/service.rs, auth/middleware.rs
- Context: Authentication and authorization

## Agent 3: Tests
- Tasks: 7, 8, 9
- Files: tests/*.rs
- Context: Integration tests for user and auth
```

### 3. Dispatch with Context

For each agent, provide:

**Task Specification:**
- Exact files to modify
- Specific task instructions
- Success criteria
- Verification steps

**Shared Context:**
- Design document summary
- Project conventions
- Existing patterns to follow
- Non-goals and constraints

**Quality Standards:**
- Code review checklist
- Testing requirements
- Commit message format
- Error handling expectations

### 4. Monitor Progress

Track each agent's progress:
- Tasks assigned vs completed
- Files modified
- Test results
- Issues encountered

**Status format:**
```
Agent 1 (User Module): ████████░░ 80% - Tasks 1,3 done, working on 5
Agent 2 (Auth Module): ██████░░░░ 60% - Tasks 2,4 done, blocked on 6
Agent 3 (Tests): ░░░░░░░░░░ 0% - Waiting for Agent 1,2
```

### 5. Aggregate Results

When agents complete:
1. Collect their changes
2. Review each for quality
3. Merge without conflicts
4. Run integrated tests
5. Document what was built

### 6. Handle Conflicts

If conflicts emerge:
1. Identify which agents touched same files
2. Analyze each change's intent
3. Combine or prioritize changes
4. Verify combined result works
5. Document conflict resolution

## Agent Configuration

### Good Agent Configuration

**Single responsibility:**
```
Agent: User CRUD
Tasks: 5 tasks across user module
Files: 3 files (model, repo, service)
Scope: Clear, focused, independent
```

### Bad Agent Configuration

**Too many responsibilities:**
```
Agent: Everything
Tasks: 50 tasks across entire codebase
Files: 200 files
Scope: Too broad, context lost, conflicts likely
```

**Shared files:**
```
Agent 1: Update User model
Agent 2: Refactor User service
Agent 3: Add User tests
Problem: All three need User model - must coordinate
```

## Quality Control

### Per-Agent Review

Before accepting agent output:

1. **Completeness check**
   - All tasks done?
   - All files modified as specified?
   - Tests written and passing?

2. **Quality check**
   - Follows project patterns?
   - Code is readable?
   - No obvious bugs?
   - Error handling present?

3. **Integration check**
   - Works with other modules?
   - Tests pass together?
   - No regressions?

### Aggregate Review

After merging all agents:

1. **Full test suite**
   ```bash
   cargo test
   ```

2. **Linting**
   ```bash
   cargo clippy
   cargo fmt --check
   ```

3. **Integration test**
   ```bash
   cargo test --test '*_integration'
   ```

4. **Manual verification**
   - Run the application
   - Test key workflows
   - Verify no errors

## Error Handling

### Agent Failure

**Symptom:** Agent reports inability to complete task

**Response:**
1. Diagnose the issue
2. Provide clarification or fix blocker
3. Either:
   - Relaunch agent with better context
   - Complete task yourself
   - Adjust plan
4. Document issue and resolution

### Merge Conflict

**Symptom:** Two agents modified same file differently

**Response:**
1. Identify conflicting changes
2. Understand each change's purpose
3. Resolve to preserve both intents
4. Test resolution
5. Document conflict and resolution

### Quality Failure

**Symptom:** Agent output doesn't meet standards

**Response:**
1. List specific quality issues
2. Request agent to fix OR fix directly
3. Re-review after changes
4. Do not proceed until quality acceptable

## Communication Protocol

### With User

**At dispatch:**
"Dispatching [N] agents to implement [Feature]:
- Agent 1: [Description] ([X] tasks)
- Agent 2: [Description] ([Y] tasks)
- ...
Will report back when complete or at checkpoints."

**At checkpoint:**
"Checkpoint: [N/M] tasks complete.
Agent 1: ✅ Done
Agent 2: Working on task [X]
Agent 3: Blocked - waiting for [reason]
Options: [Continue/Review/Adjust]"

**At completion:**
"All agents complete. 
[N] tasks finished.
[M] files modified.
[P] tests added.
Running integration tests...
[Results]

Ready for review or next phase."

### With Agents

**Dispatch prompt template:**
```
You are Agent [N], responsible for [MODULE].

## Your Mission
[Brief description of what you're building]

## Tasks to Complete
1. [Task 1]: [Description]
   - Files: [list]
   - Verify: [how to confirm it works]

2. [Task 2]: [Description]
   - Files: [list]
   - Verify: [how to confirm it works]

## Context
- Design: [link or summary]
- Patterns: [existing code to follow]
- Conventions: [project standards]

## Quality Standards
- Tests required for all features
- Follow existing code style
- Commit after each task
- Report completion with status

## Constraints
- Don't modify files outside your scope
- Don't break existing tests
- Don't skip error handling

Ready? Begin with Task 1.
```

## Success Metrics

The dispatching is successful when:
- ✅ All agents complete their tasks
- ✅ No files modified by multiple agents (or conflicts resolved)
- ✅ All tasks meet quality standards
- ✅ Tests pass individually and together
- ✅ User informed of progress and completion
- ✅ Changes can be merged cleanly

## When to Stop

Stop dispatching agents when:
- All tasks complete
- User requests review
- Quality issues can't be resolved
- Conflicts are blocking progress
- User says "stop"

## Best Practices

1. **Start conservative** - 2-3 agents, expand if working
2. **Keep scopes small** - 3-5 tasks per agent
3. **Prevent conflicts** - Assign files exclusively
4. **Monitor closely** - Check progress frequently
5. **Review thoroughly** - Don't skip quality checks
6. **Communicate often** - Keep user informed
7. **Handle issues fast** - Don't let problems compound
