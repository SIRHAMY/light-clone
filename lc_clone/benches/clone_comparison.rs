//! Benchmarks comparing `.lc()` vs `.clone()` for various types.
//!
//! These benchmarks demonstrate that:
//! 1. `.lc()` on Arc-based types has identical performance to `.clone()`
//! 2. LcClone structs are significantly faster to clone than deep-clone equivalents
//! 3. Performance scales with data size for deep clones but remains constant for light clones
//!
//! # Running Benchmarks
//!
//! ```sh
//! cargo bench -p lc_clone
//! cargo bench -p lc_clone --features im
//! ```
//!
//! # Expected Results Summary
//!
//! | Benchmark | Arc/LcClone | String/Clone | Notes |
//! |-----------|-------------|--------------|-------|
//! | `arc_str` | ~11ns | ~11ns | Identical - both are refcount bump |
//! | `arc_large_struct` | ~11ns | ~11ns | Size doesn't matter for Arc clone |
//! | `five_arc_fields` | ~41ns | ~41ns | 5 refcount bumps |
//! | `lc_vs_string_fields/10` | ~41ns | ~47ns | Small strings similar |
//! | `lc_vs_string_fields/10000` | ~41ns | ~230ns | LcClone 5.6x faster |
//! | `nested_structs/10` | ~26ns | ~36ns | Slight advantage |
//! | `nested_structs/10000` | ~26ns | ~509ns | LcClone 19.5x faster |
//! | `string_size/10` | ~11ns | ~9ns | Very small strings favor heap alloc |
//! | `string_size/10000` | ~11ns | ~85ns | LcClone 7.7x faster at scale |
//! | `arc_lc_vs_clone` | ~11ns | ~11ns | Proves .lc() == .clone() for Arc |
//!
//! With `im` feature enabled:
//!
//! | Benchmark | LcClone | Deep Clone | Notes |
//! |-----------|---------|------------|-------|
//! | `im_vector` | ~11ns | ~11ns | Both O(1) structural sharing |
//! | `struct_with_collection/100` | ~22ns | ~250ns | LcList 11x faster |
//! | `struct_with_collection/10000` | ~22ns | ~25µs | LcList 1000x+ faster |
//! | `struct_with_map/100` | ~22ns | ~2.5µs | LcMap 100x+ faster |
//! | `collection_sizes/10000` | ~11ns | ~25µs | Vec grows linearly |
//!
//! Key insight: LcClone performance is **constant** regardless of data size,
//! while String clone grows **linearly** with size.

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
    // Structs with collections
    // =========================================================================

    /// A user struct with a Vec of orders (deep clone).
    /// Clone cost: O(n) - must copy all elements.
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
    /// Clone cost: O(1) - structural sharing, just bumps refcounts.
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

    /// Benchmark: Struct containing Vec<String> vs LcList<LcStr>
    ///
    /// Expected:
    /// - UserWithLcList.lc(): constant ~22ns (2 refcount bumps + 1 u64 copy)
    /// - UserWithVec.clone(): grows linearly with order count
    ///   - 100 orders: ~250ns
    ///   - 10000 orders: ~25µs
    pub fn bench_struct_with_collection(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("struct_with_collection");

        for size in sizes {
            let lc_user = UserWithLcList::new(size);
            let vec_user = UserWithVec::new(size);

            group.bench_with_input(
                BenchmarkId::new("lc_list_lc", size),
                &lc_user,
                |b, user| b.iter(|| black_box(user.lc())),
            );

            group.bench_with_input(
                BenchmarkId::new("vec_clone", size),
                &vec_user,
                |b, user| b.iter(|| black_box(user.clone())),
            );
        }

        group.finish();
    }

    /// A cache struct with a HashMap (deep clone).
    /// Clone cost: O(n) - must copy all key-value pairs.
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
    /// Clone cost: O(1) - structural sharing.
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

    /// Benchmark: Struct containing HashMap<String, String> vs LcMap<LcStr, LcStr>
    ///
    /// Expected:
    /// - CacheWithLcMap.lc(): constant ~11ns (1 refcount bump)
    /// - CacheWithHashMap.clone(): grows linearly with entry count
    ///   - 100 entries: ~2.5µs
    ///   - 10000 entries: ~250µs
    pub fn bench_struct_with_map(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("struct_with_map");

        for size in sizes {
            let lc_cache = CacheWithLcMap::new(size);
            let hash_cache = CacheWithHashMap::new(size);

            group.bench_with_input(
                BenchmarkId::new("lc_map_lc", size),
                &lc_cache,
                |b, cache| b.iter(|| black_box(cache.lc())),
            );

            group.bench_with_input(
                BenchmarkId::new("hashmap_clone", size),
                &hash_cache,
                |b, cache| b.iter(|| black_box(cache.clone())),
            );
        }

        group.finish();
    }

    /// Benchmark: Collection size scaling for Vec<i32> vs LcList<i32>
    ///
    /// This benchmark isolates the collection cloning behavior without
    /// struct overhead, demonstrating how clone cost scales with size.
    ///
    /// Expected:
    /// - LcList<i32>.lc(): constant ~11ns at all sizes
    /// - Vec<i32>.clone(): grows linearly
    ///   - 10 elements: ~40ns
    ///   - 10000 elements: ~25µs
    pub fn bench_collection_sizes(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("collection_size_scaling");

        for size in sizes {
            let lc_list: LcList<i32> = (0..size as i32).collect();
            let vec: Vec<i32> = (0..size as i32).collect();

            group.bench_with_input(
                BenchmarkId::new("lc_list_lc", size),
                &lc_list,
                |b, list| b.iter(|| black_box(list.lc())),
            );

            group.bench_with_input(BenchmarkId::new("vec_clone", size), &vec, |b, v| {
                b.iter(|| black_box(v.clone()))
            });
        }

        group.finish();
    }

    /// Benchmark: Map size scaling for HashMap<i32, i32> vs LcMap<i32, i32>
    ///
    /// This benchmark isolates the map cloning behavior without struct overhead.
    ///
    /// Expected:
    /// - LcMap<i32, i32>.lc(): constant ~11ns at all sizes
    /// - HashMap<i32, i32>.clone(): grows linearly with entry count
    pub fn bench_map_sizes(c: &mut Criterion) {
        let sizes = [10, 100, 1_000, 10_000];

        let mut group = c.benchmark_group("map_size_scaling");

        for size in sizes {
            let lc_map: ImHashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();
            let hash_map: HashMap<i32, i32> = (0..size as i32).map(|i| (i, i * 2)).collect();

            group.bench_with_input(BenchmarkId::new("lc_map_lc", size), &lc_map, |b, map| {
                b.iter(|| black_box(map.lc()))
            });

            group.bench_with_input(
                BenchmarkId::new("hashmap_clone", size),
                &hash_map,
                |b, map| b.iter(|| black_box(map.clone())),
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
    im_benchmarks::bench_struct_with_collection,
    im_benchmarks::bench_struct_with_map,
    im_benchmarks::bench_collection_sizes,
    im_benchmarks::bench_map_sizes,
);

criterion_main!(benches);
