use light_clone::LightClone;
use std::sync::Arc;

#[derive(LightClone)]
struct PrimitiveStruct {
    a: i32,
    b: u64,
    c: f64,
    d: bool,
    e: char,
}

#[test]
fn test_basic_struct_with_primitives() {
    let s = PrimitiveStruct {
        a: 42,
        b: 100,
        c: 3.14,
        d: true,
        e: 'x',
    };

    let cloned = s.light_clone();
    assert_eq!(cloned.a, 42);
    assert_eq!(cloned.b, 100);
    assert!((cloned.c - 3.14).abs() < f64::EPSILON);
    assert!(cloned.d);
    assert_eq!(cloned.e, 'x');
}

#[test]
fn test_clone_delegates_to_lc() {
    let s = PrimitiveStruct {
        a: 1,
        b: 2,
        c: 3.0,
        d: false,
        e: 'y',
    };

    let cloned = s.clone();
    assert_eq!(cloned.a, 1);
    assert_eq!(cloned.b, 2);
}

#[derive(LightClone)]
struct ArcFieldsStruct {
    name: Arc<str>,
    data: Arc<[u8]>,
    value: i32,
}

#[test]
fn test_struct_with_arc_fields() {
    let s = ArcFieldsStruct {
        name: Arc::from("hello"),
        data: Arc::from([1u8, 2, 3].as_slice()),
        value: 42,
    };

    let cloned = s.light_clone();
    assert_eq!(&*cloned.name, "hello");
    assert_eq!(&*cloned.data, &[1, 2, 3]);
    assert_eq!(cloned.value, 42);

    // Verify Arc is shared, not deep copied
    assert!(Arc::ptr_eq(&s.name, &cloned.name));
    assert!(Arc::ptr_eq(&s.data, &cloned.data));
}

#[test]
fn test_arc_strong_count_after_lc() {
    let s = ArcFieldsStruct {
        name: Arc::from("test"),
        data: Arc::from([1u8].as_slice()),
        value: 1,
    };

    assert_eq!(Arc::strong_count(&s.name), 1);
    let cloned = s.light_clone();
    assert_eq!(Arc::strong_count(&s.name), 2);
    assert_eq!(Arc::strong_count(&cloned.name), 2);
}
