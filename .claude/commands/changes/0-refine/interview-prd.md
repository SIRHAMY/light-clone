# Interview PRD

> **Workflow Context:** Part of the changes workflow. Load the `changes` skill for full workflow documentation, templates, and guidance.

Conduct a structured interview to refine a PRD using parallel question-generating agents, then synthesize into an interview and update the PRD with findings.

## Scope

Determine which PRD to interview using this priority:

1. **User specifies path** - If the user provides a path to a PRD file, use that
2. **Current directory** - Look for `*_PRD.md` in the current directory
3. **Interactive** - If multiple PRDs found or none found, ask which to interview

Examples:
- "interview me about the auth prd" -> find and use auth PRD
- "interview changes/001_auth/001_auth_PRD.md" -> that specific file
- (no scope given, in a change folder) -> use the PRD in current folder

## Instructions

Read the PRD file first to understand what's being proposed. Then launch all 4 agents in parallel using a single message with multiple Task tool calls. Each agent generates questions, not answers.

### Agent 1: Requirements Interviewer
```
You are a requirements analyst preparing interview questions for a PRD.

Read the PRD at [path] and generate probing questions about the requirements:

1. **Clarification Questions** - Requirements that are stated but need more detail
   - "You mention [X]. What specifically should happen when [Y]?"
   - "The success criterion says [Z]. How would we measure that exactly?"

2. **Completeness Questions** - Requirements that seem missing
   - "What should happen when [scenario not mentioned]?"
   - "Is [related functionality] in scope or out?"

3. **Constraint Questions** - Boundaries that aren't clear
   - "Are there performance requirements for [feature]?"
   - "What's the maximum [scale factor] we need to support?"

4. **Priority Questions** - When tradeoffs exist
   - "If we can only do [A] or [B] in v1, which matters more?"
   - "Which success criteria are must-have vs nice-to-have?"

Generate 5-8 specific, non-obvious questions. Reference specific parts of the PRD.
Format: Numbered list with brief context for why each question matters.
```

### Agent 2: Edge Cases Interviewer
```
You are a QA engineer preparing interview questions about edge cases.

Read the PRD at [path] and generate questions about boundary conditions and edge cases:

1. **Empty/Zero States** - What happens with no data?
   - "What should users see before they've created any [items]?"
   - "What happens if [list] is empty?"

2. **Boundary Conditions** - Limits and extremes
   - "What's the maximum length for [field]?"
   - "What happens if someone tries to [extreme action]?"

3. **Error Scenarios** - When things go wrong
   - "What should happen if [operation] fails?"
   - "How do we handle [network/service] being unavailable?"

4. **Concurrent/Conflict States** - Multiple actors
   - "What if two users try to [action] at the same time?"
   - "What happens if [data] changes while user is viewing it?"

5. **Invalid Input** - Malformed or unexpected data
   - "How should we handle [invalid input type]?"
   - "What if [required field] is missing?"

Generate 5-8 specific questions about scenarios NOT addressed in the PRD.
Format: Numbered list with the edge case scenario and why it matters.
```

### Agent 3: Tradeoff Interviewer
```
You are a product strategist preparing interview questions about tradeoffs.

Read the PRD at [path] and generate questions that force prioritization decisions:

1. **Scope Tradeoffs** - What's essential vs optional?
   - "If we had to cut one feature, which would it be?"
   - "Is [feature] essential for launch or can it come later?"

2. **Quality Tradeoffs** - Where do we invest polish?
   - "How important is [UX detail] vs shipping faster?"
   - "Should [edge case] be handled gracefully or is an error acceptable?"

3. **Technical Tradeoffs** - Implementation choices
   - "Would you prefer [simple but limited] or [complex but flexible]?"
   - "Is [technical constraint] acceptable if it simplifies implementation?"

4. **User Tradeoffs** - Whose needs win?
   - "If [user type A] and [user type B] have conflicting needs, who wins?"
   - "Is this optimized for new users or power users?"

5. **Timeline Tradeoffs** - What can wait?
   - "What's the minimum viable version of this feature?"
   - "Which success criteria could be deferred to a fast-follow?"

Generate 4-6 tough tradeoff questions that don't have obvious answers.
Format: Present each as a choice between two reasonable options with pros/cons.
```

### Agent 4: User Journey Interviewer
```
You are a UX researcher preparing interview questions about user journeys.

Read the PRD at [path] and generate questions about the actual user experience:

1. **Entry Points** - How do users get here?
   - "How does a user discover this feature?"
   - "What triggers the need for [action]?"

2. **Step-by-Step Flow** - What's the exact sequence?
   - "Walk me through exactly what the user clicks/sees for [task]"
   - "What's on screen at each step of [flow]?"

3. **Decision Points** - Where do users make choices?
   - "What options does the user have at [point]?"
   - "How do they decide between [option A] and [option B]?"

4. **Feedback & Confirmation** - How do users know it worked?
   - "What confirmation does the user see after [action]?"
   - "How do they know [operation] is in progress?"

5. **Recovery & Undo** - What if they make a mistake?
   - "Can users undo [action]? How?"
   - "What happens if they navigate away mid-[flow]?"

Generate 5-7 questions that would reveal gaps in the user experience.
Format: Frame as "walk me through" or "show me" requests where possible.
```

## After Agents Complete: Synthesize and Interview

Collect all agent questions and prepare the interview:

1. **Deduplicate** - Remove questions asking the same thing differently
2. **Prioritize** - Order by importance (blockers first, nice-to-know last)
3. **Group** - Organize into logical sections
4. **Estimate** - Aim for 12-18 total questions (trim if too many)

### Interview Format

Present the interview to the user:

```
## PRD Interview: [PRD Name]

I've analyzed the PRD and have questions to help refine it. We'll go section by section.
Answer as much or as little as you'd like - say "skip" to move on, "done" when you've had enough.

### Requirements & Scope
1. [Question]
2. [Question]
...

### Edge Cases & Error Handling
1. [Question]
2. [Question]
...

### Tradeoffs & Priorities
1. [Question]
2. [Question]
...

### User Experience
1. [Question]
2. [Question]
...

Ready to begin?
```

### Conducting the Interview

1. **Ask questions one section at a time** - Don't overwhelm with all questions at once
2. **Listen and probe** - If an answer reveals new complexity, ask follow-ups
3. **Note decisions** - Track explicit decisions made ("we decided X over Y because Z")
4. **Track open items** - If user doesn't know, add to Open Questions
5. **Respect "skip" and "done"** - Don't push if user wants to move on

### After Interview: Update PRD

When the interview concludes:

1. **Summarize findings** - Brief recap of key decisions and new information
2. **Propose PRD updates** - Show specific edits to make:
   - New success criteria discovered
   - Scope clarifications (in/out)
   - Constraints identified
   - Open questions resolved or added
   - Edge cases to document
3. **Ask permission** - "Should I update the PRD with these changes?"
4. **Apply edits** - Update the PRD file with agreed changes
5. **Note interview completion** - Add a note that interview was conducted

### PRD Update Format

When updating the PRD, add an Interview Notes section if significant context was captured:

```markdown
## Interview Notes

_Interview conducted: YYYY-MM-DD_

### Key Decisions
- [Decision 1]: [Rationale]
- [Decision 2]: [Rationale]

### Deferred Items
- [Item]: [Why deferred, when to revisit]
```
