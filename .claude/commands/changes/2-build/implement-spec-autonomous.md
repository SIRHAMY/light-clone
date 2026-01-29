# Implement SPEC Autonomous

> **Workflow Context:** Part of the changes workflow. Load the `changes` skill for full workflow documentation, templates, and guidance.

Run a SPEC file autonomously with memory-safe external orchestration.

## Usage

**Run this command in a separate terminal (not inside Claude):**

```bash
./.claude/commands/changes/internal/implement-spec-autonomous.sh <spec-path> [--max-retries=N]
```

## Examples

```bash
# Run a spec with default 3 retries per phase
./.claude/commands/changes/internal/implement-spec-autonomous.sh changes/001_feature/001_feature_SPEC.md

# Run with 5 retries per phase
./.claude/commands/changes/internal/implement-spec-autonomous.sh changes/001_feature/001_feature_SPEC.md --max-retries=5
```

## How It Works

The script spawns a **fresh Claude process for each phase**, preventing memory accumulation:

1. Reads your SPEC file to find the next incomplete phase
2. Runs `claude` to execute that ONE phase
3. Claude exits, releasing all memory
4. Script checks output and loops to next phase (or retries on failure)
5. Repeats until `SPEC_COMPLETE` or max retries exceeded

## Why External?

Running autonomous mode inside Claude (`/implement-spec --autonomous`) can hit OOM after ~30 minutes because subagents share the same Node.js process memory. The external script ensures each phase runs in a completely fresh process.

## Prompt

Tell the user:

You want to run a SPEC autonomously. Here's how:

**In a separate terminal, run:**
```
./.claude/commands/changes/internal/implement-spec-autonomous.sh $ARGUMENTS
```

Replace `$ARGUMENTS` with the path to your SPEC file.

**Important:** Run this in a separate terminal, not here. Running it inside Claude would defeat the memory isolation.

To monitor progress, you can watch memory in another terminal:
```
watch -n 2 'ps aux | grep claude | grep -v grep'
```

You should see the PID change between phases (confirming fresh processes).
