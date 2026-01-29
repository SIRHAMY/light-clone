# Implement SPEC Autonomous Loop (Internal)

> **Workflow Context:** Part of the changes workflow. Load the `changes` skill for full workflow documentation, templates, and guidance.

**Do not call directly.** This is called by `.claude/commands/changes/internal/implement-spec-autonomous.sh`.

Execute the next incomplete phase of a SPEC. Designed for external orchestration.

## Usage

```
/implement-spec-autonomous-auto-loop <spec-path>
```

## Prompt

You are executing a single phase of a SPEC file.

**SPEC Path:** `$ARGUMENTS`

**Steps:**

1. Read the SPEC file at the given path
2. Read the PRD file (referenced in the SPEC header's `**PRD:**` field) to understand the "why" behind the change - this helps with judgment calls during implementation
3. Find the FIRST phase that has incomplete tasks (`- [ ]`)
4. If no incomplete phases exist:
   - Output `SPEC_COMPLETE`
   - Stop immediately

5. Execute that ONE phase:
   - Complete all tasks in the phase
   - Mark each task done (`- [x]`) as you finish

6. Run verification and code review yourself:
   a. **Run the phase's Verification checklist** — execute each item (lint, tests, build, phase-specific criteria). Fix any failures before proceeding.
   b. **Run `/code-review`** — execute the full code review.
   c. **Triage findings:**
      - **Fix now:** Critical and High severity issues, and any Medium issues that are quick wins (obvious bugs, missing error handling, security concerns). Fix these immediately.
      - **Note for later:** Low severity items and Medium suggestions that are stylistic or would require significant refactoring beyond this phase's scope. Include these in your summary.
   d. **Re-run `/code-review` after fixes** — repeat until the review passes ("Ready to Merge") or the only remaining items are ones you've triaged as "fix later."
   e. **Mark verification items complete** (`- [x]`) as they pass.

7. If verification and review pass, commit: `[ID][PN] Type: Description`
   - Add entry to Execution Log table

8. When all tasks in the phase are complete:
   - Output a brief summary (2-4 sentences): what you did, any deferred review items, and any oddities or challenges encountered
   - Output `PHASE_COMPLETE` on its own line
   - Stop (do NOT continue to next phase)

9. If phase fails or has blocking errors:
   - Output `PHASE_FAILED: [brief reason]`
   - Stop

**Output phrases (exactly one per invocation):**
- `SPEC_COMPLETE` - No more phases to execute
- `PHASE_COMPLETE` - Current phase finished successfully
- `PHASE_FAILED: [reason]` - Current phase could not be completed

**Critical:** Execute only ONE phase. The external orchestrator handles looping and retries.
