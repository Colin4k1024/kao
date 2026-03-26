---
name: "brainstorming"
description: "Refines rough ideas through Socratic questioning, explores alternatives, presents design in digestible chunks for validation. Activates before writing any code or making major decisions."
---

# Brainstorming

The brainstorming skill kicks in whenever you're starting something new, trying to understand a problem, or making a significant decision. It's designed to prevent premature coding and ensure we build the right thing.

## When to Use

Activate brainstorming when:
- User presents a rough idea or feature request
- Problem statement is vague or incomplete
- Multiple approaches are possible and trade-offs need exploration
- User says things like "I want to build...", "Can we add...", "What if we..."
- Before creating any SPEC.md or writing any code

## The Process

### 1. Socratic Discovery

Don't accept the first formulation. Ask questions that expose ambiguity, challenge assumptions, and reveal the true underlying need.

**Key questions to ask:**
- "What problem are we trying to solve?"
- "Who will use this and what do they need?"
- "How will we know we've succeeded?"
- "What are the constraints?"
- "What could go wrong?"
- "Are there similar features we can learn from?"

**Listen for:**
- Vague language that needs concretizing ("fast", "secure", "easy")
- Missing context (who, what, when, where, why)
- Assumptions that need validation
- Edge cases and error conditions
- Success metrics and acceptance criteria

### 2. Alternative Exploration

Before diving into a single solution, briefly consider alternatives. This isn't exhaustive analysis—it's checking that we're not missing obvious approaches.

Ask: "What are the alternatives?" Consider:
- Building vs buying vs integrating
- Different data models
- Different user flows
- Different technology choices
- Different scope (MVP vs full feature)

### 3. Chunked Presentation

When presenting a design or plan, break it into digestible pieces. Don't dump a complete spec on the user at once.

**Presentation strategy:**
- Start with the problem and goals (1-2 sentences)
- Present core concepts and data model first
- Show user flow diagram (if applicable)
- Discuss API design (if applicable)
- Review edge cases and error handling
- Leave detailed specifications for later

### 4. Active Validation

After each chunk, explicitly ask for validation. Don't assume silence means agreement.

Use phrases like:
- "Does this approach make sense?"
- "Are we aligned on this?"
- "Any concerns about this direction?"
- "Should I continue, or do you want to adjust here?"

## Anti-Patterns to Avoid

❌ **Don't start coding immediately** - Even if the request seems simple, go through the discovery process.

❌ **Don't present complete solutions** - Let the design emerge through dialogue.

❌ **Don't accept vague requirements** - Push for specificity until everyone knows what success looks like.

❌ **Don't skip alternatives** - At least mention the option of doing nothing or a simpler approach.

❌ **Don't over-document** - A lightweight design doc is often better than an elaborate spec for a small feature.

## Output

The output of brainstorming is a shared understanding, typically captured as a lightweight design document. This becomes input for the writing-plans skill.

Save the design document as `docs/plans/[date]-[brief-name].md` in your project.

## Success Criteria

The brainstorming session is successful when:
- ✅ The problem is clearly understood by both parties
- ✅ Success criteria are explicit and measurable
- ✅ Key decisions are documented
- ✅ User has explicitly approved the direction
- ✅ You have a clear signal to proceed to planning

## Examples

### Example 1: Feature Request

**User says:** "I want to add notifications to our app."

**Your response:**
"Let's explore this. Before I plan anything, I want to understand the use case better.

1. Who will receive these notifications - users, admins, both?
2. What triggers notifications - system events, user actions, scheduled reminders?
3. Should notifications be real-time (push) or pull-based (in-app)?
4. Do we need notification history/archival?
5. Any constraints around privacy or rate limiting?

Take me through a typical user journey where notifications would be valuable..."

### Example 2: Bug Report

**User says:** "The login is broken."

**Your response:**
"Let's make sure I understand the scope. Before I start debugging:

1. Which login? (email/password, OAuth, SSO?)
2. What does 'broken' mean? (error message, silent failure, timeout?)
3. Is this happening for all users or specific ones?
4. When did it start breaking? (after a deploy, gradually, suddenly?)
5. What's the impact? (can't log in at all, specific features fail?)

Can you walk me through the exact steps to reproduce this?"

## Remember

The goal of brainstorming is not to write a perfect spec—it's to make sure we're solving the right problem in a reasonable way. A 10-minute conversation now can save hours of building the wrong thing.
