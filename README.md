# LightClone

Compile-time enforcement for O(1) clone operations in Rust.

## Overview

LightClone is for codebases that embrace immutable data structures. It provides a marker trait and derive macro that guarantees cloning is cheap by only allowing types where cloning involves:

- **Atomic refcount increments** (`Arc`)
- **Non-atomic refcount increments** (`Rc`)
- **Bitwise copies** (`Copy` types)
- **Persistent data structures** (im, imbl, rpds with structural sharing)

Types like `String`, `Vec`, or `HashMap` that perform deep copies are rejected at compile time.

[crates.io](https://crates.io/crates/light_clone) | [docs.rs](https://docs.rs/light_clone)

## Usage

```rust
use light_clone::LightClone;
use std::sync::Arc;

#[derive(LightClone)]
struct Config {
    name: Arc<str>,
    max_connections: u32,
    timeout_ms: u64,
}

let config = Config {
    name: "production".into(),
    max_connections: 100,
    timeout_ms: 5000,
};

// .light_clone() or .lc() for short
let clone = config.lc();
```

### Compile-Time Safety

Invalid types are caught at compile time:

```rust
#[derive(LightClone)]
struct Invalid {
    data: String,  // Error: String does not implement LightClone
}
```

### Ergonomic Strings

Use `LightStr` as a cheap-to-clone string type:

```rust
use light_clone::{LightStr, IntoLightStr};

let s: LightStr = "hello".into_light_str();
let clone = s.lc();  // O(1) - just increments refcount
```

## Features

Enable integrations with popular crates via feature flags:

```toml
[dependencies]
light_clone = { version = "0.3", features = ["bytes", "smol_str"] }
```

### Persistent Collections

| Feature | Crate | Types |
|---------|-------|-------|
| `im` | [im](https://crates.io/crates/im) | `Vector`, `HashMap`, `HashSet`, `OrdMap`, `OrdSet` |
| `imbl` | [imbl](https://crates.io/crates/imbl) | `Vector`, `HashMap`, `HashSet`, `OrdMap`, `OrdSet` |
| `rpds` | [rpds](https://crates.io/crates/rpds) | `Vector`, `List`, `Queue`, `Stack`, `HashTrieMap`, `HashTrieSet`, `RedBlackTreeMap`, `RedBlackTreeSet` |

### Common Types

| Feature | Crate | Types | Clone Mechanism |
|---------|-------|-------|-----------------|
| `bytes` | [bytes](https://crates.io/crates/bytes) | `Bytes` | Arc-based ref counting |
| `smol_str` | [smol_str](https://crates.io/crates/smol_str) | `SmolStr` | Inline or Arc |
| `uuid` | [uuid](https://crates.io/crates/uuid) | `Uuid` | Copy (128-bit) |
| `rust_decimal` | [rust_decimal](https://crates.io/crates/rust_decimal) | `Decimal` | Copy (128-bit) |
| `ordered-float` | [ordered-float](https://crates.io/crates/ordered-float) | `OrderedFloat<T>`, `NotNan<T>` | Copy wrapper |
| `chrono` | [chrono](https://crates.io/crates/chrono) | `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `DateTime<Tz>`, `Month`, `Weekday`, `TimeDelta`, `Utc`, `FixedOffset` | Copy |
| `time` | [time](https://crates.io/crates/time) | `Date`, `Time`, `PrimitiveDateTime`, `OffsetDateTime`, `UtcOffset`, `Duration`, `Month`, `Weekday` | Copy |

### Meta Features

| Feature | Description |
|---------|-------------|
| `full` | Enable all optional integrations |

## Supported Types

### Primitives
All primitive types: `i8`-`i128`, `u8`-`u128`, `f32`, `f64`, `bool`, `char`, `()`

### Smart Pointers
- `Arc<T>` where `T: ?Sized`
- `Rc<T>` where `T: ?Sized`
- `std::sync::Weak<T>` where `T: ?Sized`
- `std::rc::Weak<T>` where `T: ?Sized`

### Containers
- `Option<T>` where `T: LightClone`
- `Result<T, E>` where `T: LightClone, E: LightClone`
- `PhantomData<T>`
- Tuples up to 12 elements

### Enums

```rust
#[derive(LightClone)]
enum State {
    Idle,
    Loading { progress: u8 },
    Ready(Arc<Data>),
}
```

## When to Use Immutable Data Structures

LightClone enforces that your types use immutable data structures (`Arc`, `Rc`, persistent collections) which enable O(1) cloning through structural sharing. This approach shines when:

- **Clone-heavy workloads** - Sharing state across threads, event sourcing, undo/redo systems, functional pipelines with pure transforms
- **Cloning large or nested data** - A 10KB string clone copies 10KB; an `Arc<str>` clone increments a counter
- **Concurrent code** - Clone and send freely without worrying about data races or locks
- **Structural sharing matters** - Persistent collections share unchanged portions between versions

The trade-offs to consider:

- **Mutation is still faster than cloning** - LightClone enforces cloning is cheap, not free. In-place mutation avoids refcount operations entirely, so prefer mutation for hot paths
- **Memory overhead** - Arc/Rc add pointer indirection and allocation overhead. Persistent collections trade memory for structural sharing

**Where LightClone fits:** Once you've committed to immutable data structures, LightClone provides compile-time enforcement that cloning is cheap. It catches accidental `String` or `Vec` fields that would silently introduce expensive deep clones.

## Performance

LightClone has zero runtime overhead—`.light_clone()` compiles to identical code as `.clone()`.

The real performance benefit comes from using immutable data structures:

| Scenario | Immutable | Standard | Difference |
|----------|-----------|----------|------------|
| Clone 10KB string | 11 ns | 83 ns | 7x faster |
| Clone struct with 50 levels of nesting | 15 ns | 1.9 µs | 128x faster |
| Clone 10K element vector | 41 ns | 622 ns | 15x faster |
| Clone 10K element hashmap | 15 ns | 2.2 µs | 148x faster |

Mutation has trade-offs—persistent collections are slower for small, mutation-heavy workloads but catch up as data grows. See [BENCHMARKS.md](BENCHMARKS.md) for detailed comparisons.

## Minimum Supported Rust Version

Rust 1.70.0. The `rpds` feature requires Rust 1.85+ due to upstream dependencies.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Created By

[Hamilton Greene](https://hamy.xyz)
