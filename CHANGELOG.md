# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-01-29

### Added

- `LightClone` trait for compile-time O(1) clone enforcement
- `#[derive(LightClone)]` macro for structs and enums
- `.light_clone()` method and `.lc()` shorthand
- `LightStr` type alias (`Arc<str>`) for lightweight string handling
- `IntoLightStr` trait for ergonomic conversions

#### Implementations

- Primitives: `i8`-`i128`, `u8`-`u128`, `f32`, `f64`, `bool`, `char`, `usize`, `isize`
- Smart pointers: `Arc<T>`, `Rc<T>`, `Arc<str>`, `Rc<str>`, `Arc<[T]>`, `Rc<[T]>`
- Containers: `()`, `PhantomData<T>`, `Option<T>`, `Result<T, E>`
- Tuples: Up to 12 elements
- References: `&T` where `T: LightClone`

#### Feature-gated Collections

- `im` feature: `im::Vector`, `im::HashMap`, `im::HashSet`, `im::OrdMap`, `im::OrdSet`
- `imbl` feature: `imbl::Vector`, `imbl::HashMap`, `imbl::HashSet`, `imbl::OrdMap`, `imbl::OrdSet`
- `rpds` feature: `rpds::List`, `rpds::Vector`, `rpds::HashTrieMap`, `rpds::HashTrieSet`, `rpds::RedBlackTreeMap`, `rpds::RedBlackTreeSet`
- `full` feature: Enables all collection features

[Unreleased]: https://github.com/SIRHAMY/light-clone/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/SIRHAMY/light-clone/releases/tag/v0.1.0
