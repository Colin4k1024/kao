---
name: "finishing-a-development-branch"
description: "Verifies completed work, runs tests, presents merge/PR options, and cleans up worktrees. Activates when all implementation tasks are complete."
---

# Finishing a Development Branch

The finishing-a-development-branch skill wraps up completed work by verifying everything works, presenting options for merging, and cleaning up the development environment.

## When to Use

Activate finishing-a-development-branch when:
- All implementation tasks are complete
- Tests are passing
- User says "done", "finished", "ready to merge"
- Checking if work is ready for integration

## The Process

### 1. Pre-Flight Verification

Before finishing, verify:

**Code completeness:**
- [ ] All planned tasks completed
- [ ] No TODO comments left
- [ ] No placeholder code
- [ ] Documentation updated if needed

**Tests:**
```bash
# Run full test suite
cargo test

# Run integration tests
cargo test --test '*_integration'

# Run specific feature tests
cargo test feature_name
```

**Code quality:**
```bash
# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Security audit (if applicable)
cargo audit
```

**Build:**
```bash
# Clean build
cargo build --release
```

### 2. Review Changes

Review what was built:

```bash
# See changed files
git diff --stat

# Review specific changes
git diff [file]

# Check commit history
git log --oneline -10
```

Verify:
- Changes align with original plan
- No unintended modifications
- Commit messages are clear
- Logical grouping of commits

### 3. Present Options

Present user with merge options:

```markdown
## Ready for Integration

**Summary:**
- Tasks completed: [X/Y]
- Files modified: [N]
- Tests added: [M]
- Commits: [K]

**Current state:**
- All tests passing ✅
- Code quality checks passed ✅
- Build successful ✅

## Options

### 1. Merge to Main
```bash
git checkout main
git merge feature-name
```
Fast-forward or merge commit. Simple, linear history.

### 2. Create Pull Request
Opens PR for code review before merging.
Recommended for:
- Team projects
- Significant changes
- Changes requiring stakeholder approval

### 3. Keep Branch Active
Leave branch for continued work.
Useful for:
- Incremental features
- Long-running development
- Staged rollouts

### 4. Discard Changes
```bash
git worktree remove ../feature
git branch -D feature-name
```
Removes all work. Use with caution!

## My Recommendation

[Your recommendation with rationale]

## Which option would you like?
```

### 4. Execute Selected Option

**If Merge:**
```bash
git checkout main
git merge feature-name
git push origin main

# Clean up if using worktree
git worktree remove ../feature-name
git branch -d feature-name
```

**If Pull Request:**
```bash
git push -u origin feature-name
# Open PR with description
# Link to original plan/design
```

**If Keep Active:**
- Document what's left to do
- Note any blockers
- Schedule next session

**If Discard:**
- Confirm with user first!
- Show what's being deleted
- Verify nothing lost

### 5. Post-Merge Verification

After merging:
```bash
# Verify main is stable
git checkout main
cargo test
cargo build

# Check deployment readiness
# If using CI/CD:
git log --oneline -1
# Verify CI pipeline passes
```

### 6. Celebrate & Document

**Mark completion:**
- Update task tracking if used
- Close related issues
- Update project board

**Knowledge capture:**
- Note what went well
- Document any issues encountered
- Update team on completion

## Common Scenarios

### Scenario 1: Clean Merge

Everything works, straightforward merge.

```
Status: Ready ✅
Action: Fast-forward merge
Result: Linear history, simple cleanup
```

### Scenario 2: Review Needed

User wants to see changes before merging.

```
Status: Ready for review
Action: Create PR or review session
Result: Approval, then merge
```

### Scenario 3: Conflicts Present

Merge conflicts need resolution.

```
Status: Conflicts detected
Action: Resolve conflicts
1. git merge --no-commit feature-name
2. Resolve conflicts manually
3. git add .
4. git commit
5. Verify tests still pass
```

### Scenario 4: Tests Failing

Something broke during development.

```
Status: Tests failing ❌
Action: Fix issues before proceeding
1. Run failing tests
2. Identify root cause
3. Fix in feature branch
4. Retest
5. Proceed with finish
```

### Scenario 5: Partial Completion

Not all tasks done, but user wants to save progress.

```
Status: Partial completion
Action: Create PR with WIP status
Result: Can continue later
```

## Anti-Patterns to Avoid

❌ **Don't skip verification** - Always test before finishing

❌ **Don't force through failures** - Fix problems first

❌ **Don't skip user approval** - Always present options

❌ **Don't forget to clean up** - Remove worktrees, stale branches

❌ **Don't discard without confirmation** - User must approve

❌ **Don't merge broken code** - Ensure stability first

## Post-Finish Checklist

After finishing:

- [ ] All changes merged/integrated
- [ ] Tests passing in main branch
- [ ] Build successful
- [ ] Worktree removed (if used)
- [ ] Branch deleted (if merged)
- [ ] Documentation updated
- [ ] User informed of completion
- [ ] Team notified (if applicable)
- [ ] Issues closed (if tracked)
- [ ] Next steps identified (if any)

## Success Criteria

Finishing is successful when:
- ✅ All verification checks pass
- ✅ User has chosen integration method
- ✅ Changes are merged or PR created
- ✅ Environment is clean
- ✅ Team is informed
- ✅ Documentation is current

## Follow-Up

After finishing a branch:

1. **Verify stability**
   ```bash
   git checkout main
   cargo test
   ```

2. **Check CI/CD**
   - Monitor deployment pipeline
   - Verify in staging/production

3. **Gather feedback**
   - User satisfaction
   - Any issues in production
   - Lessons learned

4. **Plan next steps**
   - What to work on next
   - Improvements to make
   - Technical debt to address

## Remember

Finishing a branch is not just about merging—it's about ensuring quality, keeping the codebase clean, and setting up for future success.
