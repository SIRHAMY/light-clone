# Change: Base Rust Crate - lc_clone

**Status:** Refined
**Created:** 2026-01-28
**Author:** sirhamy

## Problem Statement

Rust's `Clone` trait makes no guarantees about cost. A `.clone()` on a `String` deep-copies heap data, while `.clone()` on an `Arc<str>` just bumps a refcount. This makes functional/immutable programming patterns expensive by default, since idiomatic Rust structs use owned types like `String` and `Vec`.

Developers coming from F#/Haskell/Clojure expect record copying to be cheap (shallow copy + refcount bumps), but Rust's default is deep cloning. There's no compile-time way to enforce that a struct remains cheap to clone.

## User Stories / Personas

- **Functional Rust Developer** - Wants to use immutable data patterns without worrying about accidental expensive clones sneaking into their structs over time.

- **Library Author** - Wants to expose types that are guaranteed cheap to clone, giving users confidence about performance characteristics.

- **Team Lead** - Wants compile-time enforcement that prevents team members from accidentally adding `String` or `Vec` fields to performance-critical structs.

## Desired Outcome

A `lc_clone` crate that provides:

1. An `LcClone` trait marking types that are O(1) to clone (only refcount bumps and memcpys)
2. A `#[derive(LcClone)]` macro that fails at compile time if any field isn't cheap to clone
3. Built-in implementations for primitives, `Arc`, `Rc`, `Option<T: LcClone>`, `Result<T, E>`, tuples
4. Type aliases (`LcStr`, `LcList`, etc.) for ergonomic usage
5. Optional feature flags for persistent collection integrations (`im`, `rpds`)

When complete, a developer can add `#[derive(LcClone)]` to a struct and get:
- Compile error if any field would cause expensive cloning
- A `.lc()` method for explicit light cloning (preferred for clarity)
- Automatic `Clone` implementation that delegates to `.lc()`

**Usage guidance:** Prefer `.lc()` over `.clone()` in code to signal intent — it makes cheap cloning obvious at the call site.

## Success Criteria

### Must Have

- [ ] `LcClone` trait defined with `.lc()` method
- [ ] `#[derive(LcClone)]` proc macro for structs
- [ ] Implementations for all primitive types (i8-i128, u8-u128, f32, f64, bool, char, isize, usize)
- [ ] Implementations for `Arc<T>`, `Rc<T>`, `Arc<str>`, `Arc<[T]>` (no bound on T — Arc/Rc clone is always O(1))
- [ ] Implementations for `Option<T: LcClone>`, `Result<T: LcClone, E: Clone>` (errors just need Clone, not LcClone)
- [ ] Implementations for tuples up to 12 elements (compile error with clear message for 13+)
- [ ] Implementation for `PhantomData<T>`
- [ ] Implementation for `()` (unit type)
- [ ] Compile error with helpful message when deriving on struct with non-LcClone field
- [ ] Derive macro auto-generates `Clone` impl that delegates to `.lc()`
- [ ] Crate compiles and tests pass

### Should Have

- [ ] `LcStr` type alias for `Arc<str>`
- [ ] `IntoLcStr` trait for ergonomic conversions from `&str` and `String`
- [ ] Feature flag `im` for `im` crate persistent collections
- [ ] Feature flag `rpds` for `rpds` crate persistent collections
- [ ] `LcList<T>`, `LcMap<K, V>`, `LcSet<T>`, `LcOrdMap<K, V>` type aliases (behind `im` feature)
- [ ] Documentation with usage examples showing `LcStr`/`LcList` with comments explaining underlying types
- [ ] Quick reference migration table (`String` → `LcStr`, `Vec<T>` → `LcList<T>`, etc.)

### Nice to Have

- [ ] `Box<T: Copy>` implementation
- [ ] Serde integration documentation

### Deferred (Follow-up)

- [ ] Support for enums in derive macro (document Arc-wrapping workaround for v1)
- [ ] Generic type parameter support in derive macro (e.g., `struct Wrapper<T: LcClone>`)
- [ ] Benchmarks comparing `.lc()` vs `.clone()` on various types
- [ ] Improved error messages suggesting feature flags when `im`/`rpds` types used without feature

## Scope

### In Scope

- Core `LcClone` trait and derive macro
- Implementations for standard library types that are O(1) to clone
- Type aliases for common patterns
- Feature flags for `im` and `rpds` integration
- Basic documentation and examples

### Out of Scope

- `LcCloneLocal` variant for `Rc`-based single-threaded use (future work)
- Custom error diagnostics beyond what Rust provides
- Integration with other persistent collection crates beyond `im`/`rpds`
- Runtime clone cost tracking or profiling

## Non-Functional Requirements

- **Performance:** Zero runtime overhead — `.lc()` should compile to the same code as manual field-by-field Arc clones
- **Platform Support:** All platforms supported by Rust stable

## Constraints

- Must work on stable Rust (no nightly features required)
- Proc macro must be in separate crate (`lc_clone_derive`) per Rust requirements

## Dependencies

- **Depends On:** None (greenfield crate)
- **Blocks:** Future features that build on immutable data patterns

## Risks

- [ ] Orphan rules may prevent implementing `LcClone` for some external types — mitigated by documenting "wrap in Arc" pattern
- [ ] Proc macro error messages may not be as clear as desired — Rust's default trait bound errors are reasonable

## Open Questions

_All resolved during interview — see Interview Notes below._

- [x] ~~Should `Clone` be auto-derived by the macro?~~ **Yes** — macro auto-generates `Clone` that delegates to `.lc()`
- [x] ~~Should we provide type aliases?~~ **Yes** — provide `LcStr`, `LcList`, etc. with examples showing both alias and underlying type
- [x] ~~Tuple implementation size limit?~~ **12 elements** — compile error with clear message for larger tuples

## References

- [RFC Document](./light-clone-rfc.md)
- [im crate](https://crates.io/crates/im)
- [rpds crate](https://crates.io/crates/rpds)
- [arcstr crate](https://crates.io/crates/arcstr)
- [triomphe crate](https://crates.io/crates/triomphe)

## Interview Notes

_Interview conducted: 2026-01-28_

### Key Decisions

| Topic | Decision | Rationale |
|-------|----------|-----------|
| Generic type support | Defer | Can add later without breaking changes; keeps v1 macro simpler |
| Result error bound | `E: Clone` only | Errors are exceptional path, rarely cloned in hot loops |
| `.lc()` vs `.clone()` | Prefer `.lc()` explicitly | Explicit > implicit; signals intent at call site |
| Arc/Rc bounds | No bound on inner T | Arc clone is always O(1) regardless of inner type |
| Nested Arc types | Allow `Arc<Vec<T>>` | Consistent with "trust the Arc" — the Arc clone is cheap |
| Manual impl trust | Trust the user | Document the contract; same as Send/Sync model |
| Tuple limit | 12 elements | Match std; compile error for larger |
| Feature flags | Default off | Standard Rust pattern for optional heavy deps |
| Feature flag errors | Basic error + docs | Keep macro simple; migration guide explains features |
| Auto-derive Clone | Yes | Core value prop — cheap Clone is the point |
| Type aliases | Provide both styles | Examples show `LcStr` with comment showing `Arc<str>` |
| Enum support | Defer | Document Arc-wrapping workaround for v1 |
| Benchmarks | Defer | Want them, but not blocking v1 |
| Error messages | Keep simple | Document common migrations in quick reference table |
| Team enforcement | Derive is sufficient | Process/style guide handles the rest |

### Deferred Items

| Item | Reason | When to Revisit |
|------|--------|-----------------|
| Generic type parameters | Keeps v1 macro simple | Fast-follow when users need it |
| Enum support | Additional macro complexity | Fast-follow, document workaround |
| Benchmarks | Not blocking v1 | Before 1.0 stable release |
| Improved feature flag errors | Nice DX but not essential | If users report confusion |
