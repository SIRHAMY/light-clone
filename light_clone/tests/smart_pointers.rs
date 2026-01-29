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

// Weak pointer tests

#[test]
fn arc_weak_lc_returns_equivalent_weak() {
    let strong = Arc::new(42i32);
    let weak = Arc::downgrade(&strong);
    let cloned = weak.light_clone();

    // Both weak pointers should upgrade to the same Arc
    assert_eq!(weak.upgrade().map(|a| *a), Some(42));
    assert_eq!(cloned.upgrade().map(|a| *a), Some(42));
}

#[test]
fn arc_weak_count_increments_after_lc() {
    let strong = Arc::new(42i32);
    let weak = Arc::downgrade(&strong);
    assert_eq!(Arc::weak_count(&strong), 1);

    let cloned = weak.light_clone();
    assert_eq!(Arc::weak_count(&strong), 2);

    drop(cloned);
    assert_eq!(Arc::weak_count(&strong), 1);
}

#[test]
fn arc_weak_after_strong_dropped() {
    let strong = Arc::new(42i32);
    let weak = Arc::downgrade(&strong);
    drop(strong);

    // Weak should be expired
    assert!(weak.upgrade().is_none());

    // Clone should also be expired
    let cloned = weak.light_clone();
    assert!(cloned.upgrade().is_none());
}

#[test]
fn rc_weak_lc_returns_equivalent_weak() {
    let strong = Rc::new(42i32);
    let weak = Rc::downgrade(&strong);
    let cloned = weak.light_clone();

    assert_eq!(weak.upgrade().map(|a| *a), Some(42));
    assert_eq!(cloned.upgrade().map(|a| *a), Some(42));
}

#[test]
fn rc_weak_count_increments_after_lc() {
    let strong = Rc::new(42i32);
    let weak = Rc::downgrade(&strong);
    assert_eq!(Rc::weak_count(&strong), 1);

    let cloned = weak.light_clone();
    assert_eq!(Rc::weak_count(&strong), 2);

    drop(cloned);
    assert_eq!(Rc::weak_count(&strong), 1);
}

#[test]
fn rc_weak_after_strong_dropped() {
    let strong = Rc::new(42i32);
    let weak = Rc::downgrade(&strong);
    drop(strong);

    assert!(weak.upgrade().is_none());

    let cloned = weak.light_clone();
    assert!(cloned.upgrade().is_none());
}
