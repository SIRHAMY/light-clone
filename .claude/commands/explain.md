# Explain

Explain unfamiliar code - files, functions, flows, or concepts. Optionally include diagrams for complex relationships.

## Scope

Determine what to explain using this priority:

1. **User specifies target** - File path, function name, line range, or concept (e.g., "how does auth work")
2. **IDE selection** - If the user has code selected, explain that selection
3. **Interactive** - If unclear, ask what they want explained

Examples:
- "explain src/auth/session.ts" → file overview
- "explain the processPayment function" → function deep-dive
- "explain how the login flow works" → trace the code flow
- "explain lines 50-80 in parser.ts" → line-by-line breakdown
- "explain the caching layer" → concept explanation with component relationships

## Instructions

### 1. Identify the Explanation Mode

Based on the target, use the appropriate mode:

**File Overview** - For file paths
- High-level purpose of the file
- Key exports (functions, classes, types)
- Dependencies and what they're used for
- How this file fits into the larger system

**Function Deep-Dive** - For specific functions/methods
- What the function does (one sentence)
- Parameters and their purposes
- Return value and possible states
- Side effects (mutations, API calls, state changes)
- Edge cases and error handling

**Code Flow** - For concepts or "how does X work" questions
- Entry point(s)
- Step-by-step trace through the call chain
- Key decision points and branches
- Where data is transformed or persisted

**Line-by-Line** - For specific line ranges
- What each section does
- Why it's written this way
- Any non-obvious behavior

### 2. Analyze the Code

- Read the target code thoroughly
- Trace dependencies and related code as needed
- Identify patterns, design decisions, and gotchas
- Note any non-obvious behavior or edge cases

### 3. Generate Diagram (When Useful)

Include an ASCII or Mermaid diagram when:
- Multiple components interact (3+ modules/classes)
- There's a non-trivial flow (3+ steps)
- Boundaries or layers are important to understand
- User explicitly requests one

Diagram types:
- **Flow diagram** - Function call chains, request/response flows
- **Component diagram** - How modules/classes relate
- **Boundary diagram** - System layers, API boundaries, data flow
- **State diagram** - State machines, lifecycle flows

Prefer Mermaid for complex diagrams, ASCII for simple ones.

## Output Format

```
## [Target Name]

[One-sentence summary of what this code does]

### Purpose
[2-3 sentences on why this exists and what problem it solves]

### How It Works
[Explanation appropriate to the mode - overview, step-by-step, etc.]

[Diagram if applicable]

### Key Details
- [Important detail 1]
- [Important detail 2]
- [Gotchas or edge cases]

### Related Code
- `path/to/related.ts` - [brief description of relationship]
```

### Guidelines

- **Be concise** - Start brief, expand only if asked
- **Lead with the "what"** - Summary first, details after
- **Skip the obvious** - Don't explain basic language features
- **Highlight the non-obvious** - Call out gotchas, implicit behavior, or tricky logic
- **Use code snippets** - Show relevant snippets with annotations when helpful
- **Match the audience** - Adjust depth based on the question complexity
