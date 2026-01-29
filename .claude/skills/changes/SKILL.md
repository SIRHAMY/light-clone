---
name: changes
description: Structured workflow for implementing feature changes using PRDs and SPECs. Use when the user wants to make a new feature, implement a change, work on a PRD/SPEC, or asks about the changes workflow.
argument-hint: "[new|status|<change-path>]"
---

# Changes Workflow

This skill provides a structured approach for implementing feature changes using PRDs (Product Requirements Documents) and SPECs (Implementation Specifications).

## Quick Reference

| Document | Purpose | Focus |
|----------|---------|-------|
| **PRD** | Desired outcome for a change | What, not how |
| **SPEC** | Implementation strategy | How, in phases |

## Workflow Overview

```
Draft PRD → Interview → /critique-prd → Write SPEC → /critique-spec → /implement-spec → Verify
```

## Commands

| Command | Purpose |
|---------|---------|
| `/changes:0-refine:interview-prd <path>` | Structured interview to refine a PRD |
| `/changes:0-refine:critique-prd <path>` | Adversarial review of a PRD |
| `/changes:1-design:critique-spec <path>` | Adversarial review of a SPEC |
| `/changes:2-build:implement-spec <path> [range]` | Execute SPEC phases (human-in-loop) |
| `/changes:2-build:implement-spec-autonomous` | Execute SPEC phases (autonomous, run externally) |

## Common Tasks

### Create a New Change

**CRITICAL: All changes MUST follow this folder and naming structure:**

```
changes/
├── _TEMPLATE_NNN_featurename/   # Copy this for new changes
├── 001_dark-mode/
│   ├── 001_dark-mode_PRD.md
│   └── 001_dark-mode_SPEC.md
├── 002_user-auth/
│   ├── 002_user-auth_PRD.md
│   └── 002_user-auth_SPEC.md
└── 003_your-feature/            # Next change uses 003
    ├── 003_your-feature_PRD.md
    └── 003_your-feature_SPEC.md
```

When asked to create a new change:

1. **Check existing numbers** - Look in `changes/` for the highest existing number
2. **Create folder** - Use `changes/NNN_descriptive-name/` with the next sequential number (e.g., `004_new-feature/`)
3. **Create files** - Name them `NNN_descriptive-name_PRD.md` and `NNN_descriptive-name_SPEC.md`
4. **Draft PRD** - Fill in the PRD template with desired outcome and success criteria
5. **Interview** - Run `/changes:0-refine:interview-prd` to fill gaps, surface edge cases, clarify tradeoffs
6. **Write SPEC** - Break implementation into atomic phases after PRD is refined

### Check Status

When asked about status:
- List all change directories in `changes/`
- Check PRD status (Proposed, Approved, Implemented, Rejected)
- Check SPEC status (Draft, In Progress, Complete, Abandoned)
- Identify current phase if in progress

### Implement a Change

When asked to implement:
1. Read the SPEC to understand current state
2. Spawn a new agent for each phase using the Task tool
3. Each phase agent marks tasks complete as it works
4. Run code review at the end of each phase
5. Wait for phase completion before proceeding

## Key Principles

1. **PRD before SPEC** - Define success criteria before implementation approach
2. **Interview to refine** - Always offer to interview after drafting a PRD
3. **Atomic phases** - Each phase leaves codebase in a good, testable state
4. **Tests within phases** - Include tests in the phase where logic is added
5. **New agent per phase** - Fresh context for each phase (use Task tool)

## AI Instructions

When working with changes, you MUST follow these rules:

### Folder Naming (Critical)

- **NEVER** create change folders without the `NNN_` prefix
- **NEVER** create PRD/SPEC files without the `NNN_` prefix matching the folder
- **ALWAYS** check `changes/` directory first to find the next available number
- **ALWAYS** use zero-padded three-digit numbers (001, 002, ..., 010, 011, ...)

**Valid examples:**
- `changes/001_dark-mode/001_dark-mode_PRD.md`
- `changes/042_api-refactor/042_api-refactor_SPEC.md`

**Invalid examples:**
- `changes/dark-mode/` (missing number)
- `changes/1_dark-mode/` (not zero-padded)
- `changes/001_dark-mode/dark-mode_PRD.md` (file missing number prefix)

### Before Creating a Change

1. List contents of `changes/` directory
2. Find the highest numbered folder
3. Increment by 1 for the new change
4. Create folder and files with matching numbers

## Full Documentation

For complete details, read these supporting files in this skill directory:
- [workflow-guide.md](workflow-guide.md) - Full workflow documentation with AI instructions
- [templates/prd-template.md](templates/prd-template.md) - Template for new PRDs
- [templates/spec-template.md](templates/spec-template.md) - Template for new SPECs

## Usage Examples

**User:** "I want to add dark mode to the app"
**Action:** Create new change folder, draft PRD, offer interview

**User:** "What's the status of our changes?"
**Action:** List changes, show PRD/SPEC status for each

**User:** "Continue implementing the auth feature"
**Action:** Read SPEC, find current phase, spawn agent to execute

**User:** "Review my PRD"
**Action:** Read PRD, identify gaps, offer to interview or suggest `/changes:0-refine:critique-prd`
