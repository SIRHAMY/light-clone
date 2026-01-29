use lc_clone::LcClone;
use std::marker::PhantomData;
use std::sync::Arc;

#[test]
fn unit_type_implements_lc_clone() {
    let unit = ();
    let cloned = unit.lc();
    assert_eq!(unit, cloned);
}

#[test]
fn phantom_data_implements_lc_clone() {
    let phantom: PhantomData<String> = PhantomData;
    let cloned = phantom.lc();
    assert_eq!(phantom, cloned);
}

#[test]
fn phantom_data_with_non_lc_clone_inner_type() {
    // PhantomData<T> should work even if T doesn't implement LcClone
    // because PhantomData doesn't actually contain T
    let phantom: PhantomData<Vec<String>> = PhantomData;
    let cloned = phantom.lc();
    assert_eq!(phantom, cloned);
}

#[test]
fn option_some_with_arc_str() {
    let value: Option<Arc<str>> = Some(Arc::from("hello"));
    let cloned = value.lc();
    assert_eq!(value, cloned);

    // Verify Arc is shared (strong_count increases)
    if let (Some(original), Some(clone)) = (&value, &cloned) {
        assert!(Arc::ptr_eq(original, clone));
    }
}

#[test]
fn option_some_with_primitive() {
    let value: Option<i32> = Some(42);
    let cloned = value.lc();
    assert_eq!(value, cloned);
    assert_eq!(cloned, Some(42));
}

#[test]
fn option_none() {
    let value: Option<Arc<str>> = None;
    let cloned = value.lc();
    assert_eq!(value, cloned);
    assert!(cloned.is_none());
}

#[test]
fn result_ok_with_arc_str() {
    let value: Result<Arc<str>, String> = Ok(Arc::from("success"));
    let cloned = value.lc();

    match (&value, &cloned) {
        (Ok(original), Ok(clone)) => {
            assert!(Arc::ptr_eq(original, clone));
        }
        _ => panic!("Expected Ok variants"),
    }
}

#[test]
fn result_err_case() {
    let value: Result<Arc<str>, String> = Err("error".to_string());
    let cloned = value.lc();

    match (&value, &cloned) {
        (Err(original), Err(clone)) => {
            assert_eq!(original, clone);
        }
        _ => panic!("Expected Err variants"),
    }
}

#[test]
fn result_with_clone_only_error_type() {
    // String implements Clone but not LcClone, which is allowed for E
    let value: Result<i32, String> = Err("error message".to_string());
    let cloned = value.lc();
    assert_eq!(value, cloned);
}
