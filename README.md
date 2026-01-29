# LightClone

Compile-time enforcement for O(1) clone operations in Rust.

## Overview

LightClone provides a marker trait and derive macro that guarantees cloning is cheap. It only allows types where cloning involves:

- **Atomic refcount increments** (`Arc`)
- **Non-atomic refcount increments** (`Rc`)
- **Bitwise copies** (`Copy` types)
- **Persistent data structures** (im, imbl, rpds with structural sharing)

Types like `String`, `Vec`, or `HashMap` that perform deep copies are rejected at compile time.

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

Optional integrations with persistent collection libraries:

```toml
[dependencies]
light_clone = { version = "0.1", features = ["imbl"] }
```

| Feature | Crate | Description |
|---------|-------|-------------|
| `im` | [im](https://crates.io/crates/im) | Immutable collections |
| `imbl` | [imbl](https://crates.io/crates/imbl) | Maintained fork of im |
| `rpds` | [rpds](https://crates.io/crates/rpds) | Reactive persistent data structures |
| `full` | All of the above | Enable all collection integrations |

## Supported Types

### Primitives
All primitive types: `i8`-`i128`, `u8`-`u128`, `f32`, `f64`, `bool`, `char`, `()`

### Smart Pointers
- `Arc<T>` where `T: ?Sized`
- `Rc<T>` where `T: ?Sized`

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

## Performance

LightClone has zero runtime overhead. The `.light_clone()` method compiles to identical code as `.clone()` for the underlying types.

Benchmarks show 5-19x speedup compared to deep cloning when using persistent collections instead of standard library collections.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
