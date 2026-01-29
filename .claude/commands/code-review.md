# Code Review

Run a comprehensive code review using parallel agents, then synthesize findings.

## Scope

Determine what code to review using this priority:

1. **User specifies scope** - If the user provides a branch name, commit SHA, PR number/URL, or file paths, review that
2. **On a feature branch** - Review all changes on current branch vs main/master (`git diff main...HEAD`)
3. **On main/master with staged changes** - Review staged files (`git diff --staged`)
4. **On main/master, nothing staged** - Review the latest commit (`git show HEAD`)

Examples:
- "review my branch" → branch diff
- "review pr 123" or "review https://github.com/org/repo/pull/123" → fetch PR via gh
- "review commit abc123" → that specific commit
- "review src/auth.ts" → just that file's recent changes
- (no scope given, on feature branch) → automatic branch diff

## Instructions

Launch all 8 agents in parallel using a single message with multiple Task tool calls:

### Agent 1: Test Runner
```
Run relevant tests for the changed files. Report:
- Which tests were run
- Pass/fail status
- Any test failures with details
```

### Agent 2: Linter & Static Analysis
```
Run linters AND collect IDE diagnostics (using getDiagnostics) for the changed files.

Report:
- Linting tool(s) used
- Any warnings or errors found
- Auto-fixable vs manual fixes needed
- Type errors or unresolved references from IDE diagnostics
```

### Agent 3: Code Reviewer
```
First, check if CLAUDE.md or a similar project style guide exists. If so, read it to understand project conventions.

Review the code changes and provide up to 5 concrete improvements, ranked by:
- Impact (how much this improves the code)
- Effort (how hard it is to implement)

Only include genuinely important issues. If the code is clean, report fewer items or none.

Format each suggestion as:
1. [HIGH/MED/LOW Impact, HIGH/MED/LOW Effort] Title
   - What: Description of the issue
   - Why: Why this matters
   - How: Concrete suggestion to fix

Focus on non-obvious improvements - skip formatting, naming nitpicks, and things linters catch.
```

### Agent 4: Security Reviewer
```
Review the code changes for security concerns:
- Input validation and sanitization
- Injection risks (SQL, command, XSS)
- Authentication/authorization issues
- Secrets or credentials in code
- Error handling that leaks sensitive info

Also check error handling:
- Missing try/catch where needed
- Swallowed errors hiding problems
- Unhelpful error messages

Report issues with severity (Critical/High/Medium/Low) and specific file:line references.
If no issues found, report "No security concerns identified."
```

### Agent 5: Quality & Style Reviewer
```
First, check if CLAUDE.md or a similar project style guide exists. If so, read it to understand project conventions.

Review the code changes for quality and style issues:

Quality:
1. Complexity - functions too long, deeply nested, high cyclomatic complexity
2. Dead code - unused imports, unreachable code, unused variables
3. Duplication - copy-pasted logic that should be abstracted

Style Guidelines:
4. Naming conventions - does naming match project patterns and style guide?
5. File/folder organization - are files in the right place?
6. Architectural patterns - does code follow established patterns in the codebase?
7. Consistency - does new code match the style of surrounding code?
8. Project conventions - does code follow rules in the project style guide (if present)?

For each issue found, provide:
- File and location
- What the issue is
- Suggested fix

If code is clean, report "No quality or style issues identified."
```

### Agent 6: Test Quality Reviewer
```
Review test coverage and quality for the changed code:

Coverage:
- Are new code paths tested?
- Are edge cases covered?
- Are error conditions tested?

Quality:
- Do tests check meaningful behavior or just happy path?
- Are assertions specific enough?
- Would these tests catch real bugs?

Report gaps with specific suggestions. If coverage is adequate, report "Test coverage is adequate."
```

### Agent 7: Performance Reviewer
```
Review the code changes for performance concerns:
- N+1 queries or inefficient data fetching
- Blocking operations in async contexts
- Unnecessary re-renders (React) or recomputations
- Memory leaks (unclosed resources, growing collections)
- Missing pagination for large datasets
- Expensive operations in hot paths

For each concern, explain the impact and suggest a fix.
If no concerns, report "No performance concerns identified."
```

### Agent 8: Dependency & Breaking Changes Reviewer
```
Review changes for dependency and API compatibility concerns:

Dependencies (if package files changed):
- Are new dependencies justified? Check if functionality could use existing deps
- Are dependencies well-maintained? (check for recent commits, known vulnerabilities)
- Impact on bundle size for frontend dependencies

Breaking Changes (if public APIs or exports changed):
- Are any public interfaces, types, or exports modified?
- Would existing consumers of this code break?
- Is a version bump needed? (major for breaking, minor for features, patch for fixes)

Report issues with specific file references.
If no dependency changes and no public API changes, report "No dependency or compatibility concerns."
```

## After Agents Complete: Synthesize Results

Collect all agent results and produce a prioritized summary:

1. **Categorize findings** - separate issues (should fix) from suggestions (nice to have)
2. **Rank by severity** - Critical > High > Medium > Low across all agents
3. **Collapse clean results** - agents with no findings get one-line summary
4. **Give verdict** - Ready to merge / Needs attention / Needs work

### Output Format

```
## Code Review Summary

### Needs Attention (X issues)
1. [Security] Issue title - file:line
   Brief description
2. [Tests] Issue title - file:line
   Brief description

### Suggestions (X items)
1. [Quality] Title (HIGH impact, LOW effort)
   Brief description
2. [Perf] Title (MED impact, MED effort)
   Brief description
3. [Deps] Title
   Brief description

### All Clear
Tests (N passed), Linter (no issues), [other clean agents...]

### Verdict: [Ready to Merge | Needs Attention | Needs Work]
[One sentence summary of what to do next]
```

### Verdict Guidelines

- **Ready to Merge** - All tests pass, no critical/high issues, suggestions are optional
- **Needs Attention** - Has medium issues or important suggestions worth addressing
- **Needs Work** - Has critical/high issues or failing tests that must be fixed
