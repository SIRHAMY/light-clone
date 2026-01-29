# Development Workflow

This template provides a structured approach for AI-assisted development using PRDs and SPECs.

## Core Concepts

| Document | Purpose | Focus |
|----------|---------|-------|
| **Product PRD** | High-level requirements for your project | You create this at project root |
| **Change PRD** | Desired outcome for a specific change | What, not how |
| **Change SPEC** | Implementation strategy AI follows | How, in phases |

## Workflow

```
Product PRD → Draft PRD → /interview-prd → /critique-prd → Write SPEC → /critique-spec → /implement-spec → Verify
     ↑              ↑            ↑               ↑              ↑              ↑
  (project root) (per change) (refine)      (adversarial)  (per change)  (adversarial)
```

The interview and critique stages are optional but recommended. They use parallel agents to surface gaps and find problems before they become expensive to fix.

## Directory Structure

```
your-project/
├── PRD.md               # Product-level PRD (you create this)
├── CLAUDE.md            # AI instructions for your project
└── changes/
    ├── _TEMPLATE_NNN_featurename/  # Copy this for new changes
    │   ├── NNN_featurename_PRD.md
    │   └── NNN_featurename_SPEC.md
    ├── 001_dark-mode/
    │   ├── 001_dark-mode_PRD.md    # What this change accomplishes
    │   └── 001_dark-mode_SPEC.md   # How to implement it
    └── 002_user-auth/
        ├── 002_user-auth_PRD.md
        └── 002_user-auth_SPEC.md
```

## Creating a New Change

1. **Create folder** - Copy `_TEMPLATE_NNN_featurename/` to `changes/NNN_your-feature/` and rename files to `NNN_your-feature_PRD.md` and `NNN_your-feature_SPEC.md`
2. **Draft PRD** - You write initial PRD with desired outcome and success criteria
3. **Interview** - AI interviews you to fill gaps, surface edge cases, and clarify tradeoffs
4. **Critique PRD** (optional) - Run `/critique-prd` for adversarial review; address critical/high issues
5. **Write SPEC** - Break implementation into atomic phases (each leaves codebase in a good state with verifications)
6. **Critique SPEC** (optional) - Run `/critique-spec` for adversarial review; address critical/high issues
7. **Implement** - Work through phases, checking tasks and verifications
8. **Verify** - Confirm all PRD success criteria are met
9. **Retrospective** - Document learnings in SPEC

## Document Lifecycle

### PRD Status

| Status | Meaning |
|--------|---------|
| Proposed | Change under consideration |
| Approved | Ready for technical specification/implementation |
| Implemented | Change is complete |
| Rejected | Decided not to implement (reason documented) |

### SPEC Status

| Status | Meaning |
|--------|---------|
| Draft | SPEC being written |
| In Progress | Currently implementing |
| Complete | All phases done, verified |
| Abandoned | Stopped work (reason documented) |

## Working with AI

### Refining a PRD (interview)

After drafting your PRD, run a structured interview to ensure completeness:

```
/changes:0-refine:interview-prd changes/NNN_feature/NNN_feature_PRD.md
```

**What it does:**
1. Spawns 4 parallel agents to generate targeted questions:
   - **Requirements Interviewer** - Clarifications, completeness, constraints, priorities
   - **Edge Cases Interviewer** - Empty states, boundaries, errors, conflicts
   - **Tradeoff Interviewer** - Scope, quality, technical, timeline tradeoffs
   - **User Journey Interviewer** - Entry points, flows, feedback, recovery
2. Synthesizes questions into a structured interview (12-18 questions)
3. Conducts the interview section by section
4. Updates the PRD with findings

This surfaces hidden requirements, edge cases, and tradeoffs that lead to better PRDs and fewer surprises during implementation.

**Alternative (ad-hoc interview):**

For a lighter touch, just ask:
```
"Review the PRD in changes/NNN_feature/ and interview me to fill any gaps."
```

### Planning implementation

```
"Read the PRD in changes/NNN_feature/ and create a SPEC."
```

### Adversarial Critique (optional)

Run critique commands to get an adversarial review before proceeding to the next stage. These spawn parallel critic agents that look for problems from different angles.

#### `/critique-prd` - After Interview, Before Writing SPEC

```
/changes:0-refine:critique-prd changes/NNN_feature/NNN_feature_PRD.md
```

**When to use:**
- Complex features with many requirements
- Changes with unclear scope or dependencies
- High-risk changes where missing requirements are costly

**What it checks:**
- Requirements completeness (missing scenarios, untestable criteria)
- Scope & boundaries (scope creep risks, hidden dependencies)
- Feasibility & constraints (technical blockers, unrealistic constraints)
- Edge cases & failure modes (unaddressed scenarios)
- Clarity & ambiguity (vague language, contradictions)

**Verdicts:**
- **Ready for SPEC** - No critical issues, PRD is clear enough to write a technical spec against
- **Needs Refinement** - Has high-priority issues; do another interview round
- **Needs Major Rework** - Has critical issues; significant revision needed

#### `/critique-spec` - After Writing SPEC, Before Implementation

```
/changes:1-design:critique-spec changes/NNN_feature/NNN_feature_SPEC.md
```

**When to use:**
- Multi-phase implementations
- SPECs with complex dependencies between phases
- Changes where implementation order matters

**What it checks:**
- Phase completeness (missing tasks, incomplete verification)
- Dependency & ordering (wrong order, circular dependencies)
- Risk & failure modes (missing rollback strategies)
- PRD alignment (coverage gaps, scope drift)
- Testability & verification (untested paths, vague checks)
- Effort & complexity (over/under-scoped phases)

**Verdicts:**
- **Ready to Implement** - No critical issues, PRD criteria covered
- **Needs Revision** - Has high-priority issues; revise affected phases
- **Needs Major Rework** - Fundamental structural problems

#### Advisory Nature

Both critiques are advisory. The agents find potential problems; you decide which to address. Focus on:
1. **Critical issues** - Must address before proceeding
2. **High issues** - Should address before proceeding
3. **Medium issues** - Consider addressing, use judgment

## AI Instructions

These instructions are for the AI to follow when working with this workflow.

### After creating or reviewing a PRD

When you create a new PRD or review an existing one:

1. **Always prompt for interview** - After creating/reviewing, say: "I've created/reviewed the PRD. There are open questions and areas that would benefit from clarification. Ready to do the interview to fill gaps and surface edge cases?"

2. **List what needs clarification** - Identify specific areas:
   - Open Questions section items
   - Ambiguous success criteria
   - Unclear scope boundaries
   - Missing technical constraints
   - Potential edge cases

### Before creating a SPEC

When asked to create a SPEC, first check if interview was completed:

1. **Check for interview completion** - Look for signs the PRD has been refined:
   - Open Questions section should be mostly resolved (checked off or answered inline)
   - Success criteria should be specific and testable
   - Scope should be clearly defined

2. **Warn if interview appears skipped** - If the PRD still has unresolved open questions or appears to be a rough draft, say: "The PRD still has open questions/unclear areas. Are you sure you want to proceed to writing the SPEC before doing the clarification interview? The interview helps surface hidden requirements and edge cases that lead to better SPECs."

3. **Proceed if confirmed** - If user confirms they want to skip, proceed with SPEC writing but note any assumptions you're making due to unclear requirements.

### When creating a SPEC

**CRITICAL: Follow the SPEC template structure exactly.** Every SPEC must have well-defined phases with explicit checklists and verification criteria. Do not produce flat task lists or prose descriptions.

#### Required Structure (non-negotiable)

1. **Approach section** - Before the phases, include a high-level architecture summary:
   - What are we building and how does it fit into the existing system?
   - Key architectural decisions and patterns
   - Reference existing files/modules by name to show integration points
   - Keep it concise (a paragraph or two). Point to the **Design Details** appendix for deeper information.
   - **Patterns to follow** - Reference existing code files that new implementation should model. This eliminates guesswork for the implementer:
     ```markdown
     **Patterns to follow:**
     - `src/services/userService.ts` — service layer pattern with error handling
     - `src/routes/authRoutes.ts` — route registration and middleware chain
     ```
   - **Implementation boundaries** - Explicitly state what should NOT be modified or refactored:
     ```markdown
     **Implementation boundaries:**
     - Do not modify: src/core/database.ts (stable, shared infrastructure)
     - Do not refactor: existing auth middleware (out of scope, works fine)
     ```

2. **Design Details appendix** - At the bottom (before Retrospective), include detailed design information:
   - **Key Types** - Core types/interfaces being introduced or modified (with code blocks)
   - **Architecture Details** - Deeper data flow, component interactions, diagrams
   - **Design Rationale** - Why this approach, tradeoffs made, alternatives considered
   - Only include subsections that are relevant. Skip empty ones.

3. **Phase summary table** - Before the phases section, include a summary table listing all phases with complexity:
   ```markdown
   | Phase | Name | Complexity | Description |
   |-------|------|------------|-------------|
   | 1 | Setup test infra | Low | Add Jest and configure for TypeScript |
   | 2 | Add auth types | Low | Define User, Session, and auth result types |
   | 3 | Implement login | High | Create login flow with validation and error handling |
   ```
   Complexity is `Low` / `Med` / `High` — helps reviewers know where to focus attention.
   Include an **Ordering rationale** line if the phase order might be unclear (e.g., dependencies between phases).

4. **Split into phases** - Every SPEC must be broken into discrete phases. Each phase:
   - Accomplishes one logical unit of work
   - Leaves the codebase in a functional, stable state
   - Can be reviewed and committed independently
   - Starts with a one-line description (blockquote) for quick scanning
   - Has a complexity flag (`Low` / `Med` / `High`)

5. **Files touched per phase** - Each phase must list the files it will create or modify:
   ```markdown
   **Files:**
   - `src/auth/types.ts` — create — auth types and interfaces
   - `src/auth/login.ts` — modify — add session validation
   - `src/auth/login.test.ts` — create — tests for login flow
   ```
   This helps reviewers assess scope and helps the implementer avoid unnecessary exploration.

6. **Patterns per phase** (optional) - If a phase needs specific code to follow that isn't covered by the Approach section's patterns, reference it:
   ```markdown
   **Patterns:**
   - Follow `src/services/userService.ts` for service layer structure
   ```
   Omit if the Approach-level patterns are sufficient.

7. **Explicit task checklists** - Each phase must have a `**Tasks:**` section with checkbox items:
   ```markdown
   **Tasks:**
   - [ ] Specific action item 1
   - [ ] Specific action item 2
   - [ ] Write tests for [testable logic in this phase]
   ```
   Tasks must be concrete and actionable, not vague ("implement feature" is bad; "add UserAuth component with login/logout methods" is good).

8. **Verification criteria** - Each phase must have a `**Verification:**` section that includes ALL of these:
   ```markdown
   **Verification:**
   - [ ] [Phase-specific success criterion - how we know the goal is met]
   - [ ] Lint passes (run linter, fix any issues)
   - [ ] Tests pass (including new tests from this phase)
   - [ ] Codebase builds/runs without errors
   - [ ] Code review passes (`/code-review` → fix issues → repeat until pass)
   ```
   Do not skip verification items. If linting isn't set up, add a phase to set it up first.

#### Content Guidelines

9. **Include tests in each phase** - When a phase adds testable logic, include tasks for writing tests in that same phase. Don't create a separate "add tests" phase at the end.

10. **Check for test infrastructure** - If the project doesn't have a test setup:
    - Make "Set up test infrastructure" its own phase (typically Phase 1 or early)
    - This phase includes: choosing framework, configuration, first example test
    - All subsequent phases that add logic should include tests

11. **Check for lint infrastructure** - If the project doesn't have linting:
    - Include lint setup in the test infrastructure phase or as its own early phase
    - All subsequent phases must pass linting in their verification

12. **Consider testability** - If logic seems hard to test, consider whether the phase should include refactoring for testability:
   - Extract pure functions from impure code
   - Add dependency injection where needed
   - Note these as explicit tasks in the phase

#### What NOT to do

- Do NOT skip the Approach section or leave it as a single vague sentence
- Do NOT omit patterns to follow or implementation boundaries from the Approach section
- Do NOT omit the Design Details appendix when the change introduces types, new architecture, or non-obvious design decisions
- Do NOT omit the phase summary table at the top
- Do NOT produce a single phase with all tasks lumped together
- Do NOT omit the one-line description at the start of each phase
- Do NOT omit the files list from any phase
- Do NOT omit the complexity flag from phases or the summary table
- Do NOT omit the verification section or its required items
- Do NOT use vague task descriptions ("finish implementation")
- Do NOT skip code review in verification criteria
- Do NOT forget lint and test verification items

### Executing phases

When implementing a SPEC:

1. **Spawn a new agent for each phase** - Use the Task tool to create a fresh agent for each phase. This is the default behavior regardless of execution mode (human-in-the-loop or autonomous).

   Example prompt for spawning phase agent:
   ```
   "Implement Phase N of changes/NNN_feature/. Read the SPEC at [path] and the PRD it references to understand the 'why'. Execute only Phase N tasks. Mark tasks complete as you go. After tasks are done, run the Verification checklist yourself (lint, tests, build), then run /code-review. Fix critical/high issues and re-run review until it passes. Triage medium/low items — fix quick wins, note the rest for later."
   ```

2. **Do not implement phases yourself** - The orchestrating agent should coordinate, not implement. Each phase gets its own agent with fresh context.

3. **Wait for phase completion** - Before spawning the next phase agent:
   - Verify previous phase tasks are marked complete
   - Verify the subagent ran verification and code review (check REVIEW_VERDICT in its return)
   - In human-in-the-loop mode: wait for human approval
   - In autonomous mode: proceed automatically

4. **Only skip new agents if explicitly configured** - If "New Agent Per Phase: no" is set in the SPEC, then implement phases directly. This is rare.

### Starting implementation

Use the SPEC's default execution mode:

```
"Implement changes/NNN_feature/"
```

Override the SPEC's mode:

```
"Implement changes/NNN_feature/ in autonomous mode"
"Implement changes/NNN_feature/ in human-in-the-loop mode"
```

### Resuming work

```
"Continue on changes/NNN_feature/. Check the SPEC for current phase."
```

### Checking status

```
"What's the status of all active changes?"
```

## Key Principles

1. **Interview to refine PRD** - AI interviews after draft to surface hidden requirements and tradeoffs
2. **Critique to find problems early** - Adversarial review catches issues before they're expensive to fix
3. **PRD before SPEC** - Define success criteria before implementation approach
4. **Atomic phases** - Each phase is one logical unit of change, reviewable as its own PR. It leaves the codebase in a good state (compiles, runs) with verifications. If a phase is too large to review easily, split it.
5. **Tests within phases** - If a phase adds testable logic, include tests in that phase. Don't push all testing to the end.
6. **Verification at each phase** - Don't proceed until current phase is stable
7. **Code review at each phase** - Run `/code-review` after completing phase tasks; fix issues until it passes
8. **Retrospectives** - Capture learnings for future changes

## Testing in Phases

Tests belong in the phase where logic is added, not pushed to a final "add tests" phase. This ensures each phase is truly self-contained and verifiable.

### Guidelines

1. **Test with the code** - When a phase adds testable logic, include tests for that logic in the same phase.

2. **Make untestable code testable** - If logic seems hard to test, consider restructuring:
   - **Impure-Pure-Impure Sandwich** - Extract pure logic from impure (I/O, side effects) code. Test the pure core.
   - **Dependency Injection** - Pass dependencies as parameters rather than hardcoding them. Inject test doubles.
   - **Extract functions** - Pull testable units out of larger, harder-to-test functions.

3. **Test in the closest phase** - If testing truly isn't possible in the current phase (e.g., requires integration with code from a later phase), add tests in the earliest phase where it makes sense.

4. **Test infrastructure is its own phase** - If the project doesn't have a test setup yet:
   - Create a dedicated phase for adding the test project/framework
   - This is one logical unit of change: choosing framework, configuring, adding first example test
   - Subsequent phases can then include tests alongside their implementation

## Execution Modes

The workflow supports two execution modes, set in the SPEC and overridable at runtime:

| Mode | Behavior | Use When |
|------|----------|----------|
| **human-in-the-loop** | Pause for human review before each commit | Default. You want to review each phase before it's committed. |
| **autonomous** | Auto-commit and proceed after code review passes | You trust the automated checks and want faster iteration. |

### Phase Execution Flow

```
Phase Tasks Complete
        ↓
  Run /code-review
        ↓
    ┌───────────────────────────────────────┐
    │ Verdict: "Ready to Merge"?            │
    └───────────────────────────────────────┘
           │                    │
          YES                   NO
           ↓                    ↓
    ┌─────────────┐     ┌─────────────────────┐
    │ Mode?       │     │ Attempts < max?     │
    └─────────────┘     └─────────────────────┘
      │         │              │           │
  human-in   autonomous       YES          NO
   -the-loop    │              ↓           ↓
      ↓         ↓         Fix issues    Pause for
   Pause for  Auto-commit  and re-run   human help
   review     and proceed  /code-review
      ↓
   Commit
```

### Configuration

Set these in the SPEC header:

- **Execution Mode:** `human-in-the-loop` (default) or `autonomous`
- **New Agent Per Phase:** `yes` (default) or `no` - whether to start a fresh agent for each phase
- **Max Review Attempts:** Number of code review cycles before pausing (default: 3)

### New Agent Per Phase

By default, each phase is executed by a new agent instance. **This applies to both human-in-the-loop and autonomous modes.** See the "Executing phases" section in AI Instructions for how to implement this.

**Why new agents per phase:**

- **Fresh context** - No accumulated context from previous phases that might confuse the implementation
- **Full memory** - The PRD (for the "why") and SPEC (for the "how") contain everything needed; no conversation history required
- **Consistent approach** - Each phase starts clean with the same documented understanding

**Important:** The orchestrating agent should NOT implement phases directly. It should spawn a new agent for each phase using the Task tool, then coordinate the workflow (checking completion, handling approvals).

Set to `no` only when phases have tight interdependencies that benefit from shared conversation context (rare).

### Execution Commands

Two commands are available for executing SPECs:

#### `/changes:2-build:implement-spec` - Human-in-the-Loop Mode

Execute phases in a single context, stopping for human review between invocations.

```
/changes:2-build:implement-spec <spec-path> [phase-range]
```

**Examples:**
```
/changes:2-build:implement-spec changes/001_auth/auth_SPEC.md 1-3    # Phases 1-3
/changes:2-build:implement-spec changes/001_auth/auth_SPEC.md 2      # Just phase 2
/changes:2-build:implement-spec changes/001_auth/auth_SPEC.md        # All remaining phases
```

**Workflow:**
1. Human invokes with a phase range
2. AI executes those phases in current context
3. AI stops after completing the range
4. Human reviews the work
5. Human invokes again with next range when ready

**Best for:** Complex phases requiring careful review, critical features, learning a new codebase.

#### `/changes:2-build:implement-spec-autonomous` - Autonomous Mode

Execute phases using a stop-hook loop pattern with fresh context each iteration. Scales to 100+ phases without memory issues.

```
/changes:2-build:implement-spec-autonomous <spec-path> [phase-range]
```

**Examples:**
```
/changes:2-build:implement-spec-autonomous changes/001_auth/auth_SPEC.md       # All phases
/changes:2-build:implement-spec-autonomous changes/001_auth/auth_SPEC.md 4-6   # Phases 4-6
```

**Workflow:**
1. Command creates a state file that activates the stop hook
2. Each iteration reads the SPEC fresh, executes one phase
3. Stop hook re-feeds the prompt for the next iteration
4. Continues until range complete or `SPEC_COMPLETE`
5. State file cleaned up automatically

**Best for:** Long SPECs with 10+ phases, well-tested patterns, overnight execution.

#### Phase Range Syntax

| Format | Meaning |
|--------|---------|
| `1-3` | Phases 1, 2, and 3 |
| `2` | Just phase 2 |
| (none) | All remaining phases |

#### Choosing a Mode

| Consideration | HITL | Autonomous |
|---------------|------|------------|
| Context per phase | Shared (accumulates) | Fresh (no accumulation) |
| Memory usage | Grows with phases | Constant |
| Human oversight | After each range | At start/end only |
| Best for | 1-5 phases, critical work | 10+ phases, routine work |

#### Cancel Autonomous Execution

```
/cancel-spec
```

#### State Persistence

All state persists in files:
- SPEC checkboxes track task completion
- Git history tracks commits
- Execution Log section in SPEC tracks phase status

This means execution is **resumable** - if interrupted, just run the command again and it picks up from the current SPEC state.
