# Critique PRD

> **Workflow Context:** Part of the changes workflow. Load the `changes` skill for full workflow documentation, templates, and guidance.

Run an adversarial review of a PRD using parallel critic agents, then synthesize findings into a prioritized report. Use this after the interview phase, before writing the SPEC.

## Scope

Determine which PRD to critique using this priority:

1. **User specifies path** - If the user provides a path to a PRD file, use that
2. **Current directory** - Look for `*_PRD.md` in the current directory
3. **Interactive** - If multiple PRDs found or none found, ask which to critique

Examples:
- "critique the auth prd" → find and critique auth PRD
- "critique changes/001_auth/001_auth_PRD.md" → that specific file
- (no scope given, in a change folder) → critique the PRD in current folder

## Instructions

Read the PRD file first to understand what's being specified. Then launch all 5 agents in parallel using a single message with multiple Task tool calls:

### Agent 1: Requirements Completeness Critic
```
You are a requirements analyst critiquing a PRD for completeness.

Read the PRD at [path] and identify gaps in requirements:

1. **Missing Scenarios** - User flows, states, or interactions not addressed
2. **Untestable Criteria** - Success criteria that can't be objectively verified
3. **Unstated Assumptions** - Things the PRD assumes but doesn't document
4. **Missing Acceptance Criteria** - Features without clear "done" definition
5. **Undefined Behaviors** - What happens in edge cases, error states, empty states

For each issue found, report:
- Issue: Brief description
- Location: Which section of the PRD
- Severity: Critical / High / Medium
- Suggestion: How to address it

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

### Agent 2: Scope & Boundaries Critic
```
You are a scope analyst critiquing a PRD for clear boundaries.

Read the PRD at [path] and identify scope issues:

1. **Scope Creep Risks** - Features that could easily expand beyond stated bounds
2. **Unclear Boundaries** - Ambiguity about what's in/out of scope
3. **Hidden Dependencies** - External systems, APIs, or features not acknowledged
4. **Implicit Requirements** - Things needed but not explicitly listed
5. **Version/Phase Confusion** - Unclear what's MVP vs future

For each issue found, report:
- Issue: Brief description
- Location: Which section of the PRD
- Severity: Critical / High / Medium
- Suggestion: How to address it

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

### Agent 3: Feasibility & Constraints Critic
```
You are a technical feasibility analyst critiquing a PRD.

Read the PRD at [path] and identify feasibility concerns:

1. **Technical Blockers** - Requirements that may be impossible or extremely difficult
2. **Unrealistic Constraints** - Time, performance, or resource constraints that conflict
3. **Missing Technical Context** - Platform, language, or framework assumptions not stated
4. **Integration Challenges** - Difficult integrations glossed over
5. **Scaling Concerns** - Requirements that won't scale as described

For each issue found, report:
- Issue: Brief description
- Location: Which section of the PRD
- Severity: Critical / High / Medium
- Suggestion: How to address it

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

### Agent 4: Edge Cases & Failure Modes Critic
```
You are a quality analyst critiquing a PRD for robustness.

Read the PRD at [path] and identify unaddressed edge cases and failures:

1. **Unhandled Edge Cases** - Boundary conditions, empty states, maximum limits
2. **Failure Scenarios** - Network failures, invalid input, concurrent access
3. **Recovery Requirements** - What happens after failure? How to resume?
4. **Data Integrity** - What if data is corrupted, partial, or inconsistent?
5. **Security Scenarios** - Malicious input, unauthorized access, data leaks

For each issue found, report:
- Issue: Brief description
- Location: Which section of the PRD (or "Missing section")
- Severity: Critical / High / Medium
- Suggestion: How to address it

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

### Agent 5: Clarity & Ambiguity Critic
```
You are a technical writer critiquing a PRD for clarity.

Read the PRD at [path] and identify clarity issues:

1. **Vague Language** - Words like "fast", "easy", "simple", "etc." without definition
2. **Undefined Terms** - Domain terms or acronyms used without explanation
3. **Contradictions** - Requirements that conflict with each other
4. **Ambiguous Pronouns** - "It", "this", "that" with unclear referents
5. **Missing Context** - Statements that require unstated background to understand

For each issue found, report:
- Issue: Brief description with the problematic text quoted
- Location: Which section of the PRD
- Severity: Critical / High / Medium
- Suggestion: How to clarify

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

## After Agents Complete: Synthesize Findings

Collect all agent results and produce a prioritized report:

1. **Aggregate issues** - Combine all issues from all agents
2. **Deduplicate** - Merge issues that are essentially the same finding
3. **Rank by severity** - Critical first, then High, then Medium
4. **Group by type** - Organize by the category of issue
5. **Identify patterns** - Note if multiple agents flagged related concerns
6. **Give verdict** - Based on severity distribution

### Output Format

```
## PRD Critique: [PRD Name]

### Critical Issues (must address before writing SPEC)
1. [Category] Issue title
   - Description
   - Location: section/line
   - Suggestion: how to fix

### High Priority Issues (should address before writing SPEC)
1. [Category] Issue title
   - Description
   - Location: section/line
   - Suggestion: how to fix

### Medium Priority Issues (consider addressing)
1. [Category] Issue title
   - Description
   - Location: section/line
   - Suggestion: how to fix

### Patterns Identified
- [Any cross-cutting concerns multiple agents flagged]

### Summary
- Total issues: X (Y Critical, Z High, W Medium)
- Agents that found issues: [list]
- Agents with no findings: [list]

### Verdict: [Ready for SPEC | Needs Refinement | Needs Major Rework]
[One sentence explaining the verdict and recommended next step]
```

### Verdict Guidelines

- **Ready for SPEC** - No critical issues, few or no high issues, PRD is clear enough to write a technical spec against
- **Needs Refinement** - Has high-priority issues or multiple medium issues that should be addressed; recommend another interview round focused on the gaps
- **Needs Major Rework** - Has critical issues or many high issues; PRD is not ready for technical specification and needs significant revision
