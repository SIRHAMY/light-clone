# RFC: `lc_clone` Crate

## Problem

Rust's `Clone` trait makes no guarantees about cost. A `.clone()` on a `String` deep-copies heap data, while `.clone()` on an `Arc<str>` just bumps a refcount. This makes functional/immutable programming patterns expensive by default, since idiomatic Rust structs use owned types like `String` and `Vec`.

Developers coming from F#/Haskell/Clojure expect record copying to be cheap (shallow copy + refcount bumps), but Rust's default is deep cloning.

## Goal

Provide an `LcClone` trait and derive macro that:

1. **Guarantees O(1) clone cost** — only refcount bumps and memcpys
2. **Compile-time enforcement** — fails if any field isn't cheap to clone
3. **Zero runtime overhead** — just delegates to underlying clone
4. **Ergonomic** — simple `#[derive(LcClone)]` on structs

## Core Trait

```rust
/// Marker trait for types that are O(1) to clone.
/// "Lc" = Light Clone
/// 
/// Cloning an LcClone type involves only:
/// - Atomic refcount increments (Arc)
/// - Non-atomic refcount increments (Rc)
/// - Bitwise copy (Copy types)
/// 
/// Never involves heap allocation or deep copying.
pub trait LcClone: Clone {
    fn lc(&self) -> Self;
}
```

## Built-in Implementations

### Primitives (Copy types)

```rust
impl LcClone for i8 { ... }
impl LcClone for i16 { ... }
impl LcClone for i32 { ... }
impl LcClone for i64 { ... }
impl LcClone for i128 { ... }
impl LcClone for isize { ... }
impl LcClone for u8 { ... }
impl LcClone for u16 { ... }
impl LcClone for u32 { ... }
impl LcClone for u64 { ... }
impl LcClone for u128 { ... }
impl LcClone for usize { ... }
impl LcClone for f32 { ... }
impl LcClone for f64 { ... }
impl LcClone for bool { ... }
impl LcClone for char { ... }
impl<T> LcClone for PhantomData<T> { ... }
```

### Smart Pointers

```rust
impl<T: ?Sized> LcClone for Arc<T> { ... }
impl<T: ?Sized> LcClone for Rc<T> { ... }
```

### Common Arc-wrapped Types (convenience)

```rust
// These are just Arc<T>, but explicit for clarity
impl LcClone for Arc<str> { ... }
impl<T> LcClone for Arc<[T]> { ... }
```

### Containers (when inner is LcClone)

```rust
impl<T: LcClone> LcClone for Option<T> { ... }
impl<T: LcClone, E: LcClone> LcClone for Result<T, E> { ... }
impl<T: LcClone> LcClone for Box<T> where T: Copy { ... } // Only if T is Copy!
```

### Tuples

```rust
impl LcClone for () { ... }
impl<A: LcClone> LcClone for (A,) { ... }
impl<A: LcClone, B: LcClone> LcClone for (A, B) { ... }
// ... up to reasonable tuple size
```

### Persistent Collections (optional feature)

```rust
// feature = "im"
impl<T: Clone> LcClone for im::Vector<T> { ... }
impl<K: Clone, V: Clone> LcClone for im::HashMap<K, V> { ... }
impl<K: Clone, V: Clone> LcClone for im::OrdMap<K, V> { ... }

// feature = "rpds"
impl<T: Clone> LcClone for rpds::Vector<T> { ... }
// etc.
```

## NOT Implemented For

These types are explicitly excluded — they perform deep copies:

- `String` — use `Arc<str>` instead
- `Vec<T>` — use `Arc<[T]>` or `im::Vector<T>` instead
- `HashMap<K, V>` — use `Arc<HashMap<K, V>>` or `im::HashMap<K, V>` instead
- `Box<T>` (unless T: Copy) — use `Arc<T>` instead
- `PathBuf` — use `Arc<Path>` instead

## Derive Macro

### Basic Usage

```rust
use lc_clone::LcClone;

#[derive(LcClone)]
struct Person {
    id: i64,
    name: Arc<str>,
    email: Arc<str>,
}
```

### Generated Code

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

### Compile Error Example

```rust
#[derive(LcClone)]
struct BadPerson {
    id: i64,
    name: String,  // ERROR: String does not implement LcClone
}
```

Error message:
```
error[E0277]: the trait bound `String: LcClone` is not satisfied
  --> src/lib.rs:4:5
   |
4  |     name: String,
   |     ^^^^ the trait `LcClone` is not implemented for `String`
   |
   = help: consider using `Arc<str>` or `LcStr` instead of `String`
```

## Helper Types (Optional)

For ergonomics, provide type aliases:

```rust
pub type LcStr = Arc<str>;
pub type LcList<T> = im::Vector<T>;
pub type LcMap<K, V> = im::HashMap<K, V>;
pub type LcOrdMap<K, V> = im::OrdMap<K, V>;
pub type LcSet<T> = im::HashSet<T>;
```

Usage:
```rust
use lc_clone::{LcClone, LcStr, LcList};

#[derive(LcClone)]
struct Person {
    id: i64,
    name: LcStr,
    tags: LcList<LcStr>,
}
```

## Conversions

Provide easy conversions from standard types:

```rust
pub trait IntoLcStr {
    fn into_lc(self) -> LcStr;
}

impl IntoLcStr for &str {
    fn into_lc(self) -> LcStr { Arc::from(self) }
}

impl IntoLcStr for String {
    fn into_lc(self) -> LcStr { Arc::from(self) }
}

// Usage
let name: LcStr = "hamilton".into_lc();
```

Or just rely on `Arc::from()` which already exists.

## Crate Structure

```
lc_clone/
├── Cargo.toml
├── src/
│   ├── lib.rs           # Re-exports, type aliases
│   ├── trait.rs         # LcClone trait definition
│   └── impls.rs         # Built-in implementations
└── lc_clone_derive/
    ├── Cargo.toml
    └── src/
        └── lib.rs       # Proc macro
```

### Cargo.toml

```toml
[package]
name = "lc_clone"
version = "0.1.0"

[features]
default = []
im = ["dep:im"]
rpds = ["dep:rpds"]
full = ["im", "rpds"]

[dependencies]
lc_clone_derive = { path = "./lc_clone_derive" }
im = { version = "15", optional = true }
rpds = { version = "1", optional = true }
```

## Usage Patterns

### Basic Struct

```rust
use lc_clone::{LcClone, LcStr};

#[derive(LcClone, Debug)]
struct User {
    id: u64,
    name: LcStr,
    email: LcStr,
}

let user = User { 
    id: 1, 
    name: "alice".into(), 
    email: "alice@example.com".into() 
};

// Cheap! Just refcount bumps
let user2 = user.lc();
```

### Nested Structs

```rust
#[derive(LcClone)]
struct Address {
    street: LcStr,
    city: LcStr,
}

#[derive(LcClone)]
struct Person {
    name: LcStr,
    address: Address,  // Works because Address: LcClone
}
```

### With Arc for Non-LcClone Inner Types

```rust
// If you have a type that can't be LcClone, wrap it
struct ExpensiveData { /* ... */ }

#[derive(LcClone)]
struct Container {
    id: u64,
    data: Arc<ExpensiveData>,  // Arc makes it LcClone
}
```

### With Persistent Collections

```rust
use lc_clone::{LcClone, LcStr, LcList, LcMap};

#[derive(LcClone)]
struct AppState {
    users: LcMap<u64, User>,
    events: LcList<Event>,
}

// Immutable update
let new_state = AppState {
    users: state.users.update(1, new_user),
    ..state.lc()
};
```

## Pipeline Pattern

Works great with method chaining:

```rust
impl Person {
    fn with_name(mut self, name: impl Into<LcStr>) -> Self {
        self.name = name.into();
        self
    }
    
    fn with_email(mut self, email: impl Into<LcStr>) -> Self {
        self.email = email.into();
        self
    }
}

let person = Person::default()
    .with_name("alice")
    .with_email("alice@example.com");
```

## Open Questions

1. **Should `Clone` be auto-derived?** Current design derives both `LcClone` and `Clone`. Alternative: only derive `LcClone`, require explicit `Clone`.

2. **Naming?** Settled on `LcClone` (Light Clone). Method is `.lc()`. Types are `LcStr`, `LcList`, etc.

3. **Should we provide `LcStr`, `LcList`, etc?** Or just document "use Arc<str>, im::Vector"?

4. **Rc vs Arc?** Single-threaded code could use `Rc` for 2x faster clones. Support both? Feature flag? Maybe `LcClone` for Arc-based, `LcCloneLocal` for Rc-based?

5. **Integration with serde?** `Arc<str>` and `im` types serialize fine, but worth documenting.

## Prior Art

- `triomphe::Arc` — faster Arc without weak refs
- `arcstr` — ergonomic `Arc<str>`
- `im` — persistent collections
- `rpds` — persistent collections with Rc/Arc choice
- `bytes::Bytes` — essentially `Arc<[u8]>`

## Summary

This crate would formalize a pattern that's already possible in Rust but requires discipline and boilerplate. The derive macro provides compile-time enforcement that structs remain cheap to clone, enabling functional/immutable programming with confidence.

**Quick reference:**
- Crate: `lc_clone`
- Trait: `LcClone`
- Method: `.lc()`
- Types: `LcStr`, `LcList<T>`, `LcMap<K, V>`, `LcSet<T>`, `LcOrdMap<K, V>`