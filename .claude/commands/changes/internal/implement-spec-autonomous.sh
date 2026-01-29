#!/bin/bash
set -euo pipefail

# Usage: ./.claude/commands/changes/internal/implement-spec-autonomous.sh <spec-path> [--max-retries=N]

SPEC_PATH="${1:?Usage: $0 <spec-path> [--max-retries=N]}"
MAX_RETRIES=3
LOG_FILE="/tmp/implement-spec-$$.log"

# Parse optional flags
for arg in "$@"; do
    case $arg in
        --max-retries=*) MAX_RETRIES="${arg#*=}" ;;
    esac
done

timestamp() {
    date "+%Y-%m-%d %H:%M:%S"
}

echo "═══════════════════════════════════════════════════════"
echo " External Orchestrator"
echo " SPEC: $SPEC_PATH"
echo " Max retries: $MAX_RETRIES"
echo " Started: $(timestamp)"
echo "═══════════════════════════════════════════════════════"

retries=0
session_phase=1  # Counts phases completed this session, not absolute phase number

while true; do
    echo ""
    echo "─── [$(timestamp)] Starting phase (session #$session_phase) ───"

    # Run claude, tee output to log file, capture exit code
    set +e
    claude --dangerously-skip-permissions -p "/changes:internal:implement-spec-autonomous-auto-loop $SPEC_PATH" 2>&1 | tee "$LOG_FILE"
    EXIT_CODE=${PIPESTATUS[0]}
    set -e

    # Check output for known phrases
    if grep -q "SPEC_COMPLETE" "$LOG_FILE"; then
        echo ""
        echo "[$(timestamp)] ✓ SPEC_COMPLETE - All phases done!"
        rm -f "$LOG_FILE"
        exit 0
    fi

    if grep -q "PHASE_COMPLETE" "$LOG_FILE"; then
        echo ""
        echo "[$(timestamp)] ✓ Phase complete (session #$session_phase), continuing to next..."
        session_phase=$((session_phase + 1))
        retries=0  # Reset retries for new phase
        continue
    fi

    if grep -q "PHASE_FAILED\|SPEC_FAILED" "$LOG_FILE"; then
        retries=$((retries + 1))
        if [ "$retries" -ge "$MAX_RETRIES" ]; then
            echo ""
            echo "[$(timestamp)] ✗ Failed after $MAX_RETRIES attempts"
            rm -f "$LOG_FILE"
            exit 1
        fi
        echo ""
        echo "[$(timestamp)] ⚠ Phase failed, retry $retries/$MAX_RETRIES..."
        continue
    fi

    # Unknown state - treat as failure
    echo ""
    echo "[$(timestamp)] ⚠ Unknown exit state (no PHASE_COMPLETE/SPEC_COMPLETE found)"
    retries=$((retries + 1))
    if [ "$retries" -ge "$MAX_RETRIES" ]; then
        echo "[$(timestamp)] ✗ Giving up after $MAX_RETRIES attempts"
        rm -f "$LOG_FILE"
        exit 1
    fi
done
