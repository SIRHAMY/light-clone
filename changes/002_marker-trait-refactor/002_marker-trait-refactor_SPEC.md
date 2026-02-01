# SPEC: Marker Trait Refactor

**ID:** 002
**Status:** Draft
**Created:** 2026-02-01
**PRD:** N/A (Internal refactor based on Dupe comparison)
**Execution Mode:** human-in-the-loop
**New Agent Per Phase:** yes
**Max Review Attempts:** 3

## Context

After analyzing Facebook's [Dupe](https://github.com/facebookincubator/gazebo/blob/main/dupe/src/lib.rs) library, we identified that LightClone could be significantly simplified by adopting a marker trait approach. Currently, LightClone requires explicit `light_clone()` implementations and generates `Clone` from `LightClone`. Dupe takes the simpler approach: require `Clone`, provide a default `dupe()` impl that calls `clone()`.

Our current implementations already just call `clone()` (or equivalent) under the hood:
- Copy types: `*self` (same as clone)
- Smart pointers: `Arc::clone(self)` (same as clone)
- Persistent collections: `self.clone()` directly

The compile-time enforcement comes from the derive macro requiring all fields to be `LightClone`, not from the implementation bodies.

## Approach

Simplify LightClone to a marker trait with a default implementation:

```rust
pub trait LightClone: Clone {
    fn light_clone(&self) -> Self { self.clone() }
    fn lc(&self) -> Self { self.light_clone() }
}
```

All implementations become empty trait impls:

```rust
impl<T: ?Sized> LightClone for Arc<T> {}
impl LightClone for i32 {}
```

The derive macro changes to:
1. **Not** generate `Clone` (users must derive/impl Clone separately)
2. Generate an impl with `LightClone` bounds on all fields (compile-time enforcement)
3. No implementation body needed (uses default impl)

**Patterns to follow:**

- `light_clone/src/impls/smart_pointers.rs` — current impl structure (will be simplified)
- Facebook's Dupe macro — reference for derive macro approach

**Implementation boundaries:**

- Do not modify: `LightStr`, `IntoLightStr` (unrelated ergonomic features)
- Do not modify: feature-gated collection impls structure (just simplify bodies)
- Phases 4-6 add new std type implementations after the refactor is complete

## Open Questions

- [x] Should we keep the `.lc()` shorthand? **Yes** - it's ergonomic and doesn't add complexity

## Phase Summary

| Phase | Name | Complexity | Description |
|-------|------|------------|-------------|
| 1 | Simplify trait and impls | Low | Add default impl to trait, convert all impls to empty bodies |
| 2 | Simplify derive macro and update tests | Med | Remove Clone generation, add bounds-only impl, update all tests |
| 3 | Update documentation and release prep | Low | Update README, doc comments, CHANGELOG, version bump |
| 4 | Add Copy primitive impls | Low | Raw pointers, TypeId, network types, ThreadId, SystemTime, PhantomPinned |
| 5 | Add wrapper type impls | Low | Bound, Pin, NonNull, Poll, Cell, ManuallyDrop |
| 6 | Add function pointer impls | Low | Function pointer impls (0-12 params) via macro |

**Ordering rationale:** Trait and impls change together (Phase 1) to avoid inconsistent state. Derive macro and tests change atomically (Phase 2) to keep CI green. Documentation and release prep (Phase 3) follow the implementation. New impls (Phases 4-6) are additive and split by category for clean commits.

---

## Phases

### Phase 1: Simplify trait and impls

> Add default implementation to LightClone trait and convert all impls to empty bodies

**Complexity:** Low

**Goal:** Make `light_clone()` have a default implementation that calls `clone()`, turning LightClone into a marker trait, and simplify all existing implementations to empty bodies.

**Files:**

- `light_clone/src/trait_def.rs` — modify — add default impl to `light_clone()`
- `light_clone/src/impls/primitives.rs` — modify — empty impl bodies
- `light_clone/src/impls/smart_pointers.rs` — modify — empty impl bodies
- `light_clone/src/impls/tuples.rs` — modify — empty impl bodies
- `light_clone/src/impls/containers.rs` — modify — empty impl bodies
- `light_clone/src/impls/im_collections.rs` — modify — empty impl bodies
- `light_clone/src/impls/imbl_collections.rs` — modify — empty impl bodies
- `light_clone/src/impls/rpds_collections.rs` — modify — empty impl bodies
- `light_clone/src/impls/bytes_types.rs` — modify — empty impl bodies
- `light_clone/src/impls/chrono_types.rs` — modify — empty impl bodies
- `light_clone/src/impls/ordered_float_types.rs` — modify — empty impl bodies
- `light_clone/src/impls/rust_decimal_types.rs` — modify — empty impl bodies
- `light_clone/src/impls/smol_str_types.rs` — modify — empty impl bodies
- `light_clone/src/impls/time_types.rs` — modify — empty impl bodies
- `light_clone/src/impls/uuid.rs` — modify — empty impl bodies

**Tasks:**

- [ ] Add default implementation `fn light_clone(&self) -> Self { self.clone() }` to trait
- [ ] Keep `.lc()` shorthand with its existing default impl
- [ ] Update trait documentation to reflect marker trait nature
- [ ] Simplify `primitives.rs` macro to generate empty impls
- [ ] Simplify `smart_pointers.rs` to empty impls
- [ ] Simplify `tuples.rs` macro to generate empty impls
- [ ] Simplify `containers.rs` to empty impls
- [ ] Simplify `im_collections.rs` to empty impls
- [ ] Simplify `imbl_collections.rs` to empty impls
- [ ] Simplify `rpds_collections.rs` to empty impls
- [ ] Simplify `bytes_types.rs` to empty impls
- [ ] Simplify `chrono_types.rs` to empty impls
- [ ] Simplify `ordered_float_types.rs` to empty impls
- [ ] Simplify `rust_decimal_types.rs` to empty impls
- [ ] Simplify `smol_str_types.rs` to empty impls
- [ ] Simplify `time_types.rs` to empty impls
- [ ] Simplify `uuid.rs` to empty impls
- [ ] Remove any unused imports after simplification

**Verification:**

- [ ] Trait compiles with default impl
- [ ] All tests pass: `cargo test --workspace --all-features`
- [ ] Clippy passes: `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- [ ] Code review passes

**Commit:** `[002][P1] Clean: Simplify LightClone to marker trait with empty impls`

**Notes:**

This is backwards compatible - existing explicit impls will continue to work, they just become unnecessary. The macros for primitives and tuples become much simpler - just `impl LightClone for $t {}`.

---

### Phase 2: Simplify derive macro and update tests

> Update derive macro to generate bounds-only impl and update all tests atomically

**Complexity:** Med

**Goal:** Change the derive macro to:
1. NOT generate a `Clone` impl (user must provide separately)
2. Generate a `LightClone` impl with bounds but no body
3. Maintain compile-time enforcement via field bounds

Update all tests in the same phase to keep CI green.

**Files:**

- `light_clone_derive/src/lib.rs` — modify — simplify generated code
- `light_clone/tests/` — modify — update derive usage
- `light_clone/tests/ui/` — modify — update compile-fail tests

**Tasks:**

- [ ] Remove `Clone` impl generation from derive macro
- [ ] Change generated `LightClone` impl to have empty body (uses default)
- [ ] Ensure where clause requires `LightClone` bound on all fields
- [ ] Verify generics handling: type parameters, lifetimes, existing where clauses (follow Dupe's pattern)
- [ ] Update derive macro documentation
- [ ] Update integration tests to use `#[derive(Clone, LightClone)]`
- [ ] Update UI tests for new expected error messages (trait bound errors)
- [ ] Verify compile-fail tests still catch non-LightClone fields
- [ ] Add integration test: manual `Clone` impl + derived `LightClone`
- [ ] Add integration test: derived `Clone` + derived `LightClone`
- [ ] Add compile-fail test: `LightClone` without `Clone` (should fail)
- [ ] Add integration test: generic struct `Container<T>` requires `T: LightClone`
- [ ] Add compile-fail test: generic struct with `T = String` (should fail)

**Verification:**

- [ ] Derive works with `#[derive(Clone, LightClone)]`
- [ ] Compile error when field doesn't impl LightClone
- [ ] Generic structs correctly require `LightClone` bounds on type parameters
- [ ] All tests pass: `cargo test --workspace --all-features`
- [ ] UI tests pass with correct error messages
- [ ] Clippy passes: `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- [ ] Code review passes

**Commit:** `[002][P2] Clean: Simplify derive macro to bounds-only with updated tests`

**Notes:**

The key change is conceptual: before, `#[derive(LightClone)]` was sufficient. After, users write `#[derive(Clone, LightClone)]`. The enforcement still works because the generated impl requires `LightClone` bounds on fields.

---

### Phase 3: Update documentation and release prep

> Update README, doc comments, CHANGELOG, and bump version

**Complexity:** Low

**Goal:** Update all documentation to reflect the marker trait approach, add CHANGELOG entry for breaking change, and bump version in both crates.

**Files:**

- `README.md` — modify — update usage examples
- `light_clone/src/trait_def.rs` — modify — update doc comments
- `light_clone/src/lib.rs` — modify — update module docs if any
- `CHANGELOG.md` — modify or create — add breaking change entry
- `light_clone/Cargo.toml` — modify — bump version
- `light_clone_derive/Cargo.toml` — modify — bump version

**Tasks:**

- [x] Update README usage examples to show `#[derive(Clone, LightClone)]`
- [x] Update trait documentation to describe marker trait nature
- [x] Add note about Dupe similarity for users familiar with that crate
- [x] Update any other doc comments referencing the old pattern
- [x] Add CHANGELOG entry describing the breaking change
- [x] Bump version in `light_clone/Cargo.toml`
- [x] Bump version in `light_clone_derive/Cargo.toml` (lockstep versioning)
- [x] Update `light_clone`'s dependency on `light_clone_derive` to new version

**Verification:**

- [x] Documentation accurately reflects new API
- [x] Doc examples compile: `cargo test --doc --workspace`
- [x] CHANGELOG clearly describes the breaking change
- [x] Both crate versions match (lockstep versioning)
- [x] Code review passes

**Commit:** `[002][P3] Docs: Update documentation and bump version for marker trait API`

**Notes:**

This is a breaking change but acceptable for an alpha crate with minimal users. No migration guide needed beyond the CHANGELOG entry.

---

### Phase 4: Add Copy primitive impls

> Add LightClone implementations for Copy primitives that Dupe covers

**Complexity:** Low

**Goal:** Add implementations for Copy primitive types: raw pointers, TypeId, network types, ThreadId, SystemTime, PhantomPinned.

**Files:**

- `light_clone/src/impls/primitives.rs` — modify — add new impls

**Tasks:**

- [x] Add raw pointer impls: `*const T`, `*mut T`
- [x] Add `std::any::TypeId` impl
- [x] Add `std::marker::PhantomPinned` impl
- [x] Add network types: `Ipv4Addr`, `Ipv6Addr`, `SocketAddrV4`, `SocketAddrV6`
- [x] Add `std::thread::ThreadId` impl
- [x] Add `std::time::SystemTime` impl
- [x] Add tests for new impls

**Verification:**

- [x] All new types can call `.light_clone()` and `.lc()`
- [x] All tests pass: `cargo test --workspace --all-features`
- [x] Clippy passes: `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- [x] Code review passes

**Commit:** `[002][P4] Feature: Add LightClone for Copy primitives`

**Notes:**

With the marker trait pattern, all these impls are trivial one-liners:
```rust
impl LightClone for TypeId {}
impl<T> LightClone for *const T {}
// etc.
```

---

### Phase 5: Add wrapper type impls

> Add LightClone implementations for wrapper types

**Complexity:** Low

**Goal:** Add implementations for wrapper types: Bound, Pin, NonNull, Poll, Cell, ManuallyDrop.

**Files:**

- `light_clone/src/impls/containers.rs` — modify — add new impls

**Tasks:**

- [x] Add `std::ops::Bound<T>` impl (where T: LightClone)
- [x] Add `std::pin::Pin<T>` impl (where T: LightClone)
- [x] Add `std::ptr::NonNull<T>` impl
- [x] Add `std::task::Poll<T>` impl (where T: LightClone)
- [x] Add `std::cell::Cell<T>` impl (where T: Copy + LightClone)
- [x] Add `std::mem::ManuallyDrop<T>` impl (where T: LightClone)
- [x] Add tests for new impls

**Verification:**

- [x] All new types can call `.light_clone()` and `.lc()`
- [x] All tests pass: `cargo test --workspace --all-features`
- [x] Clippy passes: `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- [x] Code review passes

**Commit:** `[002][P5] Feature: Add LightClone for wrapper types`

**Notes:**

These wrappers delegate to their inner type's LightClone bound where appropriate.

---

### Phase 6: Add function pointer impls

> Add LightClone implementations for function pointers (0-12 params)

**Complexity:** Low

**Goal:** Add implementations for function pointers using a macro pattern similar to tuples.

**Files:**

- `light_clone/src/impls/fn_pointers.rs` — create — function pointer impls
- `light_clone/src/impls/mod.rs` — modify — add fn_pointers module

**Tasks:**

- [ ] Create `fn_pointers.rs` with macro for fn pointer impls (0-12 params)
- [ ] Add `mod fn_pointers;` to `impls/mod.rs`
- [ ] Add tests for function pointers at 0, 1, 6, and 12 params

**Verification:**

- [ ] Function pointers can call `.light_clone()` and `.lc()`
- [ ] All tests pass: `cargo test --workspace --all-features`
- [ ] Clippy passes: `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- [ ] Code review passes

**Commit:** `[002][P6] Feature: Add LightClone for function pointers`

**Notes:**

The function pointer macro follows the same pattern as tuples:
```rust
macro_rules! impl_light_clone_for_fn {
    ($($arg:ident),*) => {
        impl<Ret, $($arg),*> LightClone for fn($($arg),*) -> Ret {}
    };
}
```

---

## Final Verification

- [ ] All phases complete
- [ ] All existing tests pass
- [ ] New usage pattern works: `#[derive(Clone, LightClone)]`
- [ ] Compile-time enforcement still works (non-LightClone fields rejected)
- [ ] `.lc()` shorthand still works
- [ ] Documentation reflects new API
- [ ] CHANGELOG documents breaking change
- [ ] Version bumped in both crates
- [ ] All std types from Dupe are now supported

## Execution Log

| Phase | Status | Commit | Notes |
|-------|--------|--------|-------|
| P3 | Complete | 8b95e4d | Documentation was partially done in P1/P2; finished README/CHANGELOG updates |
| P4 | Complete | afba47b | Added Copy primitives: raw pointers, TypeId, PhantomPinned, network types, ThreadId, SystemTime |
| P5 | Complete | ae31ebb | Added wrapper types: Bound, Pin, NonNull, Poll, Cell, ManuallyDrop |

## Design Details

### Key Types

The trait definition changes from:

```rust
// Before
pub trait LightClone: Clone {
    fn light_clone(&self) -> Self;  // Required
    fn lc(&self) -> Self { self.light_clone() }
}
```

To:

```rust
// After
pub trait LightClone: Clone {
    fn light_clone(&self) -> Self { self.clone() }  // Default impl
    fn lc(&self) -> Self { self.light_clone() }
}
```

### Architecture Details

**Derive macro output changes:**

Before:
```rust
#[derive(LightClone)]
struct Foo { a: Arc<String> }

// Generated:
impl LightClone for Foo {
    fn light_clone(&self) -> Self {
        Self { a: self.a.light_clone() }
    }
}
impl Clone for Foo {
    fn clone(&self) -> Self { self.light_clone() }
}
```

After:
```rust
#[derive(Clone, LightClone)]  // User writes both
struct Foo { a: Arc<String> }

// Generated (only LightClone, not Clone):
impl LightClone for Foo
where
    Arc<String>: LightClone,  // Compile-time enforcement
{}
```

**Compile-time enforcement flow:**

1. User writes `#[derive(Clone, LightClone)]`
2. Derive macro generates `impl LightClone for T where Field1: LightClone, Field2: LightClone, ...`
3. If any field doesn't impl LightClone → compile error
4. If all fields impl LightClone → impl succeeds, uses default `clone()` delegation

### Design Rationale

**Why marker trait approach?**

1. **Simpler implementations** - Empty impl bodies instead of explicit clone logic
2. **Simpler derive macro** - No need to generate Clone, just bounds checking
3. **Same compile-time enforcement** - The bounds still prevent non-O(1) types
4. **Aligns with Dupe** - Users familiar with Dupe will recognize the pattern
5. **Conceptually clearer** - "This is a marker saying Clone is cheap" vs "This defines the clone"

**Tradeoff: requiring `#[derive(Clone, LightClone)]`**

The main downside is users must write both derives instead of just one. This is a minor ergonomic cost for significant simplification. It also makes the relationship clearer: Clone is the actual implementation, LightClone is a marker/assertion that Clone is cheap.

**Why keep `.lc()` shorthand?**

Dupe doesn't have this, but it's a nice ergonomic win for call sites with no added complexity. Keeping it maintains backwards compatibility at call sites.

---

## Retrospective

[Fill in after completion]

### What worked well?

### What was harder than expected?

### What would we do differently next time?
