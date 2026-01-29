//! Benchmarks comparing `.light_clone()` vs `.clone()` for various types.
//!
//! These benchmarks demonstrate that:
//! 1. `.light_clone()` on Arc-based types has identical performance to `.clone()`
//! 2. LightClone structs are significantly faster to clone than deep-clone equivalents
//! 3. Performance scales with data size for deep clones but remains constant for light clones
//! 4. Clone-then-mutate patterns heavily favor persistent data structures
//!
//! # Running Benchmarks
//!
//! ```sh
//! cargo bench -p light_clone              # Core benchmarks only
//! cargo bench -p light_clone --all-features  # Full comparison (im, imbl, rpds vs std)
//! ```
//!
//! # Benchmark Organization
//!
//! Benchmarks are organized by what they test:
//!
//! - `arc_*` - Arc overhead verification (always available)
//! - `collection__*` - Vector clone/mutate comparing std, im, imbl, rpds
//! - `map__*` - Map clone/mutate comparing std, im, imbl, rpds
//!
//! Each collection/map category has two variants:
//! - `__clone` - Pure clone performance
//! - `__clone_then_mutate` - Clone followed by a mutation (common functional pattern)
//!
//! # Persistent Collection Libraries
//!
//! The benchmarks compare these libraries (when their features are enabled):
//!
//! | Library | Feature | Vector Type | Map Type |
//! |---------|---------|-------------|----------|
//! | std | (always) | `Vec<T>` | `HashMap<K,V>` |
//! | im | `im` | `im::Vector<T>` | `im::HashMap<K,V>` |
//! | imbl | `imbl` | `imbl::Vector<T>` | `imbl::HashMap<K,V>` |
//! | rpds | `rpds` | `rpds::Vector<T>` | `rpds::HashTrieMap<K,V>` |
//!
//! # Expected Results Summary
//!
//! ## Core Benchmarks (no features required)
//!
//! | Benchmark | Arc/LightClone | String/Clone | Notes |
//! |-----------|-------------|--------------|-------|
//! | `arc_str` | ~11ns | ~11ns | Identical - both are refcount bump |
//! | `arc_large_struct` | ~11ns | ~11ns | Size doesn't matter for Arc clone |
//! | `arc_lc_vs_clone` | ~11ns | ~11ns | Proves .light_clone() == .clone() for Arc |
//! | `five_arc_fields` | ~41ns | ~41ns | 5 refcount bumps |
//! | `lc_vs_string_fields/10000` | ~41ns | ~230ns | LightClone 5.6x faster |
//! | `nested_structs/10000` | ~26ns | ~509ns | LightClone 19.5x faster |
//!
//! ## Collection Benchmarks (requires persistent collection features)
//!
//! | Benchmark | im/imbl/rpds | std | Notes |
//! |-----------|--------------|-----|-------|
//! | `collection__clone/10000` | ~11ns | ~25µs | 2000x+ faster |
//! | `collection__clone_then_mutate/10000` | ~50-80ns | ~25µs | 300-500x faster |
//! | `map__clone/10000` | ~11ns | ~100µs | 10000x+ faster |
//! | `map__clone_then_mutate/10000` | ~50-100ns | ~100µs | 1000x+ faster |
//!
//! Key insight: Persistent collections have **constant** clone cost regardless of size,
//! while std collections grow **linearly** with size.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use light_clone::LightClone;
use std::hint::black_box;
use std::sync::Arc;

// =============================================================================
// Test data structures
// =============================================================================

/// A large struct (8KB+) to demonstrate that Arc clone cost doesn't depend on inner size.
/// The struct's size is irrelevant - Arc clone is always just an atomic refcount bump.
#[derive(Clone)]
#[allow(dead_code)]
struct LargeStruct {
    data: [u8; 4096],
    values: [i64; 512],
}

impl Default for LargeStruct {
    fn default() -> Self {
        Self {
            data: [0u8; 4096],
            values: [0i64; 512],
        }
    }
}

/// An LightClone struct with 5 Arc fields.
/// Clone cost: 5 atomic refcount increments (~41ns total).
#[derive(LightClone)]
struct FiveArcFields {
    field1: Arc<str>,
    field2: Arc<str>,
    field3: Arc<str>,
    field4: Arc<str>,
    field5: Arc<str>,
}

impl FiveArcFields {
    fn new(s: &str) -> Self {
        let arc_s: Arc<str> = Arc::from(s);
        Self {
            field1: arc_s.clone(),
            field2: arc_s.clone(),
            field3: arc_s.clone(),
            field4: arc_s.clone(),
            field5: arc_s,
        }
    }
}

/// Equivalent struct with String fields for comparison.
/// Clone cost: 5 heap allocations + 5 memcpys of string data.
/// Cost grows linearly with string size.
#[derive(Clone)]
#[allow(dead_code)]
struct FiveStringFields {
    field1: String,
    field2: String,
    field3: String,
    field4: String,
    field5: String,
}

impl FiveStringFields {
    fn new(s: &str) -> Self {
        Self {
            field1: s.to_string(),
            field2: s.to_string(),
            field3: s.to_string(),
            field4: s.to_string(),
            field5: s.to_string(),
        }
    }
}

/// Level 1 nested struct (LightClone version)
#[derive(LightClone)]
struct NestedLevel1Lc {
    value: Arc<str>,
    count: i64,
}

/// Level 2 nested struct (LightClone version)
#[derive(LightClone)]
struct NestedLevel2Lc {
    inner: NestedLevel1Lc,
    name: Arc<str>,
}

/// Level 3 nested struct (LightClone version).
/// Clone cost: 3 Arc refcount bumps + 1 i64 copy = constant time.
#[derive(LightClone)]
struct NestedLevel3Lc {
    inner: NestedLevel2Lc,
    data: Arc<str>,
}

impl NestedLevel3Lc {
    fn new(s: &str) -> Self {
        let arc_s: Arc<str> = Arc::from(s);
        Self {
            inner: NestedLevel2Lc {
                inner: NestedLevel1Lc {
                    value: arc_s.clone(),
                    count: 42,
                },
                name: arc_s.clone(),
            },
            data: arc_s,
        }
    }
}

/// Level 1 nested struct (String version - deep clone)
#[derive(Clone)]
#[allow(dead_code)]
struct NestedLevel1String {
    value: String,
    count: i64,
}

/// Level 2 nested struct (String version - deep clone)
#[derive(Clone)]
#[allow(dead_code)]
struct NestedLevel2String {
    inner: NestedLevel1String,
    name: String,
}

/// Level 3 nested struct (String version - deep clone).
/// Clone cost: 3 heap allocations + 3 memcpys = O(n) where n is total string size.
#[derive(Clone)]
#[allow(dead_code)]
struct NestedLevel3String {
    inner: NestedLevel2String,
    data: String,
}

impl NestedLevel3String {
    fn new(s: &str) -> Self {
        Self {
            inner: NestedLevel2String {
                inner: NestedLevel1String {
                    value: s.to_string(),
                    count: 42,
                },
                name: s.to_string(),
            },
            data: s.to_string(),
        }
    }
}

// =============================================================================
// Helper functions
// =============================================================================

fn make_string(size_bytes: usize) -> String {
    "x".repeat(size_bytes)
}

// =============================================================================
// Core Benchmarks
// =============================================================================

/// Benchmark: `Arc<str>.light_clone()` vs `Arc<str>.clone()`
///
/// Expected: Identical performance (~11ns), as `.light_clone()` simply calls `.clone()` for Arc.
fn bench_arc_str(c: &mut Criterion) {
    let arc_str: Arc<str> = Arc::from("hello, world!");

    let mut group = c.benchmark_group("arc_str");

    group.bench_function("lc", |b| b.iter(|| black_box(arc_str.light_clone())));

    group.bench_function("clone", |b| b.iter(|| black_box(arc_str.clone())));

    group.finish();
}

/// Benchmark: `Arc<LargeStruct>.light_clone()` vs `Arc<LargeStruct>.clone()`
///
/// Expected: Identical performance (~11ns), proving Arc clone cost is independent
/// of the inner type's size. An 8KB struct clones just as fast as a 13-byte string.
fn bench_arc_large_struct(c: &mut Criterion) {
    let arc_large: Arc<LargeStruct> = Arc::new(LargeStruct::default());

    let mut group = c.benchmark_group("arc_large_struct");

    group.bench_function("lc", |b| b.iter(|| black_box(arc_large.light_clone())));

    group.bench_function("clone", |b| b.iter(|| black_box(arc_large.clone())));

    group.finish();
}

/// Benchmark: LightClone struct with 5 Arc fields
///
/// Expected: Identical performance for .light_clone() and .clone() (~41ns for 5 fields).
/// The derived Clone impl delegates to .light_clone(), so they're the same operation.
fn bench_five_arc_fields(c: &mut Criterion) {
    let five_arc = FiveArcFields::new("benchmark test string");

    let mut group = c.benchmark_group("five_arc_fields");

    group.bench_function("lc", |b| b.iter(|| black_box(five_arc.light_clone())));

    group.bench_function("clone", |b| b.iter(|| black_box(five_arc.clone())));

    group.finish();
}

/// Benchmark: LightClone struct vs equivalent struct with String fields
///
/// Expected: LightClone performance is constant (~41ns) regardless of string size.
/// String-based struct performance degrades linearly with size:
/// - 10 bytes: ~47ns (similar to Arc)
/// - 10KB: ~230ns (5.6x slower than Arc)
fn bench_lc_vs_string_fields(c: &mut Criterion) {
    let sizes = [10, 100, 1_000, 10_000];

    let mut group = c.benchmark_group("lc_vs_string_fields");

    for size in sizes {
        let s = make_string(size);
        let lc_struct = FiveArcFields::new(&s);
        let string_struct = FiveStringFields::new(&s);

        group.bench_with_input(
            BenchmarkId::new("arc_lc", size),
            &lc_struct,
            |b, struct_ref| b.iter(|| black_box(struct_ref.light_clone())),
        );

        group.bench_with_input(
            BenchmarkId::new("string_clone", size),
            &string_struct,
            |b, struct_ref| b.iter(|| black_box(struct_ref.clone())),
        );
    }

    group.finish();
}

/// Benchmark: nested structs (3 levels deep) with Arc vs String
///
/// Expected: Shows O(1) vs O(n) scaling most dramatically:
/// - Arc-based: constant ~26ns at all sizes
/// - String-based at 10KB: ~509ns (19.5x slower)
fn bench_nested_structs(c: &mut Criterion) {
    let sizes = [10, 100, 1_000, 10_000];

    let mut group = c.benchmark_group("nested_structs_3_levels");

    for size in sizes {
        let s = make_string(size);
        let lc_nested = NestedLevel3Lc::new(&s);
        let string_nested = NestedLevel3String::new(&s);

        group.bench_with_input(
            BenchmarkId::new("arc_nested_lc", size),
            &lc_nested,
            |b, struct_ref| b.iter(|| black_box(struct_ref.light_clone())),
        );

        group.bench_with_input(
            BenchmarkId::new("string_nested_clone", size),
            &string_nested,
            |b, struct_ref| b.iter(|| black_box(struct_ref.clone())),
        );
    }

    group.finish();
}

/// Benchmark: different string sizes for Arc<str> vs String
///
/// Expected:
/// - Arc<str>.light_clone(): constant ~11ns at all sizes
/// - String.clone(): grows from ~9ns (10 bytes) to ~85ns (10KB)
///
/// Note: Very small strings may be faster with String due to short string
/// optimization and avoiding atomic operations, but Arc wins at scale.
fn bench_string_sizes(c: &mut Criterion) {
    let sizes = [10, 100, 1_000, 10_000];

    let mut group = c.benchmark_group("string_size_comparison");

    for size in sizes {
        let s = make_string(size);
        let arc_s: Arc<str> = Arc::from(s.as_str());
        let string_s = s.clone();

        group.bench_with_input(BenchmarkId::new("arc_str_lc", size), &arc_s, |b, arc| {
            b.iter(|| black_box(arc.light_clone()))
        });

        group.bench_with_input(
            BenchmarkId::new("string_clone", size),
            &string_s,
            |b, string| b.iter(|| black_box(string.clone())),
        );
    }

    group.finish();
}

/// Benchmark: Direct comparison of `Arc::lc()` vs `Arc::clone()`
///
/// This is a sanity check benchmark that proves `.light_clone()` has zero overhead
/// compared to `.clone()` for Arc types. Both operations are identical:
/// a single atomic refcount increment.
///
/// Expected: Identical performance (~11ns for both).
fn bench_arc_lc_vs_clone(c: &mut Criterion) {
    let arc: Arc<str> = Arc::from("test string for direct comparison");

    let mut group = c.benchmark_group("arc_lc_vs_clone");

    group.bench_function("arc.light_clone()", |b| {
        b.iter(|| black_box(&arc).light_clone())
    });
    group.bench_function("arc.clone()", |b| b.iter(|| black_box(&arc).clone()));

    group.finish();
}

// =============================================================================
// Feature-gated benchmarks for persistent collections (im, imbl, rpds)
// =============================================================================

/// Benchmarks comparing persistent collection libraries (im, imbl, rpds) against std.
///
/// These benchmarks require at least one persistent collection feature to be enabled.
/// Run with `--all-features` to see all libraries compared side-by-side.
#[cfg(any(feature = "im", feature = "imbl", feature = "rpds"))]
mod persistent_benchmarks {
    use super::*;
    use std::collections::HashMap;

    // =========================================================================
    // Collection benchmarks (clone and clone-then-mutate)
    // =========================================================================

    /// Benchmark: Collection clone - comparing Vec against persistent vectors
    ///
    /// Expected:
    /// - Persistent vectors: constant ~11ns (structural sharing)
    /// - Vec: grows linearly with size
    pub fn bench_collection_clone(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("collection__clone");

        for size in sizes {
            // std::Vec (baseline)
            let vec: Vec<i32> = (0..size as i32).collect();
            group.bench_with_input(BenchmarkId::new("std_vec", size), &vec, |b, v| {
                b.iter(|| black_box(v.clone()))
            });

            // im::Vector
            #[cfg(feature = "im")]
            {
                let im_vec: im::Vector<i32> = (0..size as i32).collect();
                group.bench_with_input(BenchmarkId::new("im_vector", size), &im_vec, |b, v| {
                    b.iter(|| black_box(v.light_clone()))
                });
            }

            // imbl::Vector
            #[cfg(feature = "imbl")]
            {
                let imbl_vec: imbl::Vector<i32> = (0..size as i32).collect();
                group.bench_with_input(BenchmarkId::new("imbl_vector", size), &imbl_vec, |b, v| {
                    b.iter(|| black_box(v.light_clone()))
                });
            }

            // rpds::Vector
            #[cfg(feature = "rpds")]
            {
                let rpds_vec: rpds::Vector<i32> = (0..size as i32).collect();
                group.bench_with_input(BenchmarkId::new("rpds_vector", size), &rpds_vec, |b, v| {
                    b.iter(|| black_box(v.light_clone()))
                });
            }
        }

        group.finish();
    }

    /// Benchmark: Collection clone-then-mutate pattern
    ///
    /// This is the key benchmark for functional programming patterns where
    /// you clone a collection and then modify the clone.
    pub fn bench_collection_clone_then_mutate(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("collection__clone_then_mutate");

        for size in sizes {
            // std::Vec (baseline)
            let vec: Vec<i32> = (0..size as i32).collect();
            group.bench_with_input(BenchmarkId::new("std_vec", size), &vec, |b, v| {
                b.iter(|| {
                    let mut cloned = black_box(v).clone();
                    cloned.push(999);
                    black_box(cloned)
                })
            });

            // im::Vector
            #[cfg(feature = "im")]
            {
                let im_vec: im::Vector<i32> = (0..size as i32).collect();
                group.bench_with_input(BenchmarkId::new("im_vector", size), &im_vec, |b, v| {
                    b.iter(|| {
                        let mut cloned = black_box(v).light_clone();
                        cloned.push_back(999);
                        black_box(cloned)
                    })
                });
            }

            // imbl::Vector
            #[cfg(feature = "imbl")]
            {
                let imbl_vec: imbl::Vector<i32> = (0..size as i32).collect();
                group.bench_with_input(BenchmarkId::new("imbl_vector", size), &imbl_vec, |b, v| {
                    b.iter(|| {
                        let mut cloned = black_box(v).light_clone();
                        cloned.push_back(999);
                        black_box(cloned)
                    })
                });
            }

            // rpds::Vector (note: rpds uses push_back_mut for mutation)
            #[cfg(feature = "rpds")]
            {
                let rpds_vec: rpds::Vector<i32> = (0..size as i32).collect();
                group.bench_with_input(BenchmarkId::new("rpds_vector", size), &rpds_vec, |b, v| {
                    b.iter(|| {
                        let cloned = black_box(v).light_clone().push_back(999);
                        black_box(cloned)
                    })
                });
            }
        }

        group.finish();
    }

    // =========================================================================
    // Map benchmarks (clone and clone-then-mutate)
    // =========================================================================

    /// Benchmark: Map clone - comparing HashMap against persistent maps
    pub fn bench_map_clone(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("map__clone");

        for size in sizes {
            // std::HashMap (baseline)
            let hash_map: HashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();
            group.bench_with_input(
                BenchmarkId::new("std_hashmap", size),
                &hash_map,
                |b, map| b.iter(|| black_box(map.clone())),
            );

            // im::HashMap
            #[cfg(feature = "im")]
            {
                let im_map: im::HashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();
                group.bench_with_input(BenchmarkId::new("im_hashmap", size), &im_map, |b, map| {
                    b.iter(|| black_box(map.light_clone()))
                });
            }

            // imbl::HashMap
            #[cfg(feature = "imbl")]
            {
                let imbl_map: imbl::HashMap<i32, i32> =
                    (0..size as i32).map(|i| (i, i * 2)).collect();
                group.bench_with_input(
                    BenchmarkId::new("imbl_hashmap", size),
                    &imbl_map,
                    |b, map| b.iter(|| black_box(map.light_clone())),
                );
            }

            // rpds::HashTrieMap
            #[cfg(feature = "rpds")]
            {
                let rpds_map: rpds::HashTrieMap<i32, i32> =
                    (0..size as i32).map(|i| (i, i * 2)).collect();
                group.bench_with_input(
                    BenchmarkId::new("rpds_hashtriemap", size),
                    &rpds_map,
                    |b, map| b.iter(|| black_box(map.light_clone())),
                );
            }
        }

        group.finish();
    }

    /// Benchmark: Map clone-then-mutate pattern
    pub fn bench_map_clone_then_mutate(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("map__clone_then_mutate");

        for size in sizes {
            // std::HashMap (baseline)
            let hash_map: HashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();
            group.bench_with_input(
                BenchmarkId::new("std_hashmap", size),
                &hash_map,
                |b, map| {
                    b.iter(|| {
                        let mut cloned = black_box(map).clone();
                        cloned.insert(99999, 99999);
                        black_box(cloned)
                    })
                },
            );

            // im::HashMap
            #[cfg(feature = "im")]
            {
                let im_map: im::HashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();
                group.bench_with_input(BenchmarkId::new("im_hashmap", size), &im_map, |b, map| {
                    b.iter(|| {
                        let mut cloned = black_box(map).light_clone();
                        cloned.insert(99999, 99999);
                        black_box(cloned)
                    })
                });
            }

            // imbl::HashMap
            #[cfg(feature = "imbl")]
            {
                let imbl_map: imbl::HashMap<i32, i32> =
                    (0..size as i32).map(|i| (i, i * 2)).collect();
                group.bench_with_input(
                    BenchmarkId::new("imbl_hashmap", size),
                    &imbl_map,
                    |b, map| {
                        b.iter(|| {
                            let mut cloned = black_box(map).light_clone();
                            cloned.insert(99999, 99999);
                            black_box(cloned)
                        })
                    },
                );
            }

            // rpds::HashTrieMap (note: rpds uses insert for persistent update)
            #[cfg(feature = "rpds")]
            {
                let rpds_map: rpds::HashTrieMap<i32, i32> =
                    (0..size as i32).map(|i| (i, i * 2)).collect();
                group.bench_with_input(
                    BenchmarkId::new("rpds_hashtriemap", size),
                    &rpds_map,
                    |b, map| {
                        b.iter(|| {
                            let cloned = black_box(map).light_clone().insert(99999, 99999);
                            black_box(cloned)
                        })
                    },
                );
            }
        }

        group.finish();
    }
}

// =============================================================================
// Criterion setup - conditionally include persistent collection benchmarks
// =============================================================================

#[cfg(not(any(feature = "im", feature = "imbl", feature = "rpds")))]
criterion_group!(
    benches,
    bench_arc_str,
    bench_arc_large_struct,
    bench_five_arc_fields,
    bench_lc_vs_string_fields,
    bench_nested_structs,
    bench_string_sizes,
    bench_arc_lc_vs_clone,
);

#[cfg(any(feature = "im", feature = "imbl", feature = "rpds"))]
criterion_group!(
    benches,
    bench_arc_str,
    bench_arc_large_struct,
    bench_five_arc_fields,
    bench_lc_vs_string_fields,
    bench_nested_structs,
    bench_string_sizes,
    bench_arc_lc_vs_clone,
    // Collection benchmarks (comparing im, imbl, rpds, std)
    persistent_benchmarks::bench_collection_clone,
    persistent_benchmarks::bench_collection_clone_then_mutate,
    // Map benchmarks (comparing im, imbl, rpds, std)
    persistent_benchmarks::bench_map_clone,
    persistent_benchmarks::bench_map_clone_then_mutate,
);

criterion_main!(benches);
