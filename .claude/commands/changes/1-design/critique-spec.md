# Critique SPEC

> **Workflow Context:** Part of the changes workflow. Load the `changes` skill for full workflow documentation, templates, and guidance.

Run an adversarial review of a SPEC using parallel critic agents, then synthesize findings into a prioritized report. Use this after writing the SPEC, before implementation.

## Scope

Determine which SPEC to critique using this priority:

1. **User specifies path** - If the user provides a path to a SPEC file, use that
2. **Current directory** - Look for `*_SPEC.md` in the current directory
3. **Interactive** - If multiple SPECs found or none found, ask which to critique

The critic agents will also need the corresponding PRD to check alignment. Look for it in the same directory with matching naming convention (e.g., `001_auth_PRD.md` for `001_auth_SPEC.md`).

Examples:
- "critique the auth spec" → find and critique auth SPEC
- "critique changes/001_auth/001_auth_SPEC.md" → that specific file
- (no scope given, in a change folder) → critique the SPEC in current folder

## Instructions

Read the SPEC and its corresponding PRD first to understand the full context. Then launch all 6 agents in parallel using a single message with multiple Task tool calls:

### Agent 1: Phase Completeness Critic
```
You are a project manager critiquing a SPEC for phase completeness.

Read the SPEC at [spec_path] and identify gaps in phase definitions:

1. **Missing Tasks** - Work that needs to happen but isn't listed
2. **Unclear Task Boundaries** - Tasks that are vague about what "done" means
3. **Incomplete Verification** - Phases without adequate verification steps
4. **Missing Cleanup** - Technical debt or TODOs introduced but not addressed
5. **Orphaned Work** - Tasks mentioned in one phase but never completed

For each issue found, report:
- Issue: Brief description
- Phase: Which phase number
- Severity: Critical / High / Medium
- Suggestion: How to address it

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

### Agent 2: Dependency & Ordering Critic
```
You are a systems analyst critiquing a SPEC for correct ordering.

Read the SPEC at [spec_path] and identify dependency issues:

1. **Wrong Phase Order** - Phase B depends on Phase A output but comes before it
2. **Circular Dependencies** - Phases that depend on each other
3. **Missing Prerequisites** - Phases that assume setup not done in earlier phases
4. **Parallelization Opportunities** - Phases that could run in parallel but are sequential
5. **Blocking Dependencies** - External dependencies that could stall progress

For each issue found, report:
- Issue: Brief description
- Phases Affected: Which phase numbers
- Severity: Critical / High / Medium
- Suggestion: How to reorder or restructure

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

### Agent 3: Risk & Failure Modes Critic
```
You are a risk analyst critiquing a SPEC for implementation risks.

Read the SPEC at [spec_path] and identify risks:

1. **Implementation Risks** - Technically difficult tasks without mitigation plans
2. **Missing Rollback Strategies** - Changes that can't be easily undone if they fail
3. **Single Points of Failure** - Tasks where failure blocks all progress
4. **External Risks** - Dependencies on external services, APIs, or teams
5. **Data Risks** - Migrations, transformations, or deletions without safeguards

For each issue found, report:
- Issue: Brief description
- Phase: Which phase number
- Severity: Critical / High / Medium
- Suggestion: Mitigation strategy

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

### Agent 4: PRD Alignment Critic
```
You are a quality analyst checking SPEC alignment with PRD.

Read both the SPEC at [spec_path] and the PRD at [prd_path]. Identify alignment issues:

1. **Missing Success Criteria** - PRD criteria not addressed by any phase
2. **Scope Drift** - Tasks in SPEC that weren't in PRD (feature creep)
3. **Constraint Violations** - SPEC approaches that violate PRD constraints
4. **Incomplete Coverage** - PRD requirements only partially addressed
5. **Assumption Mismatches** - SPEC assumes things PRD doesn't state

For each issue found, report:
- Issue: Brief description
- PRD Reference: Which requirement/criterion
- SPEC Reference: Which phase/task (or "Missing")
- Severity: Critical / High / Medium
- Suggestion: How to align

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

### Agent 5: Testability & Verification Critic
```
You are a QA engineer critiquing a SPEC for testability.

Read the SPEC at [spec_path] and identify testing gaps:

1. **Untested Code Paths** - Logic added without corresponding tests
2. **Vague Verification Steps** - "Verify it works" without specific checks
3. **Missing Test Infrastructure** - Tests assumed but no setup phase
4. **Integration Test Gaps** - Components tested in isolation but not together
5. **Manual-Only Verification** - Checks that should be automated but aren't

For each issue found, report:
- Issue: Brief description
- Phase: Which phase number
- Severity: Critical / High / Medium
- Suggestion: Specific test or verification to add

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

### Agent 6: Effort & Complexity Critic
```
You are a technical lead critiquing a SPEC for realistic scoping.

Read the SPEC at [spec_path] and identify scoping issues:

1. **Over-Scoped Phases** - Phases trying to do too much at once
2. **Under-Scoped Phases** - Phases that seem artificially small (should be combined)
3. **Hidden Complexity** - Simple-looking tasks that are actually complex
4. **Unclear Boundaries** - Tasks that could balloon in scope during implementation
5. **Technical Debt Accumulation** - Spec structure that front-loads debt

For each issue found, report:
- Issue: Brief description
- Phase: Which phase number
- Severity: Critical / High / Medium
- Suggestion: How to rescope

If no issues found in a category, skip it.
End with a summary count: "Found X issues (Y Critical, Z High, W Medium)"
```

## After Agents Complete: Synthesize Findings

Collect all agent results and produce a prioritized report:

1. **Aggregate issues** - Combine all issues from all agents
2. **Deduplicate** - Merge issues that are essentially the same finding
3. **Rank by severity** - Critical first, then High, then Medium
4. **Group by phase** - Show issues per phase for easy addressing
5. **Identify patterns** - Note if multiple agents flagged related concerns
6. **Give verdict** - Based on severity distribution

### Output Format

```
## SPEC Critique: [SPEC Name]

### Critical Issues (must address before implementation)
1. [Category] Issue title
   - Phase: N
   - Description
   - Suggestion: how to fix

### High Priority Issues (should address before implementation)
1. [Category] Issue title
   - Phase: N
   - Description
   - Suggestion: how to fix

### Medium Priority Issues (consider addressing)
1. [Category] Issue title
   - Phase: N
   - Description
   - Suggestion: how to fix

### Phase-by-Phase Summary
| Phase | Issues | Most Severe | Notes |
|-------|--------|-------------|-------|
| 1     | 2      | High        | Missing tests |
| 2     | 0      | -           | Clean |
| ...   | ...    | ...         | ... |

### Patterns Identified
- [Any cross-cutting concerns multiple agents flagged]

### PRD Alignment
- Success criteria covered: X/Y
- Any scope drift: [yes/no with details]

### Summary
- Total issues: X (Y Critical, Z High, W Medium)
- Phases with issues: [list]
- Clean phases: [list]

### Verdict: [Ready to Implement | Needs Revision | Needs Major Rework]
[One sentence explaining the verdict and recommended next step]
```

### Verdict Guidelines

- **Ready to Implement** - No critical issues, few or no high issues, all PRD criteria covered, phases are well-structured
- **Needs Revision** - Has high-priority issues or PRD alignment gaps; recommend revising affected phases before implementation
- **Needs Major Rework** - Has critical issues, missing PRD criteria, or fundamental structural problems; spec needs significant revision
