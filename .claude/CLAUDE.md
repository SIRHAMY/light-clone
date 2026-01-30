# LightClone

A Rust library providing compile-time enforcement for O(1) clone operations.

## Project Overview

LightClone is a marker trait and derive macro system that guarantees cheap cloning by only allowing types where cloning involves:
- Atomic refcount increments (`Arc`)
- Non-atomic refcount increments (`Rc`)
- Bitwise copy operations (`Copy` types)

## Workspace Structure

```
light_clone/          # Main library crate
  src/
    lib.rs            # Public API re-exports
    trait_def.rs      # LightClone trait definition
    aliases.rs        # LightStr type alias (Arc<str>)
    conversions.rs    # IntoLightStr trait
    impls/            # Trait implementations
      primitives.rs   # i8-u128, f32, f64, bool, char
      smart_pointers.rs # Arc, Rc
      tuples.rs       # Tuples up to 12 elements
      containers.rs   # Unit, PhantomData, Option, Result
      im_collections.rs    # Feature: im crate
      imbl_collections.rs  # Feature: imbl crate
      rpds_collections.rs  # Feature: rpds crate
  tests/              # Integration tests
    ui/               # Compile-fail tests (trybuild)
  benches/            # Criterion benchmarks

light_clone_derive/   # Procedural macro crate
  src/lib.rs          # Derive macro for structs and enums
```

## Development Commands

```bash
# Run all tests
cargo test --workspace --all-features

# Run benchmarks
cargo bench -p light_clone --all-features

# Format code
cargo fmt

# Lint
cargo clippy --workspace --all-features
```

## CI Validation

**Run these checks after making code changes to catch CI failures early:**

```bash
# Check compilation (fast feedback)
cargo check --workspace --all-features

# Check formatting (must pass exactly)
cargo fmt --check

# Run clippy with warnings as errors (matches CI)
cargo clippy --workspace --all-features -- -D warnings
```

When working on this codebase:
1. Run `cargo check` frequently while developing for fast feedback
2. Run `cargo fmt` before finishing any code changes
3. Run `cargo clippy -- -D warnings` before considering work complete
4. Fix all clippy warnings - CI treats warnings as errors

## Key Design Decisions

- **Compile-time safety**: Invalid types (String, Vec, etc.) are rejected at compile time
- **Zero runtime overhead**: `.light_clone()` compiles to the same code as `.clone()`
- **Ergonomic API**: `.lc()` shorthand for `.light_clone()`
- **Persistent collections**: Feature-gated support for im, imbl, and rpds crates

## Adding New Implementations

When adding LightClone implementations:
1. Only implement for types with O(1) clone (Arc, Rc, Copy, persistent collections)
2. Add tests in the appropriate test file
3. For collections, use feature gates

---

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
