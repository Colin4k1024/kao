---
name: "receiving-code-review"
description: "Responds to code review feedback constructively, implements fixes, and communicates progress. Handles feedback professionally and improves code quality."
---

# Receiving Code Review

The receiving-code-review skill helps you respond professionally to feedback, implement fixes efficiently, and use criticism to improve code quality.

## When to Use

Activate receiving-code-review when:
- You receive review feedback on your code
- Reviewer identifies issues
- User or team member provides critique
- After requesting-code-review reveals problems

## Mindset

### The Right Attitude

**Understand that:**
- Reviews improve code quality
- Reviewers catch what you missed
- All feedback is valuable (even if wrong)
- Code is separate from self-worth
- Iteration makes code better

**Avoid:**
- Taking feedback personally
- Becoming defensive
- Dismissing feedback without consideration
- Making excuses for problems
- Arguing about style preferences

### Assume Positive Intent

Reviewers want to help:
- They're not attacking you
- They're catching bugs before production
- They're ensuring code maintainability
- They're investing in your success

## The Process

### 1. Read All Feedback

First pass - understand everything:
```
"I notice the reviewer mentioned X, Y, Z issues. Let me read through everything before responding."
```

### 2. Don't React Immediately

Wait before responding:
- Read all feedback
- Understand each point
- Don't respond while emotional
- Give yourself time to process

### 3. Acknowledge and Categorize

Organize feedback:

```markdown
## Review Feedback - Acknowledged

### Critical Issues
- ✅ Acknowledged - Will fix immediately
- 🤔 Need clarification - Asking follow-up

### Major Issues  
- ✅ Acknowledged - Will fix
- ❌ Disagree - Will explain reasoning

### Minor Issues
- ✅ Will fix
- ❌ Won't fix - Will explain why
```

### 4. Respond Professionally

**Acknowledge all feedback:**
```markdown
"Thank you for the thorough review! I've addressed all the issues. Here's my response:"
```

**For each issue:**

**If you agree:**
```markdown
**Issue #1 (SQL injection):** 
✅ Good catch! This is a real security risk. I've fixed it by using parameterized queries. See commit [hash].
```

**If you disagree:**
```markdown
**Issue #2 (naming):**
🤔 I see your point, but I chose `getUserData()` because it matches the existing pattern in the codebase (see `getPostData()`, `getCommentData()` in the same file). Happy to rename if you'd prefer consistency with the API spec naming, but wanted to explain the current choice.
```

**If you need clarification:**
```markdown
**Issue #3 (error handling):**
Could you clarify what behavior you'd like when the database is unreachable? I assumed returning a 503, but let me know if you prefer a different approach.
```

### 5. Implement Fixes

For each acknowledged issue:

**Critical Issues - Fix Immediately:**
```bash
# Create fix branch if needed
git checkout -b fix/review-issues

# Fix issue
# Write test to prevent regression
# Commit with clear message
```

**Major Issues - Fix Before Merge:**
```bash
# Fix each issue
git add .
git commit -m "fix: address review comments

- Add parameterized queries to prevent SQL injection
- Add error handling for notification failures
- Add rate limiting middleware

Closes #[issue-number]"
```

**Minor Issues - Fix or Track:**
```bash
# Fix if quick
git commit -m "refactor: extract duplicated validation logic

Consolidates email validation into shared validate_email() function
as suggested in review."

# OR if not fixing
git commit -m "Note: Skipping duplication refactor as it's low priority
and would introduce risk at this stage. Will address in tech debt cleanup."
```

### 6. Re-Submit for Review

**Provide clear status:**
```markdown
## Review Response

All issues addressed:

### ✅ Fixed
- **SQL Injection** - Now using parameterized queries (commit: abc123)
- **Error handling** - Added try-catch with retry logic (commit: def456)
- **Naming** - Clarified reasoning, kept current naming

### ❌ Not Fixed (with explanation)
- **Magic numbers** - Extracted to constants would break API compatibility

### 🔄 In Progress
- **Rate limiting** - Implementing, will push shortly

Ready for re-review when rate limiting is complete.
```

**Show what changed:**
```bash
# Show fixes
git diff main...fix/review-issues

# Show specific fixes
git show [commit-hash]
```

## Communication Guidelines

### Do:

✅ **Thank reviewers** - They're investing time in your code

✅ **Be specific** - "I've fixed X in file Y at line Z"

✅ **Show evidence** - "Tests pass, here's output..."

✅ **Ask questions** - Clarify if feedback is unclear

✅ **Provide context** - Explain your decisions

✅ **Be humble** - Assume reviewer might be right

✅ **Be grateful for alternatives** - "Thanks for suggesting Y"

### Don't:

❌ **Don't be defensive** - "This is actually fine because..."

❌ **Don't make excuses** - "I was rushed" or "Legacy code"

❌ **Don't dismiss feedback** - "That's just style preference"

❌ **Don't argue endlessly** - Pick battles, concede gracefully

❌ **Don't passive-aggressive** - "Fine, I'll change it" vs. genuine acknowledgment

❌ **Don't ignore feedback** - All feedback deserves response

❌ **Don't take it personally** - They're reviewing code, not you

## Handling Different Types of Feedback

### 1. Critical Security Issues

**Response:**
```markdown
✅ This is a valid security concern. Great catch! 
I've implemented [solution] and added a test to prevent regression.
```

**Action:**
- Fix immediately
- Add regression test
- Consider if similar issues elsewhere
- Document what you learned

### 2. Architectural Concerns

**Response:**
```markdown
🤔 I see the architectural concern. The current structure was chosen because [reasons]. However, your point about [issue] is valid. 

Options:
A) Refactor now (takes ~2 hours)
B) Create tech debt ticket for later
C) Keep current approach with documented rationale

What do you prefer?
```

**Action:**
- Discuss trade-offs
- Make decision together
- Document decision

### 3. Style Preferences

**Response:**
```markdown
✅ Fair point on style. I've updated to follow [convention].

(or)

🤔 I noticed the codebase uses [X] style in other places, so I matched that. Want me to change everything to [Y] convention for consistency?
```

**Action:**
- Follow established patterns
- If no pattern exists, pick one and document it
- Don't let style debates block progress

### 4. Incorrect Assumptions

**Response:**
```markdown
🤔 I think there might be a misunderstanding here. The code actually does [X] because [explanation]. Did I miss something?

(Or if reviewer is correct)

✅ You're right, I misunderstood. I've updated the code to [solution].
```

**Action:**
- Clarify if reviewer misunderstood
- Accept if you misunderstood
- Don't be condescending

### 5. "Nit" Comments

**Response:**
```markdown
✅ Fixed (or) 🤔 Good point, but I've chosen [alternative] because [reasons]. Happy to change if you feel strongly.
```

**Action:**
- Fix if trivial
- Don't argue if it's truly nitpicky
- Pick battles

## Example Conversations

### Example 1: Agreeing with Feedback

**Reviewer:**
```
❌ Critical: SQL injection in user query at handlers.rs:45
```

**Your Response:**
```
✅ Great catch - this is definitely a security risk. I've fixed it by switching to parameterized queries. 

Commit: abc123
Test added: test_injection_prevention()

Ready for re-review.
```

### Example 2: Disagreeing with Reasoning

**Reviewer:**
```
⚠️ Should use HashMap instead of Vec for better performance
```

**Your Response:**
```
🤔 Good thinking on performance, but in this case Vec is actually better because:

1. We iterate sequentially 99% of the time
2. Collection is always small (< 100 items)
3. HashMap has higher memory overhead

Vec: O(n) lookup, low memory
HashMap: O(1) lookup, 2x memory for empty buckets

For our use case, Vec is more appropriate. Happy to change if you see a different pattern in the data though!
```

**Outcome:** Reviewer accepts explanation

### Example 3: Asking for Clarification

**Reviewer:**
```
⚠️ This error handling seems incomplete
```

**Your Response:**
```
🤔 Could you clarify what error cases you're concerned about? 

The current implementation handles:
- Database connection failures → 503
- Query timeout → 504  
- Invalid input → 400

Is there a specific case you think is missing, or should we add more context to the error messages?
```

**Outcome:** Reviewer clarifies, you understand concern

## Anti-Patterns to Avoid

❌ **Don't dismiss feedback** - "This is fine, it'll never happen"

❌ **Don't be passive-aggressive** - "Sure, I'll change it even though it's stupid"

❌ **Don't argue forever** - Pick your battles, concede gracefully

❌ **Don't ignore feedback** - Every comment deserves acknowledgment

❌ **Don't take it personally** - "You always criticize my code"

❌ **Don't fix and forget** - Learn from feedback

❌ **Don't get defensive** - "I wrote it this way because..."

## Learning from Feedback

### Track Patterns

If you see the same feedback repeatedly:
```markdown
## Feedback Patterns to Address

1. "Missing error handling" - Appeared 3 times this month
   → Action: Add error handling to coding checklist
   
2. "SQL injection risk" - Appeared 2 times
   → Action: Review SQLx parameterized query patterns
```

### Personal Improvement

```markdown
## Areas to Improve

- ✅ Started adding more tests after feedback
- 🔄 Working on error handling consistency  
- 📚 Learning more about security best practices
```

### Knowledge Sharing

Share what you learned:
```markdown
"I got feedback that I wasn't handling async errors properly. 
Learned about [technique]. Here's what I now do..."
```

## Success Criteria

Receiving reviews is successful when:
- ✅ All feedback acknowledged professionally
- ✅ Issues fixed appropriately
- ✅ Good communication maintained
- ✅ Code quality improved
- ✅ Relationships strengthened
- ✅ Learning occurred

## Remember

- Reviews make code better
- Feedback is a gift
- Professionalism matters
- Fix it right, not fast
- Learn and improve
