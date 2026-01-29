# Coding Style Guide

A functional-ish programming style guide emphasizing clarity, safety, and maintainability.

## Core Principles

### Explicit Over Implicit

Make code behavior obvious at the call site. Avoid magic, hidden state, and implicit conversions.

### Immutability by Default

Prefer `const`, `final`, `readonly`, or equivalent. Only use mutable variables when mutation is genuinely needed.

### Pure Functions

Functions should:
- Return the same output for the same input
- Have no side effects
- Not depend on external mutable state

When side effects are necessary, isolate them at the edges of your system.

### Constrain Mutations

When mutation is required:
- Limit scope to the smallest possible area
- Make it obvious where mutation occurs
- Prefer transforming data through pure functions, then mutating once at the end

## Type System

### Strong and Explicit Types

- Avoid `any`, `unknown`, or equivalent escape hatches
- Define explicit types for function parameters and return values
- Let inference work for local variables when the type is obvious

### Discriminated Unions

Use tagged unions to model states that are mutually exclusive:

```typescript
type LoadState<T> =
  | { status: 'idle' }
  | { status: 'loading' }
  | { status: 'success'; data: T }
  | { status: 'error'; error: Error }
```

### Result and Option Types

Prefer Result/Option types over exceptions for expected failure cases:

- `Option<T>` / `Maybe<T>` - for values that may not exist
- `Result<T, E>` - for operations that may fail

Reserve exceptions for truly exceptional circumstances (bugs, unrecoverable errors).

## Data and Behavior Separation

Keep data structures (what things are) separate from operations (what you do with them):

```typescript
// Data
type User = {
  id: string
  name: string
  email: string
}

// Behavior (separate module/file)
function validateEmail(user: User): Result<User, ValidationError> { ... }
function formatDisplayName(user: User): string { ... }
```

Avoid mixing data and methods in the same class when possible.

## Naming

### Be Explicit

Names should describe what something is or does without requiring context:

```typescript
// Bad
const t = 30
const data = fetchUser()

// Good
const sessionTimeoutSeconds = 30
const currentUser = fetchUser()
```

### Include Units

When a value represents a measurement, include the unit in the name:

| Unit | Abbreviation | Example |
|------|--------------|---------|
| Milliseconds | Ms | `timeoutMs`, `delayMs` |
| Seconds | Seconds | `durationSeconds`, `ttlSeconds` |
| Minutes | Minutes | `intervalMinutes` |
| Hours | Hours | `maxAgeHours` |
| Days | Days | `retentionDays` |
| Bytes | Bytes | `fileSizeBytes` |
| Kilobytes | Kb | `maxSizeKb` |
| Megabytes | Mb | `bufferSizeMb` |

Use abbreviations only when universally understood (Ms, Kb, Mb). Spell out less common units.

### Boolean Naming

Prefix booleans with `is`, `has`, `should`, `can`, or similar:

```typescript
const isEnabled = true
const hasPermission = false
const shouldRetry = attempts < maxRetries
```
