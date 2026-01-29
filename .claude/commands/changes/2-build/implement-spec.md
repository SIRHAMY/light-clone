# Implement SPEC

> **Workflow Context:** Part of the changes workflow. Load the `changes` skill for full workflow documentation, templates, and guidance.

Execute phases of a SPEC using fresh subagents per phase. Supports both human-in-the-loop and autonomous modes.

Each phase runs in an isolated subagent context, preventing context accumulation across phases.

## Usage

```
/implement-spec <spec-path> [phase-range] [--autonomous] [--max-retries=N]
```

**Examples:**
- `/implement-spec spec.md` - Human-in-loop, all phases
- `/implement-spec spec.md 1-3` - Human-in-loop, phases 1-3
- `/implement-spec spec.md --autonomous` - Autonomous, all phases, default 3 retries
- `/implement-spec spec.md 2-4 --autonomous --max-retries=5` - Autonomous, phases 2-4, 5 retries

## Prompt

You are an orchestrator executing a SPEC.

**Arguments:** `$ARGUMENTS`

**First, parse the arguments:**
- Extract flags: `--autonomous` (boolean), `--max-retries=N` (number, default 3)
- Remaining args: first is SPEC_PATH, second (optional) is PHASE_RANGE
- Set MODE = "autonomous" if `--autonomous` present, else "human-in-loop"

**Phase range parsing:**
- `1-3` or `P1-P3` → phases 1, 2, and 3
- `2` or `P2` → just phase 2
- (no range) → all remaining incomplete phases

**Now read the SPEC file** at SPEC_PATH to understand:
1. The overall spec structure
2. Which phases exist and their numbers
3. Which phases have incomplete tasks (`- [ ]`)

**Build your execution list:**
- If a phase range is specified, use those phases
- Otherwise, find all phases with incomplete tasks

---

## Orchestration Loop

```
for each phase_number in execution_list:
    retries = 0

    loop:
        result = spawn_phase_subagent(phase_number)

        if result.status == "completed":
            log_success(phase_number)

            if MODE == "human-in-loop":
                # Human reviews changes before commit
                report_to_user(result)
                show_uncommitted_changes()  # e.g., git status, git diff summary
                ask "Review complete. Commit and continue to next phase? (commit/retry/stop)"
                user_response = wait_for_user_input()

                if user_response == "commit":
                    commit_changes(result.suggested_commit_msg)
                    break  # Move to next phase
                elif user_response == "retry":
                    git_reset_changes()
                    continue  # Retry the phase
                else:  # stop
                    output "SPEC_STOPPED: User chose to stop after phase {phase_number}"
                    STOP
            else:
                # Autonomous mode - orchestrator commits and proceeds
                commit_changes(result.suggested_commit_msg)
                break  # Move to next phase

        elif result.status in ["partial", "failed"]:
            if MODE == "autonomous":
                retries += 1
                log_retry(phase_number, retries, result.errors)

                if retries >= MAX_RETRIES:
                    output "SPEC_FAILED: Phase {phase_number} failed after {MAX_RETRIES} attempts"
                    output "Last error: {result.errors}"
                    STOP
                # else: loop continues, retry the phase

            else:  # human-in-loop
                report_failure_to_user(result)
                ask "Retry this phase, skip to next, or stop?"
                wait_for_user_input()
                # User decides: retry, skip, or stop

output "SPEC_COMPLETE" (or "PHASE_RANGE_COMPLETE" if range was specified)
```

---

## Subagent Prompt Template

Use the Task tool with `subagent_type: "general-purpose"` and this prompt:

```
Execute Phase N of the SPEC at: [SPEC_PATH]

**Your task:**
1. Read the SPEC file
2. Read the PRD file (referenced in the SPEC header) to understand the "why" behind the change
3. Find Phase N section (look for `### Phase N:` or `## Phase N:`)
4. Execute ALL tasks in that phase
5. Mark each task complete (`- [x]`) as you finish it
6. Run verification and code review (see below)
7. Do NOT commit - the orchestrator handles commits

**After completing all tasks, run verification yourself:**

1. **Run the phase's Verification checklist** - Execute each verification item (lint, tests, build, phase-specific criteria). Fix any failures before proceeding.
2. **Run `/code-review`** - Execute the full code review.
3. **Triage code review findings:**
   - **Fix now:** Critical and High severity issues, and any Medium issues that are quick wins (obvious bugs, missing error handling, security concerns). Fix these immediately.
   - **Note for later:** Low severity issues and Medium suggestions that are stylistic, nice-to-have, or would require significant refactoring beyond this phase's scope. Include these in your NOTES.
4. **Re-run `/code-review` after fixes** - Repeat until the review passes ("Ready to Merge") or the only remaining items are ones you've triaged as "fix later."
5. **Mark verification items complete** (`- [x]`) as they pass.

**When complete, return a summary in this exact format:**
```
STATUS: completed | partial | failed
TASKS_DONE: X/Y
REVIEW_VERDICT: [Ready to Merge | Needs Attention (with deferred items listed)]
ERRORS: [list any errors, or "none"]
NOTES: [brief description of what was done, plus any deferred review items]
SUGGESTED_COMMIT_MSG: [ID][PN] Type: Description
```

**Important:**
- Read the PRD first - it provides context for judgment calls during implementation
- Only work on Phase N tasks
- Do not proceed to other phases
- You own verification: run tests, lint, build, and code review yourself — do not leave these for the orchestrator
- If you encounter blocking errors, report them and set STATUS to partial or failed
- Never commit - leave that to the orchestrator
```

---

## Orchestrator Logging

**On phase success:**
```
✅ Phase N completed (X/Y tasks)
```

**On retry (autonomous mode):**
```
⚠️ Phase N failed (attempt M/MAX_RETRIES): [error summary]
🔄 Retrying Phase N...
```

**On completion:**
- `SPEC_COMPLETE` - All phases in the SPEC are done
- `PHASE_RANGE_COMPLETE` - All phases in the specified range are done
- `SPEC_FAILED: [reason]` - A phase exceeded max retries (autonomous only)
- `SPEC_STOPPED` - User chose to stop (human-in-loop only)

---

## Mode Summary

| Behavior | Human-in-loop (default) | Autonomous (`--autonomous`) |
|----------|------------------------|----------------------------|
| Commit timing | Orchestrator commits after human approves | Orchestrator commits automatically |
| Between phases | Human reviews code, then chooses: commit/retry/stop | Auto-proceed |
| On failure | Report to user, let them decide | Retry up to max-retries |
| Retries | User-controlled | Automatic (default 3) |

**Key design:** Subagents never commit. They implement and return results. The orchestrator handles all commits - this keeps subagents focused on implementation and centralizes commit logic.

**Human-in-loop flow per phase:**
1. Subagent executes tasks
2. Subagent runs verification (lint, tests, build) and fixes failures
3. Subagent runs `/code-review`, triages findings (fix critical/high now, defer low), re-runs until passing
4. Subagent returns with review verdict (no commit)
5. Orchestrator shows uncommitted changes to human
6. Human reviews the actual code changes
7. Human chooses: commit (proceed), retry (redo phase), or stop
8. Orchestrator commits with suggested message

**Autonomous flow per phase:**
1. Subagent executes tasks
2. Subagent runs verification (lint, tests, build) and fixes failures
3. Subagent runs `/code-review`, triages findings (fix critical/high now, defer low), re-runs until passing
4. Subagent returns with review verdict (no commit)
5. Orchestrator commits with suggested message
6. Proceed to next phase

**To cancel:** Just interrupt the conversation. No cleanup needed.
