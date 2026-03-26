---
name: "using-git-worktrees"
description: "Creates isolated Git worktrees for parallel development on different branches. Prevents context-switching overhead and merge conflicts. Activates when starting new development."
---

# Using Git Worktrees

The using-git-worktrees skill leverages Git worktrees to create isolated working directories for parallel development. This enables working on multiple features or fixes simultaneously without context-switching overhead.

## When to Use

Activate using-git-worktrees when:
- Starting new feature development
- Need to work on multiple things simultaneously
- Bug fix is urgent but current work isn't ready
- Want to avoid stashing or committing unfinished work
- Planning to work in parallel with subagents

## Why Worktrees Instead of Branches?

### Traditional Branch Approach
```
main ───●─────●───── work on feature A
                └──●─────●───── switch back to feature A
```

Problems:
- Uncommitted work must be stashed or committed
- Switching contexts is disruptive
- Hard to work on multiple features simultaneously

### Worktree Approach
```
main ───●
│
├── feature-A ───●─────●───── isolated worktree
│
└── bug-fix ───●─────●───── different worktree
```

Benefits:
- Multiple branches visible as separate directories
- No stashing required - worktree has its own working directory
- Switch between tasks by changing directories
- Parallel development without conflicts

## The Process

### 1. Assess Current State

Check current working directory:
```bash
git status
git branch
```

Determine:
- Is there uncommitted work?
- What branch are you on?
- What branch do you need?

### 2. Create Worktree

Create a new worktree for your task:

```bash
git worktree add [path] [branch-name]
```

Examples:
```bash
# Create feature branch in new worktree
git worktree add ../feature-notifications feature/notifications

# Create bugfix branch in new worktree
git worktree add ../bugfix-login bugfix/login-broken

# Create from existing branch
git worktree add ../hotfix-urgent hotfix/vip-crash
```

### 3. Configure Worktree

In the new worktree:
```bash
# Navigate to worktree
cd ../feature-notifications

# Verify you're in the right place
pwd
git status

# Install dependencies if needed
cargo build  # or npm install, etc.

# Run tests to ensure clean baseline
cargo test
```

### 4. Work in Isolation

Each worktree operates independently:
- Worktree 1: `~/projects/myapp/feature-auth`
- Worktree 2: `~/projects/myapp/feature-notifications`
- Main repo: `~/projects/myapp`

Switch between worktrees by changing directories:
```bash
cd ../feature-auth    # Switch to auth feature
cd ../feature-notifications  # Switch to notifications
```

### 5. Sync Changes

When ready to merge:
```bash
# In the worktree
git pull origin main  # or rebase on main

# Merge to main
git checkout main
git merge feature/notifications

# Clean up worktree
git worktree remove ../feature-notifications
git branch -d feature/notifications
```

## Worktree Management

### List All Worktrees
```bash
git worktree list
```

Output:
```
~/projects/myapp          8f12a3c [main]
~/projects/feature-auth  9b8c7d4 [feature/auth]
~/projects/bugfix-login  2d3e4f5 [bugfix/login]
```

### Remove Worktree
```bash
# Safe removal (if merged)
git worktree remove ../feature-auth

# Force removal (if not merged)
git worktree remove --force ../feature-auth
```

### Prune Stale Worktrees
```bash
git worktree prune
```

## Directory Naming Conventions

Choose clear, consistent names:
```
../feature-[name]      # New features
../bugfix-[issue]      # Bug fixes
../hotfix-[severity]   # Urgent fixes
../refactor-[module]   # Refactoring
../experiment-[name]    # Exploratory work
```

## Use Cases

### Use Case 1: Urgent Bug Fix

**Scenario:** Working on feature A, but critical bug is reported.

**Without worktree:**
1. Stash or commit current work
2. Checkout/create bugfix branch
3. Fix bug
4. Merge/release
5. Go back to feature A
6. Unstash/reapply work

**With worktree:**
1. `git worktree add ../bugfix-login hotfix/login-crash`
2. Navigate to bugfix-login
3. Fix bug
4. Merge/release
5. Done - feature-a worktree untouched

### Use Case 2: Parallel Features

**Scenario:** Need to implement two related features.

**Approach:**
1. `git worktree add ../feature-payments feature/payments`
2. `git worktree add ../feature-refunds feature/refunds`
3. Work on payments in one directory
4. Work on refunds in another
5. Integrate when both ready

### Use Case 3: Subagent Development

**Scenario:** Dispatching multiple subagents.

**Approach:**
1. Create worktree for each subagent
2. Each subagent works in isolation
3. Review each worktree's output
4. Merge completed worktrees
5. Clean up when done

```bash
git worktree add ../agent-1-user-module agent/user-module
git worktree add ../agent-2-auth-module agent/auth-module
# Dispatch agents to respective directories
```

## Anti-Patterns to Avoid

❌ **Don't create too many worktrees** - Hard to track, uses disk space

❌ **Don't modify same files in different worktrees** - Conflicts when merging

❌ **Don't forget which worktree you're in** - Use clear directory names

❌ **Don't leave worktrees unmerged** - Can become stale and confusing

❌ **Don't share worktrees** - Each should be used by one person/agent

## Integration with Development Flow

### Complete Workflow

1. **Start new work**
   ```bash
   git worktree add ../feature-new feature/new
   cd ../feature-new
   ```

2. **Implement feature**
   ```bash
   # Follow writing-plans, executing-plans skills
   # Write tests, implement, commit
   ```

3. **Sync with main**
   ```bash
   git fetch origin
   git rebase origin/main  # or merge
   ```

4. **Run tests**
   ```bash
   cargo test
   ```

5. **Merge and cleanup**
   ```bash
   git checkout main
   git merge feature/new
   git worktree remove ../feature-new
   git branch -d feature/new
   ```

## Best Practices

1. **Name clearly** - Directory and branch names should match
2. **Stay organized** - Keep worktrees in predictable location
3. **Clean up** - Remove when done to save space
4. **Track worktrees** - Use `git worktree list` to see all
5. **Sync regularly** - Rebase on main to avoid big merges
6. **Test before merge** - Run full test suite in worktree

## Success Criteria

Using worktrees is successful when:
- ✅ Isolated development environment for each task
- ✅ No uncommitted work lost or stashed
- ✅ Easy switching between contexts
- ✅ Clean merge when done
- ✅ No conflicts from parallel work
- ✅ Team can work in parallel

## Troubleshooting

### "Worktree already exists"
```bash
git worktree list  # Check existing
git worktree remove ../duplicate  # Remove if safe
```

### "Branch already exists"
```bash
git branch -a | grep branch-name  # Check if exists
git checkout branch-name          # Use existing
# OR
git worktree add ../path new-branch-name --create  # Force create
```

### "Cannot lock"
```bash
git worktree prune     # Clean stale
git worktree list       # Check status
```
