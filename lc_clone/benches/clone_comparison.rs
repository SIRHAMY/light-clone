//! Benchmarks comparing `.lc()` vs `.clone()` for various types.
//!
//! These benchmarks demonstrate that:
//! 1. `.lc()` on Arc-based types has identical performance to `.clone()`
//! 2. LcClone structs are significantly faster to clone than deep-clone equivalents
//! 3. Performance scales with data size for deep clones but remains constant for light clones
//! 4. Clone-then-mutate patterns heavily favor persistent data structures
//!
//! # Running Benchmarks
//!
//! ```sh
//! cargo bench -p lc_clone              # Core benchmarks only
//! cargo bench -p lc_clone --all-features  # Include collection benchmarks
//! ```
//!
//! # Benchmark Organization
//!
//! Benchmarks are organized by what they test:
//!
//! - `arc_*` - Arc overhead verification
//! - `collection__*` - Raw collection clone/mutate (Vec vs LcList)
//! - `map__*` - Raw map clone/mutate (HashMap vs LcMap)
//! - `struct_with_collection__*` - Structs containing collections
//! - `struct_with_map__*` - Structs containing maps
//!
//! Each category has two variants:
//! - `__clone` - Pure clone performance
//! - `__clone_then_mutate` - Clone followed by a mutation (common functional pattern)
//!
//! # Expected Results Summary
//!
//! ## Core Benchmarks (no features required)
//!
//! | Benchmark | Arc/LcClone | String/Clone | Notes |
//! |-----------|-------------|--------------|-------|
//! | `arc_str` | ~11ns | ~11ns | Identical - both are refcount bump |
//! | `arc_large_struct` | ~11ns | ~11ns | Size doesn't matter for Arc clone |
//! | `arc_lc_vs_clone` | ~11ns | ~11ns | Proves .lc() == .clone() for Arc |
//! | `five_arc_fields` | ~41ns | ~41ns | 5 refcount bumps |
//! | `lc_vs_string_fields/10000` | ~41ns | ~230ns | LcClone 5.6x faster |
//! | `nested_structs/10000` | ~26ns | ~509ns | LcClone 19.5x faster |
//!
//! ## Collection Benchmarks (requires `im` feature)
//!
//! | Benchmark | LcClone | Std Clone | Notes |
//! |-----------|---------|-----------|-------|
//! | `collection__clone/10000` | ~11ns | ~25µs | 2000x+ faster |
//! | `collection__clone_then_mutate/10000` | ~50ns | ~25µs | 500x+ faster |
//! | `map__clone/10000` | ~11ns | ~100µs | 10000x+ faster |
//! | `map__clone_then_mutate/10000` | ~80ns | ~100µs | 1000x+ faster |
//! | `struct_with_collection__clone/10000` | ~22ns | ~3µs | 100x+ faster |
//! | `struct_with_map__clone/10000` | ~11ns | ~250µs | 20000x+ faster |
//!
//! Key insight: LcClone performance is **constant** regardless of data size,
//! while std collections grow **linearly** with size.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use lc_clone::LcClone;
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

/// An LcClone struct with 5 Arc fields.
/// Clone cost: 5 atomic refcount increments (~41ns total).
#[derive(LcClone)]
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

/// Level 1 nested struct (LcClone version)
#[derive(LcClone)]
struct NestedLevel1Lc {
    value: Arc<str>,
    count: i64,
}

/// Level 2 nested struct (LcClone version)
#[derive(LcClone)]
struct NestedLevel2Lc {
    inner: NestedLevel1Lc,
    name: Arc<str>,
}

/// Level 3 nested struct (LcClone version).
/// Clone cost: 3 Arc refcount bumps + 1 i64 copy = constant time.
#[derive(LcClone)]
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

/// Benchmark: `Arc<str>.lc()` vs `Arc<str>.clone()`
///
/// Expected: Identical performance (~11ns), as `.lc()` simply calls `.clone()` for Arc.
fn bench_arc_str(c: &mut Criterion) {
    let arc_str: Arc<str> = Arc::from("hello, world!");

    let mut group = c.benchmark_group("arc_str");

    group.bench_function("lc", |b| b.iter(|| black_box(arc_str.lc())));

    group.bench_function("clone", |b| b.iter(|| black_box(arc_str.clone())));

    group.finish();
}

/// Benchmark: `Arc<LargeStruct>.lc()` vs `Arc<LargeStruct>.clone()`
///
/// Expected: Identical performance (~11ns), proving Arc clone cost is independent
/// of the inner type's size. An 8KB struct clones just as fast as a 13-byte string.
fn bench_arc_large_struct(c: &mut Criterion) {
    let arc_large: Arc<LargeStruct> = Arc::new(LargeStruct::default());

    let mut group = c.benchmark_group("arc_large_struct");

    group.bench_function("lc", |b| b.iter(|| black_box(arc_large.lc())));

    group.bench_function("clone", |b| b.iter(|| black_box(arc_large.clone())));

    group.finish();
}

/// Benchmark: LcClone struct with 5 Arc fields
///
/// Expected: Identical performance for .lc() and .clone() (~41ns for 5 fields).
/// The derived Clone impl delegates to .lc(), so they're the same operation.
fn bench_five_arc_fields(c: &mut Criterion) {
    let five_arc = FiveArcFields::new("benchmark test string");

    let mut group = c.benchmark_group("five_arc_fields");

    group.bench_function("lc", |b| b.iter(|| black_box(five_arc.lc())));

    group.bench_function("clone", |b| b.iter(|| black_box(five_arc.clone())));

    group.finish();
}

/// Benchmark: LcClone struct vs equivalent struct with String fields
///
/// Expected: LcClone performance is constant (~41ns) regardless of string size.
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
            |b, struct_ref| b.iter(|| black_box(struct_ref.lc())),
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
            |b, struct_ref| b.iter(|| black_box(struct_ref.lc())),
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
/// - Arc<str>.lc(): constant ~11ns at all sizes
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
            b.iter(|| black_box(arc.lc()))
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
/// This is a sanity check benchmark that proves `.lc()` has zero overhead
/// compared to `.clone()` for Arc types. Both operations are identical:
/// a single atomic refcount increment.
///
/// Expected: Identical performance (~11ns for both).
fn bench_arc_lc_vs_clone(c: &mut Criterion) {
    let arc: Arc<str> = Arc::from("test string for direct comparison");

    let mut group = c.benchmark_group("arc_lc_vs_clone");

    group.bench_function("arc.lc()", |b| b.iter(|| black_box(&arc).lc()));
    group.bench_function("arc.clone()", |b| b.iter(|| black_box(&arc).clone()));

    group.finish();
}

// =============================================================================
// Feature-gated benchmarks for `im` crate
// =============================================================================

#[cfg(feature = "im")]
mod im_benchmarks {
    use super::*;
    use im::{HashMap as ImHashMap, Vector};
    use lc_clone::{LcList, LcMap, LcStr};
    use std::collections::HashMap;

    /// Benchmark: `im::Vector<i32>.lc()` vs `.clone()`
    ///
    /// Expected: Identical performance, as im::Vector uses structural sharing
    /// and clone is O(1). The `.lc()` simply delegates to `.clone()`.
    pub fn bench_im_vector(c: &mut Criterion) {
        let vector: Vector<i32> = (0..1000).collect();

        let mut group = c.benchmark_group("im_vector");

        group.bench_function("lc", |b| b.iter(|| black_box(vector.lc())));

        group.bench_function("clone", |b| b.iter(|| black_box(vector.clone())));

        group.finish();
    }

    // =========================================================================
    // Collection benchmarks (clone and clone-then-mutate)
    // =========================================================================

    /// Benchmark: Collection clone - Vec<i32> vs LcList<i32>
    ///
    /// Expected:
    /// - LcList: constant ~11ns (structural sharing)
    /// - Vec: grows linearly with size
    pub fn bench_collection_clone(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("collection__clone");

        for size in sizes {
            let lc_list: LcList<i32> = (0..size as i32).collect();
            let vec: Vec<i32> = (0..size as i32).collect();

            group.bench_with_input(
                BenchmarkId::new("lc_list", size),
                &lc_list,
                |b, list| b.iter(|| black_box(list.lc())),
            );

            group.bench_with_input(BenchmarkId::new("vec", size), &vec, |b, v| {
                b.iter(|| black_box(v.clone()))
            });
        }

        group.finish();
    }

    /// Benchmark: Collection clone-then-mutate pattern
    ///
    /// This is the key benchmark for functional programming patterns where
    /// you clone a collection and then modify the clone.
    ///
    /// Expected:
    /// - LcList: clone is O(1), push_back is O(log n) - total ~50ns
    /// - Vec: clone is O(n), push is O(1) amortized - total dominated by clone
    pub fn bench_collection_clone_then_mutate(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("collection__clone_then_mutate");

        for size in sizes {
            let lc_list: LcList<i32> = (0..size as i32).collect();
            let vec: Vec<i32> = (0..size as i32).collect();

            group.bench_with_input(
                BenchmarkId::new("lc_list", size),
                &lc_list,
                |b, list| {
                    b.iter(|| {
                        let mut cloned = black_box(list).lc();
                        cloned.push_back(999);
                        black_box(cloned)
                    })
                },
            );

            group.bench_with_input(BenchmarkId::new("vec", size), &vec, |b, v| {
                b.iter(|| {
                    let mut cloned = black_box(v).clone();
                    cloned.push(999);
                    black_box(cloned)
                })
            });
        }

        group.finish();
    }

    // =========================================================================
    // Map benchmarks (clone and clone-then-mutate)
    // =========================================================================

    /// Benchmark: Map clone - HashMap<i32, i32> vs LcMap<i32, i32>
    ///
    /// Expected:
    /// - LcMap: constant ~11ns (structural sharing)
    /// - HashMap: grows linearly with entry count
    pub fn bench_map_clone(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("map__clone");

        for size in sizes {
            let lc_map: ImHashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();
            let hash_map: HashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();

            group.bench_with_input(BenchmarkId::new("lc_map", size), &lc_map, |b, map| {
                b.iter(|| black_box(map.lc()))
            });

            group.bench_with_input(BenchmarkId::new("hashmap", size), &hash_map, |b, map| {
                b.iter(|| black_box(map.clone()))
            });
        }

        group.finish();
    }

    /// Benchmark: Map clone-then-mutate pattern
    ///
    /// Expected:
    /// - LcMap: clone is O(1), insert is O(log n) - total ~50ns
    /// - HashMap: clone is O(n), insert is O(1) amortized - total dominated by clone
    pub fn bench_map_clone_then_mutate(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("map__clone_then_mutate");

        for size in sizes {
            let lc_map: ImHashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();
            let hash_map: HashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();

            group.bench_with_input(BenchmarkId::new("lc_map", size), &lc_map, |b, map| {
                b.iter(|| {
                    let mut cloned = black_box(map).lc();
                    cloned.insert(99999, 99999);
                    black_box(cloned)
                })
            });

            group.bench_with_input(BenchmarkId::new("hashmap", size), &hash_map, |b, map| {
                b.iter(|| {
                    let mut cloned = black_box(map).clone();
                    cloned.insert(99999, 99999);
                    black_box(cloned)
                })
            });
        }

        group.finish();
    }

    // =========================================================================
    // Struct with collection benchmarks (clone and clone-then-mutate)
    // =========================================================================

    /// A user struct with a Vec of orders (deep clone).
    #[derive(Clone)]
    #[allow(dead_code)]
    struct UserWithVec {
        id: u64,
        name: String,
        orders: Vec<String>,
    }

    impl UserWithVec {
        fn new(order_count: usize) -> Self {
            Self {
                id: 12345,
                name: "Alice".to_string(),
                orders: (0..order_count)
                    .map(|i| format!("order-{i:05}"))
                    .collect(),
            }
        }
    }

    /// A user struct with an LcList of orders (light clone).
    #[derive(LcClone)]
    #[allow(dead_code)]
    struct UserWithLcList {
        id: u64,
        name: LcStr,
        orders: LcList<LcStr>,
    }

    impl UserWithLcList {
        fn new(order_count: usize) -> Self {
            Self {
                id: 12345,
                name: Arc::from("Alice"),
                orders: (0..order_count)
                    .map(|i| Arc::from(format!("order-{i:05}").as_str()))
                    .collect(),
            }
        }
    }

    /// Benchmark: Struct with collection clone
    ///
    /// Expected:
    /// - UserWithLcList: constant ~22ns (2 refcount bumps + 1 u64 copy)
    /// - UserWithVec: grows linearly with order count
    pub fn bench_struct_with_collection_clone(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("struct_with_collection__clone");

        for size in sizes {
            let lc_user = UserWithLcList::new(size);
            let vec_user = UserWithVec::new(size);

            group.bench_with_input(
                BenchmarkId::new("lc_struct", size),
                &lc_user,
                |b, user| b.iter(|| black_box(user.lc())),
            );

            group.bench_with_input(
                BenchmarkId::new("std_struct", size),
                &vec_user,
                |b, user| b.iter(|| black_box(user.clone())),
            );
        }

        group.finish();
    }

    /// Benchmark: Struct with collection clone-then-mutate
    ///
    /// This shows the real-world pattern of cloning a struct and adding to its collection.
    ///
    /// Expected:
    /// - LcStruct: clone O(1) + push O(log n) - constant-ish
    /// - StdStruct: clone O(n) + push O(1) - dominated by clone
    pub fn bench_struct_with_collection_clone_then_mutate(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("struct_with_collection__clone_then_mutate");

        for size in sizes {
            let lc_user = UserWithLcList::new(size);
            let vec_user = UserWithVec::new(size);
            let new_order: LcStr = Arc::from("order-99999");

            group.bench_with_input(
                BenchmarkId::new("lc_struct", size),
                &(&lc_user, &new_order),
                |b, (user, order)| {
                    b.iter(|| {
                        let mut cloned = black_box(*user).lc();
                        cloned.orders.push_back((*order).lc());
                        black_box(cloned)
                    })
                },
            );

            group.bench_with_input(
                BenchmarkId::new("std_struct", size),
                &vec_user,
                |b, user| {
                    b.iter(|| {
                        let mut cloned = black_box(user).clone();
                        cloned.orders.push("order-99999".to_string());
                        black_box(cloned)
                    })
                },
            );
        }

        group.finish();
    }

    // =========================================================================
    // Struct with map benchmarks (clone and clone-then-mutate)
    // =========================================================================

    /// A cache struct with a HashMap (deep clone).
    #[derive(Clone)]
    #[allow(dead_code)]
    struct CacheWithHashMap {
        entries: HashMap<String, String>,
    }

    impl CacheWithHashMap {
        fn new(entry_count: usize) -> Self {
            Self {
                entries: (0..entry_count)
                    .map(|i| (format!("key-{i:05}"), format!("value-{i:05}")))
                    .collect(),
            }
        }
    }

    /// A cache struct with an LcMap (light clone).
    #[derive(LcClone)]
    #[allow(dead_code)]
    struct CacheWithLcMap {
        entries: LcMap<LcStr, LcStr>,
    }

    impl CacheWithLcMap {
        fn new(entry_count: usize) -> Self {
            Self {
                entries: (0..entry_count)
                    .map(|i| {
                        (
                            Arc::from(format!("key-{i:05}").as_str()),
                            Arc::from(format!("value-{i:05}").as_str()),
                        )
                    })
                    .collect(),
            }
        }
    }

    /// Benchmark: Struct with map clone
    ///
    /// Expected:
    /// - CacheWithLcMap: constant ~11ns (1 refcount bump)
    /// - CacheWithHashMap: grows linearly with entry count
    pub fn bench_struct_with_map_clone(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("struct_with_map__clone");

        for size in sizes {
            let lc_cache = CacheWithLcMap::new(size);
            let hash_cache = CacheWithHashMap::new(size);

            group.bench_with_input(
                BenchmarkId::new("lc_struct", size),
                &lc_cache,
                |b, cache| b.iter(|| black_box(cache.lc())),
            );

            group.bench_with_input(
                BenchmarkId::new("std_struct", size),
                &hash_cache,
                |b, cache| b.iter(|| black_box(cache.clone())),
            );
        }

        group.finish();
    }

    /// Benchmark: Struct with map clone-then-mutate
    ///
    /// Expected:
    /// - LcStruct: clone O(1) + insert O(log n) - constant-ish
    /// - StdStruct: clone O(n) + insert O(1) - dominated by clone
    pub fn bench_struct_with_map_clone_then_mutate(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("struct_with_map__clone_then_mutate");

        for size in sizes {
            let lc_cache = CacheWithLcMap::new(size);
            let hash_cache = CacheWithHashMap::new(size);
            let new_key: LcStr = Arc::from("key-99999");
            let new_value: LcStr = Arc::from("value-99999");

            group.bench_with_input(
                BenchmarkId::new("lc_struct", size),
                &(&lc_cache, &new_key, &new_value),
                |b, (cache, key, value)| {
                    b.iter(|| {
                        let mut cloned = black_box(*cache).lc();
                        cloned.entries.insert((*key).lc(), (*value).lc());
                        black_box(cloned)
                    })
                },
            );

            group.bench_with_input(
                BenchmarkId::new("std_struct", size),
                &hash_cache,
                |b, cache| {
                    b.iter(|| {
                        let mut cloned = black_box(cache).clone();
                        cloned
                            .entries
                            .insert("key-99999".to_string(), "value-99999".to_string());
                        black_box(cloned)
                    })
                },
            );
        }

        group.finish();
    }
}

// =============================================================================
// Criterion setup - conditionally include im benchmarks
// =============================================================================

#[cfg(not(feature = "im"))]
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

#[cfg(feature = "im")]
criterion_group!(
    benches,
    bench_arc_str,
    bench_arc_large_struct,
    bench_five_arc_fields,
    bench_lc_vs_string_fields,
    bench_nested_structs,
    bench_string_sizes,
    bench_arc_lc_vs_clone,
    im_benchmarks::bench_im_vector,
    // Collection benchmarks
    im_benchmarks::bench_collection_clone,
    im_benchmarks::bench_collection_clone_then_mutate,
    // Map benchmarks
    im_benchmarks::bench_map_clone,
    im_benchmarks::bench_map_clone_then_mutate,
    // Struct with collection benchmarks
    im_benchmarks::bench_struct_with_collection_clone,
    im_benchmarks::bench_struct_with_collection_clone_then_mutate,
    // Struct with map benchmarks
    im_benchmarks::bench_struct_with_map_clone,
    im_benchmarks::bench_struct_with_map_clone_then_mutate,
);

criterion_main!(benches);
