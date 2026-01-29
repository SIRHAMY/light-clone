# SPEC: [Change Name]

**ID:** NNN
**Status:** Draft | In Progress | Complete | Abandoned
**Created:** YYYY-MM-DD
**PRD:** ./NNN_featurename_PRD.md
**Execution Mode:** human-in-the-loop | autonomous
**New Agent Per Phase:** yes | no
**Max Review Attempts:** 3

## Context

[Why are we implementing this now? Any background the AI needs to understand the approach. Link to relevant code, patterns, or prior implementations in the codebase.]

## Approach

<!-- High-level architecture and strategy. Answer: What are we building? How does it fit into the existing system? What's the overall design? -->
<!-- Keep this concise - a paragraph or two plus an optional diagram. Cover: overall strategy, key architectural decisions, how new components connect to existing system, core patterns. -->
<!-- For deeper details (type definitions, detailed rationale, alternatives considered), see the Design Details appendix at the bottom. -->

**Patterns to follow:**

<!-- Reference existing code that new implementation should model. This helps implementers match existing conventions without exploring. -->

- `path/to/existing/file.ts` — [what pattern it demonstrates]
- `path/to/another/file.ts` — [what pattern it demonstrates]

**Implementation boundaries:**

<!-- What is explicitly out of scope for this implementation? Prevents scope creep. -->

- Do not modify: [files/modules that should not be touched]
- Do not refactor: [existing code that should be left as-is even if imperfect]

## Open Questions

<!-- Questions discovered during spec creation that need human input before implementation. -->
<!-- Do NOT proceed with implementation until all open questions are resolved. -->
<!-- Delete this section once all questions are answered and decisions are reflected in the spec. -->

- [ ] [Question about requirement, approach, or tradeoff that needs human decision]

## Phase Summary

<!-- List all phases upfront for easy scanning. Include ordering rationale if non-obvious. -->

| Phase | Name | Complexity | Description |
|-------|------|------------|-------------|
| 1 | [Phase Name] | Low / Med / High | [One-line description of what this phase accomplishes] |
| 2 | [Phase Name] | Low / Med / High | [One-line description] |
| 3 | [Phase Name] | Low / Med / High | [One-line description] |

**Ordering rationale:** [Optional - explain why phases are in this order if it might be unclear, e.g., "Phase 2 depends on the types defined in Phase 1" or "Infrastructure must be set up before feature work"]

---

## Phases

Each phase should leave the codebase in a functional, stable state. Complete and verify each phase before moving to the next.

**Note:** If the project lacks test infrastructure, add a phase to set it up before phases that need tests.

---

### Phase 1: [Phase Name]

> [One-line description for quick scanning - same as in summary table]

**Complexity:** Low / Med / High

**Goal:** [What this phase accomplishes - one sentence, can be slightly more detailed than the description]

**Files:**

- `path/to/file.ts` — create / modify — [what changes]
- `path/to/other.ts` — modify — [what changes]
- `path/to/test.test.ts` — create — [tests for what]

**Patterns:** <!-- Optional - reference specific code to follow for this phase. Omit if covered by Approach section. -->

- Follow `path/to/example.ts` for [pattern description]

**Tasks:**

- [ ] Task A
- [ ] Task B
- [ ] Task C
- [ ] Write tests for [testable logic added in this phase]

**Verification:**

- [ ] [How we know this phase is complete]
- [ ] Tests pass
- [ ] Codebase builds/runs without errors
- [ ] Code review passes (`/code-review` → fix issues → repeat until pass)

**Commit:** `[ID][P1] Type: Description` (Type: Feature | Fix | Clean | Docs)

**Notes:**

[Implementation decisions, gotchas discovered during work]

---

### Phase 2: [Phase Name]

> [One-line description for quick scanning]

**Complexity:** Low / Med / High

**Goal:** [What this phase accomplishes]

**Files:**

- `path/to/file.ts` — create / modify — [what changes]
- `path/to/test.test.ts` — create — [tests for what]

**Patterns:** <!-- Optional - omit if covered by Approach section -->

- Follow `path/to/example.ts` for [pattern description]

**Tasks:**

- [ ] Task A
- [ ] Task B
- [ ] Write tests for [testable logic added in this phase]

**Verification:**

- [ ] [How we know this phase is complete]
- [ ] Tests pass (including no regressions)
- [ ] Code review passes (`/code-review` → fix issues → repeat until pass)

**Commit:** `[ID][P2] Type: Description` (Type: Feature | Fix | Clean | Docs)

**Notes:**

---

## Final Verification

- [ ] All phases complete
- [ ] All PRD success criteria met
- [ ] Tests pass
- [ ] No regressions introduced
- [ ] Code reviewed (if applicable)

## Execution Log

<!-- Updated automatically during autonomous execution via /implement-spec -->
<!-- Each phase agent appends an entry when it completes -->

| Phase | Status | Commit | Notes |
|-------|--------|--------|-------|

## Design Details

<!-- Appendix: deeper technical details referenced by the Approach section. -->
<!-- Not everything needs to go here - only include sections that are relevant. -->

### Key Types

[Define the core types/interfaces/data structures introduced or modified by this change. Include field descriptions where non-obvious.]

```
[language-appropriate type definitions]
```

### Architecture Details

[Deeper explanation of the system design - data flow, component interactions, state management, etc. Include diagrams if helpful.]

### Design Rationale

[Why this approach over alternatives? What tradeoffs were made? What constraints drove the design?]

---

## Retrospective

[Fill in after completion]

### What worked well?

### What was harder than expected?

### What would we do differently next time?
