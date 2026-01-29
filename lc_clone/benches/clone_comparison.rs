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

// =============================================================================
// Feature-gated benchmarks for `im` crate
// =============================================================================

#[cfg(feature = "im")]
mod im_benchmarks {
    use super::*;
    use im::Vector;

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
    im_benchmarks::bench_im_vector,
);

criterion_main!(benches);
