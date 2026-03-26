# Superpowers Skills - Installation Complete ✅

## Skills Installed

Successfully installed **12 Superpowers skills** to your Trae IDE:

### Collaboration (8 skills)

1. **brainstorming** - Refines ideas through Socratic questioning
2. **writing-plans** - Breaks designs into 2-5 minute actionable tasks
3. **executing-plans** - Implements tasks with RED-GREEN-REFACTOR cycle
4. **subagent-driven-development** - Parallel subagent execution with review
5. **dispatching-parallel-agents** - Manages concurrent agent workflows
6. **using-git-worktrees** - Isolated parallel development
7. **finishing-a-development-branch** - Verifies, merges, and cleans up
8. **requesting-code-review** - Reviews code with severity-based reporting
9. **receiving-code-review** - Responds professionally to feedback

### Testing (1 skill)

10. **test-driven-development** - RED-GREEN-REFACTOR cycle enforcement

### Debugging (2 skills)

11. **systematic-debugging** - 4-phase root cause analysis
12. **verification-before-completion** - Ensures fixes actually work

## How These Skills Work

The Superpowers framework activates automatically when relevant:

### 1. **Brainstorming** (Before Writing Code)
When you describe what you want to build, the agent will:
- Ask clarifying questions (Socratic method)
- Explore alternatives
- Present design in digestible chunks
- Get your approval before proceeding

### 2. **Writing Plans** (After Design Approval)
Once design is approved, the agent will:
- Break work into 2-5 minute tasks
- Each task has exact file paths
- Include verification steps
- Create implementation plan

### 3. **Executing Plans** (Implementation)
When you say "go", the agent will:
- Follow RED-GREEN-REFACTOR for each task
- Write failing tests first
- Implement minimal code
- Review at checkpoints

### 4. **Parallel Development** (For Large Tasks)
For big features (10+ tasks), the agent can:
- Dispatch multiple subagents
- Work on independent tasks in parallel
- Review each agent's work
- Handle conflicts

### 5. **Code Review** (Quality Assurance)
Before merging, the agent will:
- Review against plan specifications
- Check for bugs, security issues
- Report issues by severity
- Request your approval

### 6. **Finishing** (Completion)
When tasks complete, the agent will:
- Run full verification
- Present merge/PR options
- Clean up worktrees
- Ensure stability

## Automatic Triggers

These skills activate automatically based on context:

| Skill | Activates When |
|-------|---------------|
| brainstorming | You mention building something |
| writing-plans | Design is approved |
| executing-plans | You say "go" or approve plan |
| subagent-driven-development | Plan has 10+ tasks |
| using-git-worktrees | Starting new development |
| requesting-code-review | Between tasks or before merge |
| receiving-code-review | You receive feedback |
| test-driven-development | During implementation |
| systematic-debugging | Bug is reported |
| verification-before-completion | After any fix/implementation |
| finishing-a-development-branch | All tasks complete |
| dispatching-parallel-agents | Multiple independent tasks |

## Key Principles

### 🎯 **RED-GREEN-REFACTOR**
Every feature follows this cycle:
1. **RED** - Write failing test
2. **GREEN** - Write minimal code to pass
3. **REFACTOR** - Clean up (optional)

### 🎯 **2-5 Minute Tasks**
Tasks are small and verifiable:
- Can be completed quickly
- Easy to verify
- Safe to commit
- Clear ownership

### 🎯 **No Coding Before Design**
The agent won't jump into code until:
- Problem is understood
- Design is approved
- Plan is clear

### 🎯 **Continuous Verification**
Before claiming "done":
- All tests pass
- Manual verification done
- Edge cases tested
- No regressions

## Example Workflow

### User: "I want to add user notifications"

### Agent Response:
```
Let me understand what you're building. Before I start planning anything:

1. **Who receives notifications?** (users, admins, both?)
2. **What triggers them?** (system events, user actions, scheduled?)
3. **Real-time or pull-based?** (push notifications or in-app?)
4. **Do you need notification history?**
5. **Any constraints around privacy/rate limits?**

[Through Socratic questioning, we refine the requirements]

Great! Here's the design for notifications:

## Core Concept
- Notifications are stored in database
- Users have an inbox view
- Supports real-time updates via WebSocket

## Data Model
[Showing design in digestible chunks]

Does this approach make sense? Should I continue?
```

### User: "Yes, looks good. Go ahead!"

### Agent:
```
Perfect! Here's the implementation plan:

## Plan: User Notifications

**Prerequisites:**
- [ ] Add notifications table migration
- [ ] Create notification model

**Tasks (18 total, ~90 minutes):**
1. Create migration for notifications table
2. Add notification model
3. Add repository for notifications
4. Add service layer
5. Create API endpoints
6. Add WebSocket support
7. Write tests
...

Starting implementation with RED-GREEN-REFACTOR.

### Task 1: Add notification model
[RED] Writing test...
[GREEN] Implementing...
[REFACTOR] Cleaning up...

Checkpoint: Tasks 1-5 complete. All tests passing.
Should I continue with tasks 6-10?
```

## Benefits

✅ **Prevents building wrong things** - Questions before coding  
✅ **Reduces bugs** - TDD catches issues early  
✅ **Faster development** - Parallel subagents for large tasks  
✅ **Better quality** - Systematic reviews and verification  
✅ **Clear progress** - Checkpoints keep you informed  
✅ **Easier maintenance** - Well-documented plans and fixes  

## Getting Started

Next time you want to build something, just describe it:

- "I want to add..."
- "Can we improve..."
- "Help me fix this bug..."
- "What if we..."

The agent will automatically activate the appropriate skill!

## Documentation

Each skill has detailed instructions in `.trae/skills/<skill-name>/SKILL.md`

For more details on a specific skill, refer to its documentation.
