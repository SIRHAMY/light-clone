use light_clone::LightClone;
use std::sync::Arc;

// Tuple struct tests
#[derive(LightClone)]
struct TupleTwo(i32, Arc<str>);

#[test]
fn test_tuple_struct() {
    let t = TupleTwo(42, Arc::from("hello"));
    let cloned = t.light_clone();

    assert_eq!(cloned.0, 42);
    assert_eq!(&*cloned.1, "hello");
    assert!(Arc::ptr_eq(&t.1, &cloned.1));
}

#[derive(LightClone)]
struct TupleSingle(i32);

#[test]
fn test_tuple_single() {
    let t = TupleSingle(42);
    let cloned = t.light_clone();
    assert_eq!(cloned.0, 42);
}

#[derive(LightClone)]
struct TupleMany(i32, u64, f64, bool, Arc<str>);

#[test]
fn test_tuple_many_fields() {
    let t = TupleMany(1, 2, 3.0, true, Arc::from("many"));
    let cloned = t.light_clone();

    assert_eq!(cloned.0, 1);
    assert_eq!(cloned.1, 2);
    assert!((cloned.2 - 3.0).abs() < f64::EPSILON);
    assert!(cloned.3);
    assert_eq!(&*cloned.4, "many");
}

#[test]
fn test_tuple_clone_delegates_to_lc() {
    let t = TupleTwo(10, Arc::from("clone"));
    let cloned = t.clone();

    assert_eq!(cloned.0, 10);
    assert!(Arc::ptr_eq(&t.1, &cloned.1));
}

// Unit struct tests
#[derive(LightClone)]
struct UnitStruct;

#[test]
fn test_unit_struct() {
    let u = UnitStruct;
    let _cloned = u.light_clone();
    // Unit struct has no fields to verify, just ensure it compiles and runs
}

#[test]
fn test_unit_struct_clone() {
    let u = UnitStruct;
    let _cloned = u.clone();
}

#[derive(LightClone)]
struct Marker;

#[test]
fn test_multiple_unit_structs() {
    let m = Marker;
    let _cloned = m.light_clone();
}
