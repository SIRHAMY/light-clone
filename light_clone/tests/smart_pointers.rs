use light_clone::LightClone;
use std::rc::Rc;
use std::sync::Arc;

#[test]
fn arc_i32_lc_returns_equal_value() {
    let original = Arc::new(42i32);
    let cloned = original.light_clone();
    assert_eq!(*original, *cloned);
}

#[test]
fn arc_str_lc_returns_equal_value() {
    let original: Arc<str> = Arc::from("hello world");
    let cloned = original.light_clone();
    assert_eq!(&*original, &*cloned);
}

#[test]
fn arc_slice_lc_returns_equal_value() {
    let original: Arc<[u8]> = Arc::from(vec![1u8, 2, 3, 4, 5]);
    let cloned = original.light_clone();
    assert_eq!(&*original, &*cloned);
}

#[test]
fn arc_strong_count_increments_after_lc() {
    let original = Arc::new(42i32);
    assert_eq!(Arc::strong_count(&original), 1);

    let cloned = original.light_clone();
    assert_eq!(Arc::strong_count(&original), 2);
    assert_eq!(Arc::strong_count(&cloned), 2);

    drop(cloned);
    assert_eq!(Arc::strong_count(&original), 1);
}

#[test]
fn arc_shares_same_allocation() {
    let original = Arc::new(42i32);
    let cloned = original.light_clone();
    assert!(Arc::ptr_eq(&original, &cloned));
}

#[test]
fn rc_i32_lc_returns_equal_value() {
    let original = Rc::new(42i32);
    let cloned = original.light_clone();
    assert_eq!(*original, *cloned);
}

#[test]
fn rc_str_lc_returns_equal_value() {
    let original: Rc<str> = Rc::from("hello world");
    let cloned = original.light_clone();
    assert_eq!(&*original, &*cloned);
}

#[test]
fn rc_slice_lc_returns_equal_value() {
    let original: Rc<[u8]> = Rc::from(vec![1u8, 2, 3, 4, 5]);
    let cloned = original.light_clone();
    assert_eq!(&*original, &*cloned);
}

#[test]
fn rc_strong_count_increments_after_lc() {
    let original = Rc::new(42i32);
    assert_eq!(Rc::strong_count(&original), 1);

    let cloned = original.light_clone();
    assert_eq!(Rc::strong_count(&original), 2);
    assert_eq!(Rc::strong_count(&cloned), 2);

    drop(cloned);
    assert_eq!(Rc::strong_count(&original), 1);
}

#[test]
fn rc_shares_same_allocation() {
    let original = Rc::new(42i32);
    let cloned = original.light_clone();
    assert!(Rc::ptr_eq(&original, &cloned));
}

#[test]
fn arc_with_large_inner_type_still_cheap() {
    // Even Arc containing a large Vec is O(1) to clone
    // because we're cloning the Arc, not the Vec
    let data = vec![0u8; 1_000_000];
    let original = Arc::new(data);
    let cloned = original.light_clone();
    assert!(Arc::ptr_eq(&original, &cloned));
    assert_eq!(Arc::strong_count(&original), 2);
}

#[test]
fn rc_with_large_inner_type_still_cheap() {
    let data = vec![0u8; 1_000_000];
    let original = Rc::new(data);
    let cloned = original.light_clone();
    assert!(Rc::ptr_eq(&original, &cloned));
    assert_eq!(Rc::strong_count(&original), 2);
}
