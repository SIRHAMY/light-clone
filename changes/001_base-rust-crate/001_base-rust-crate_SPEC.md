# SPEC: Base Rust Crate - lc_clone

**ID:** 001
**Status:** Draft
**Created:** 2026-01-28
**PRD:** ./001_base-rust-crate_PRD.md
**Execution Mode:** human-in-the-loop
**New Agent Per Phase:** yes
**Max Review Attempts:** 3

## Context

This is a greenfield Rust crate implementing a "light clone" pattern for functional/immutable programming. The crate provides compile-time enforcement that types are O(1) to clone (only refcount bumps and memcpys).

Rust requires proc-macros to be in a separate crate, so we'll have a workspace with:
- `lc_clone` — main crate with trait, impls, and type aliases
- `lc_clone_derive` — proc-macro crate for `#[derive(LcClone)]`

## Approach

The implementation follows a bottom-up approach:

1. **Project structure first** — Set up workspace with both crates
2. **Core trait** — Define `LcClone` trait with `.lc()` method
3. **Primitive impls** — All Copy types get trivial implementations
4. **Smart pointer impls** — `Arc<T>` and `Rc<T>` (the core value of this crate)
5. **Container impls** — `Option`, `Result`, tuples, `PhantomData`, `()`
6. **Derive macro** — The key feature: auto-derive with compile-time enforcement
7. **Ergonomics** — Type aliases (`LcStr`) and conversion traits (`IntoLcStr`)
8. **Feature flags** — Optional `im` and `rpds` integrations
9. **Benchmarks** — Prove `.lc()` matches or beats manual Arc clones

**Patterns to follow:**

- Standard Rust crate conventions (lib.rs, mod.rs pattern)
- `syn`/`quote`/`proc-macro2` for derive macro (industry standard)
- Feature flags for optional dependencies

**Implementation boundaries:**

- Do not implement for enums in derive macro (deferred per PRD)
- Do not implement generic type parameter support in derive macro (deferred)
- Do not create `LcCloneLocal` variant (out of scope)

## Phase Summary

| Phase | Name | Complexity | Description |
|-------|------|------------|-------------|
| 1 | Project Setup | Low | Create workspace with lc_clone and lc_clone_derive crates |
| 2 | Core Trait & Primitives | Low | Define LcClone trait and implement for all primitives |
| 3 | Smart Pointers | Low | Implement LcClone for Arc<T>, Rc<T>, Arc<str>, Arc<[T]> |
| 4 | Containers | Med | Implement for Option, Result, tuples (up to 12), PhantomData, () |
| 5 | Derive Macro | High | Create proc-macro that generates LcClone and Clone impls |
| 6 | Type Aliases & Conversions | Low | Add LcStr, IntoLcStr trait for ergonomic usage |
| 7 | Feature Flags | Med | Add im and rpds feature flags with implementations |
| 8 | Benchmarks | Med | Add criterion benchmarks comparing .lc() vs .clone() |

**Ordering rationale:** Each phase builds on the previous. The derive macro (Phase 5) needs the trait and impls from Phases 2-4 to test against. Benchmarks come last since they need everything working.

---

## Phases

### Phase 1: Project Setup

> Create workspace with lc_clone and lc_clone_derive crates

**Complexity:** Low

**Goal:** Establish the Rust workspace structure with both crates configured correctly.

**Files:**

- `Cargo.toml` — create — workspace definition
- `lc_clone/Cargo.toml` — create — main crate with dependency on derive
- `lc_clone/src/lib.rs` — create — placeholder with re-export of derive
- `lc_clone_derive/Cargo.toml` — create — proc-macro crate
- `lc_clone_derive/src/lib.rs` — create — placeholder proc-macro

**Tasks:**

- [x] Create root `Cargo.toml` as workspace with members
- [x] Create `lc_clone/Cargo.toml` with dependency on `lc_clone_derive`
- [x] Create `lc_clone/src/lib.rs` with placeholder re-export
- [x] Create `lc_clone_derive/Cargo.toml` as proc-macro crate with syn/quote/proc-macro2
- [x] Create `lc_clone_derive/src/lib.rs` with placeholder derive macro
- [x] Verify workspace compiles with `cargo build`

**Verification:**

- [x] `cargo build` succeeds
- [x] Both crates are recognized in the workspace

**Commit:** `[001][P1] Feature: Create workspace with lc_clone and lc_clone_derive crates`

---

### Phase 2: Core Trait & Primitives

> Define LcClone trait and implement for all primitive types

**Complexity:** Low

**Goal:** Define the `LcClone` trait and implement it for all primitive Copy types.

**Files:**

- `lc_clone/src/lib.rs` — modify — re-export trait
- `lc_clone/src/trait_def.rs` — create — LcClone trait definition
- `lc_clone/src/impls/mod.rs` — create — module for implementations
- `lc_clone/src/impls/primitives.rs` — create — implementations for primitives
- `lc_clone/tests/primitives.rs` — create — tests for primitive implementations

**Tasks:**

- [x] Define `LcClone` trait in `trait_def.rs`:
  ```rust
  pub trait LcClone: Clone {
      fn lc(&self) -> Self;
  }
  ```
- [x] Create macro for implementing LcClone for Copy types
- [x] Implement for: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char
- [x] Re-export `LcClone` from `lib.rs`
- [x] Write tests verifying `.lc()` returns identical value for each primitive type
- [x] Write test verifying `LcClone: Clone` bound is satisfied

**Verification:**

- [x] All primitive types implement LcClone
- [x] Tests pass: `cargo test -p lc_clone`
- [x] Code review passes

**Commit:** `[001][P2] Feature: Add LcClone trait and primitive implementations`

---

### Phase 3: Smart Pointers

> Implement LcClone for Arc<T>, Rc<T>, Arc<str>, Arc<[T]>

**Complexity:** Low

**Goal:** Implement LcClone for reference-counted smart pointers — the core value proposition.

**Files:**

- `lc_clone/src/impls/mod.rs` — modify — add smart_pointers module
- `lc_clone/src/impls/smart_pointers.rs` — create — Arc/Rc implementations
- `lc_clone/tests/smart_pointers.rs` — create — tests for smart pointer implementations

**Tasks:**

- [x] Implement `LcClone for Arc<T>` where `T: ?Sized` (no bound on T — Arc clone is always O(1))
- [x] Implement `LcClone for Rc<T>` where `T: ?Sized`
- [x] Verify `Arc<str>` and `Arc<[T]>` work via the generic impl (they should)
- [x] Write tests for `Arc<i32>`, `Arc<str>`, `Arc<[u8]>`
- [x] Write tests for `Rc<i32>`, `Rc<str>`, `Rc<[u8]>`
- [x] Write test verifying Arc::strong_count increments correctly after `.lc()`

**Verification:**

- [x] `Arc<T>` and `Rc<T>` implement LcClone for any T
- [x] Tests pass: `cargo test -p lc_clone`
- [x] Code review passes

**Commit:** `[001][P3] Feature: Add LcClone for Arc and Rc smart pointers`

---

### Phase 4: Containers

> Implement for Option, Result, tuples (up to 12), PhantomData, ()

**Complexity:** Medium

**Goal:** Implement LcClone for container types that wrap LcClone values.

**Files:**

- `lc_clone/src/impls/mod.rs` — modify — add containers module
- `lc_clone/src/impls/containers.rs` — create — Option, Result, PhantomData, unit
- `lc_clone/src/impls/tuples.rs` — create — tuple implementations up to 12 elements
- `lc_clone/tests/containers.rs` — create — tests for container implementations
- `lc_clone/tests/tuples.rs` — create — tests for tuple implementations

**Tasks:**

- [x] Implement `LcClone for ()` (unit type)
- [x] Implement `LcClone for PhantomData<T>` (no bound on T needed)
- [x] Implement `LcClone for Option<T>` where `T: LcClone`
- [x] Implement `LcClone for Result<T, E>` where `T: LcClone, E: Clone` (per PRD: errors just need Clone)
- [x] Create macro for tuple implementations
- [x] Implement for tuples (A,), (A, B), ... up to 12 elements, all requiring LcClone bounds
- [x] Write tests for `Option<Arc<str>>`, `Option<i32>`, `None` case
- [x] Write tests for `Result<Arc<str>, String>`, `Ok` and `Err` cases
- [x] Write tests for tuples: `(i32,)`, `(i32, Arc<str>)`, 12-element tuple
- [x] Write test for `PhantomData<String>` (should work even though String isn't LcClone)

**Verification:**

- [x] All container types implement LcClone with correct bounds
- [x] Tests pass: `cargo test -p lc_clone`
- [x] Code review passes

**Commit:** `[001][P4] Feature: Add LcClone for Option, Result, tuples, PhantomData, and unit`

---

### Phase 5: Derive Macro

> Create proc-macro that generates LcClone and Clone impls

**Complexity:** High

**Goal:** Create `#[derive(LcClone)]` macro that:
1. Generates `LcClone` impl calling `.lc()` on each field
2. Generates `Clone` impl that delegates to `.lc()`
3. Fails at compile time if any field type doesn't implement `LcClone`

**Files:**

- `lc_clone_derive/Cargo.toml` — modify — ensure syn features for derive
- `lc_clone_derive/src/lib.rs` — modify — implement derive macro
- `lc_clone/src/lib.rs` — modify — re-export derive macro
- `lc_clone/tests/derive_basic.rs` — create — basic derive tests
- `lc_clone/tests/derive_nested.rs` — create — nested struct tests
- `lc_clone/tests/derive_compile_fail.rs` — create — compile-fail test setup

**Tasks:**

- [ ] Parse struct with `syn::DeriveInput`
- [ ] Extract struct fields (named and tuple structs)
- [ ] Generate `LcClone` impl:
  ```rust
  impl LcClone for StructName {
      fn lc(&self) -> Self {
          Self {
              field1: self.field1.lc(),
              field2: self.field2.lc(),
              // ...
          }
      }
  }
  ```
- [ ] Generate `Clone` impl that delegates to `.lc()`:
  ```rust
  impl Clone for StructName {
      fn clone(&self) -> Self {
          self.lc()
      }
  }
  ```
- [ ] Handle tuple structs: `struct Foo(Arc<str>, i32)`
- [ ] Handle unit structs: `struct Marker;`
- [ ] Emit clear error for enums: "LcClone derive is not yet supported for enums. Consider wrapping in Arc."
- [ ] Re-export `LcClone` derive macro from `lc_clone` crate
- [ ] Write tests: basic struct with primitives
- [ ] Write tests: struct with Arc<str> fields
- [ ] Write tests: nested struct (struct containing another LcClone struct)
- [ ] Write tests: tuple struct
- [ ] Write tests: unit struct
- [ ] Set up trybuild for compile-fail tests
- [ ] Write compile-fail test: struct with String field (should fail with trait bound error)
- [ ] Write compile-fail test: struct with Vec<T> field

**Verification:**

- [ ] `#[derive(LcClone)]` works on structs with LcClone fields
- [ ] Compile error occurs for structs with non-LcClone fields
- [ ] Tests pass: `cargo test --workspace`
- [ ] Code review passes

**Commit:** `[001][P5] Feature: Add derive macro for LcClone with Clone delegation`

---

### Phase 6: Type Aliases & Conversions

> Add LcStr, IntoLcStr trait for ergonomic usage

**Complexity:** Low

**Goal:** Provide ergonomic type aliases and conversion traits.

**Files:**

- `lc_clone/src/lib.rs` — modify — re-export type aliases and traits
- `lc_clone/src/aliases.rs` — create — type aliases
- `lc_clone/src/conversions.rs` — create — IntoLcStr trait
- `lc_clone/tests/aliases.rs` — create — tests for type aliases
- `lc_clone/tests/conversions.rs` — create — tests for conversion trait

**Tasks:**

- [ ] Create `LcStr` type alias: `pub type LcStr = Arc<str>;`
- [ ] Create `IntoLcStr` trait with `fn into_lc(self) -> LcStr`
- [ ] Implement `IntoLcStr for &str`
- [ ] Implement `IntoLcStr for String`
- [ ] Implement `IntoLcStr for &String`
- [ ] Add documentation with examples showing both alias and underlying type
- [ ] Write test: create LcStr from &str using into_lc()
- [ ] Write test: create LcStr from String using into_lc()
- [ ] Write test: use LcStr in a derived struct

**Verification:**

- [ ] Type aliases work as drop-in for Arc<str>
- [ ] IntoLcStr provides ergonomic conversion
- [ ] Tests pass: `cargo test -p lc_clone`
- [ ] Code review passes

**Commit:** `[001][P6] Feature: Add LcStr type alias and IntoLcStr conversion trait`

---

### Phase 7: Feature Flags

> Add im and rpds feature flags with implementations

**Complexity:** Medium

**Goal:** Add optional integrations with `im` and `rpds` persistent collection crates.

**Files:**

- `lc_clone/Cargo.toml` — modify — add optional dependencies and features
- `lc_clone/src/lib.rs` — modify — conditional re-exports for feature-gated types
- `lc_clone/src/impls/mod.rs` — modify — conditional modules
- `lc_clone/src/impls/im_collections.rs` — create — im crate implementations
- `lc_clone/src/impls/rpds_collections.rs` — create — rpds crate implementations
- `lc_clone/src/aliases.rs` — modify — add LcList, LcMap, etc. behind features
- `lc_clone/tests/im_integration.rs` — create — tests for im feature
- `lc_clone/tests/rpds_integration.rs` — create — tests for rpds feature

**Tasks:**

- [ ] Add `im` as optional dependency (version 15.x)
- [ ] Add `rpds` as optional dependency (version 1.x)
- [ ] Define features in Cargo.toml: `im = ["dep:im"]`, `rpds = ["dep:rpds"]`, `full = ["im", "rpds"]`
- [ ] Implement LcClone for `im::Vector<T>` where `T: Clone`
- [ ] Implement LcClone for `im::HashMap<K, V>` where `K: Clone + Eq + Hash, V: Clone`
- [ ] Implement LcClone for `im::OrdMap<K, V>` where `K: Clone + Ord, V: Clone`
- [ ] Implement LcClone for `im::HashSet<T>` where `T: Clone + Eq + Hash`
- [ ] Implement LcClone for `im::OrdSet<T>` where `T: Clone + Ord`
- [ ] Implement LcClone for rpds equivalents (Vector, HashTrieMap, etc.)
- [ ] Add type aliases behind `im` feature: `LcList<T>`, `LcMap<K, V>`, `LcSet<T>`, `LcOrdMap<K, V>`
- [ ] Write tests with `im` feature: Vector, HashMap, OrdMap operations
- [ ] Write tests with `rpds` feature: Vector, HashTrieMap operations
- [ ] Write test: struct with LcList field compiles when im feature enabled

**Verification:**

- [ ] `cargo test -p lc_clone --features im` passes
- [ ] `cargo test -p lc_clone --features rpds` passes
- [ ] `cargo test -p lc_clone --features full` passes
- [ ] Type aliases are available only when features enabled
- [ ] Code review passes

**Commit:** `[001][P7] Feature: Add im and rpds feature flags with LcClone implementations`

---

### Phase 8: Benchmarks

> Add criterion benchmarks comparing .lc() vs .clone()

**Complexity:** Medium

**Goal:** Prove `.lc()` has zero overhead compared to manual Arc clones, and is faster than deep clones.

**Files:**

- `lc_clone/Cargo.toml` — modify — add criterion dev-dependency and bench target
- `lc_clone/benches/clone_comparison.rs` — create — benchmark suite

**Tasks:**

- [ ] Add criterion as dev-dependency
- [ ] Add `[[bench]]` target in Cargo.toml
- [ ] Benchmark: `Arc<str>.lc()` vs `Arc<str>.clone()` — should be identical
- [ ] Benchmark: `Arc<LargeStruct>.lc()` vs `Arc<LargeStruct>.clone()` — should be identical
- [ ] Benchmark: LcClone struct with 5 Arc fields: `.lc()` vs manual field-by-field clone
- [ ] Benchmark: LcClone struct vs equivalent struct with String fields using `.clone()`
- [ ] Benchmark with `im` feature: `im::Vector<i32>.lc()` vs `.clone()` — should be identical
- [ ] Benchmark: nested structs (3 levels deep) with Arc vs String — show O(1) vs O(n)
- [ ] Add different sizes for String comparison: 10 bytes, 100 bytes, 1KB, 10KB
- [ ] Document benchmark results in comments

**Verification:**

- [ ] `cargo bench -p lc_clone` runs successfully
- [ ] `.lc()` on Arc-based types has same perf as `.clone()` (within noise)
- [ ] `.lc()` on LcClone structs is significantly faster than deep cloning String-based equivalents
- [ ] Benchmark report shows clear difference between light and deep cloning
- [ ] Code review passes

**Commit:** `[001][P8] Feature: Add criterion benchmarks comparing .lc() to .clone()`

---

## Final Verification

- [ ] All phases complete
- [ ] All PRD success criteria met (Must Have section)
- [ ] `cargo test --workspace --all-features` passes
- [ ] `cargo bench -p lc_clone` runs and shows expected performance
- [ ] No regressions introduced
- [ ] Documentation is present and accurate

## Execution Log

| Phase | Status | Commit | Notes |
|-------|--------|--------|-------|
| P1 | Complete | 50cfbe4 | Workspace structure created with both crates |
| P2 | Complete | cf8f0be | LcClone trait and primitive implementations added |
| P3 | Complete | fd54879 | LcClone for Arc<T> and Rc<T> with ?Sized support |
| P4 | Complete | 44ecd26 | LcClone for Option, Result, tuples (1-12), PhantomData, unit |

## Design Details

### Key Types

```rust
/// Marker trait for types that are O(1) to clone.
/// Cloning involves only:
/// - Atomic refcount increments (Arc)
/// - Non-atomic refcount increments (Rc)
/// - Bitwise copy (Copy types)
pub trait LcClone: Clone {
    fn lc(&self) -> Self;
}

/// Type alias for Arc<str> — an immutable, cheaply-cloneable string
pub type LcStr = Arc<str>;

/// Ergonomic conversion to LcStr
pub trait IntoLcStr {
    fn into_lc(self) -> LcStr;
}

// Type aliases (behind `im` feature)
pub type LcList<T> = im::Vector<T>;
pub type LcMap<K, V> = im::HashMap<K, V>;
pub type LcSet<T> = im::HashSet<T>;
pub type LcOrdMap<K, V> = im::OrdMap<K, V>;
```

### Derive Macro Generated Code

For input:
```rust
#[derive(LcClone)]
struct Person {
    id: i64,
    name: LcStr,
    email: LcStr,
}
```

Generates:
```rust
impl LcClone for Person {
    fn lc(&self) -> Self {
        Person {
            id: self.id.lc(),
            name: self.name.lc(),
            email: self.email.lc(),
        }
    }
}

impl Clone for Person {
    fn clone(&self) -> Self {
        self.lc()
    }
}
```

### Design Rationale

**Why `LcClone: Clone` bound?**
- Ensures all LcClone types are also Clone
- Allows LcClone types to work anywhere Clone is expected
- The `.lc()` method is the preferred explicit call, but Clone works too

**Why auto-derive Clone?**
- Core value proposition is "cheap Clone"
- Reduces boilerplate — user just adds `#[derive(LcClone)]`
- Clone impl simply delegates to `.lc()`, ensuring consistency

**Why no bound on T for Arc<T>?**
- `Arc::clone()` is always O(1) — just a refcount bump
- Doesn't matter what T is; the Arc itself is what's cloned
- Allows `Arc<Vec<String>>` — the Vec isn't cloned, just the Arc

**Why E: Clone instead of E: LcClone for Result<T, E>?**
- Errors are exceptional path
- Rarely cloned in hot loops
- Reduces friction for common error types like String

---

## Retrospective

[Fill in after completion]

### What worked well?

### What was harder than expected?

### What would we do differently next time?
