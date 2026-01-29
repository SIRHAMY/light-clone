use light_clone::LightClone;
use std::sync::Arc;

#[test]
fn single_element_tuple() {
    let tuple = (42i32,);
    let cloned = tuple.light_clone();
    assert_eq!(tuple, cloned);
}

#[test]
fn two_element_tuple_with_arc() {
    let tuple = (42i32, Arc::<str>::from("hello"));
    let cloned = tuple.light_clone();
    assert_eq!(tuple.0, cloned.0);
    assert!(Arc::ptr_eq(&tuple.1, &cloned.1));
}

#[test]
fn three_element_tuple() {
    let tuple = (1i32, 2i64, 3.0f64);
    let cloned = tuple.light_clone();
    assert_eq!(tuple, cloned);
}

#[test]
fn twelve_element_tuple() {
    let tuple = (
        1i8, 2i16, 3i32, 4i64, 5i128, 6isize, 7u8, 8u16, 9u32, 10u64, 11u128, 12usize,
    );
    let cloned = tuple.light_clone();
    assert_eq!(tuple, cloned);
}

#[test]
fn nested_tuple() {
    let tuple = ((1i32, 2i32), (3i32, 4i32));
    let cloned = tuple.light_clone();
    assert_eq!(tuple, cloned);
}

#[test]
fn nested_tuple_preserves_arc_sharing() {
    let shared: Arc<str> = Arc::from("nested");
    let tuple = ((shared.light_clone(), 1i32), (2i32, shared.light_clone()));
    let cloned = tuple.light_clone();

    // Verify Arc sharing is preserved through nested tuple cloning
    assert!(Arc::ptr_eq(&tuple.0 .0, &cloned.0 .0));
    assert!(Arc::ptr_eq(&tuple.1 .1, &cloned.1 .1));
    // The two Arcs in the original also share the same allocation
    assert!(Arc::ptr_eq(&tuple.0 .0, &tuple.1 .1));
}

#[test]
fn tuple_with_arc_values() {
    let a: Arc<str> = Arc::from("first");
    let b: Arc<str> = Arc::from("second");
    let tuple = (a.light_clone(), b.light_clone());
    let cloned = tuple.light_clone();

    assert!(Arc::ptr_eq(&tuple.0, &cloned.0));
    assert!(Arc::ptr_eq(&tuple.1, &cloned.1));
}

#[test]
fn tuple_with_option() {
    let tuple = (Some(42i32), None::<i32>, Some(Arc::<str>::from("test")));
    let cloned = tuple.light_clone();
    assert_eq!(tuple.0, cloned.0);
    assert_eq!(tuple.1, cloned.1);
    if let (Some(orig), Some(clone)) = (&tuple.2, &cloned.2) {
        assert!(Arc::ptr_eq(orig, clone));
    }
}

#[test]
fn all_tuple_sizes_compile() {
    // Verify all tuple sizes from 1 to 12 compile and work
    let _t1 = (1i32,).light_clone();
    let _t2 = (1i32, 2i32).light_clone();
    let _t3 = (1i32, 2i32, 3i32).light_clone();
    let _t4 = (1i32, 2i32, 3i32, 4i32).light_clone();
    let _t5 = (1i32, 2i32, 3i32, 4i32, 5i32).light_clone();
    let _t6 = (1i32, 2i32, 3i32, 4i32, 5i32, 6i32).light_clone();
    let _t7 = (1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32).light_clone();
    let _t8 = (1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32).light_clone();
    let _t9 = (1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32).light_clone();
    let _t10 = (1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32, 10i32).light_clone();
    let _t11 = (
        1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32, 10i32, 11i32,
    )
        .light_clone();
    let _t12 = (
        1i32, 2i32, 3i32, 4i32, 5i32, 6i32, 7i32, 8i32, 9i32, 10i32, 11i32, 12i32,
    )
        .light_clone();
}
