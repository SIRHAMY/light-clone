use light_clone::LightClone;
use std::marker::PhantomData;
use std::sync::Arc;

#[test]
#[allow(clippy::let_unit_value, clippy::unit_cmp)]
fn unit_type_implements_light_clone() {
    let unit = ();
    let cloned = unit.light_clone();
    assert_eq!(unit, cloned);
}

#[test]
fn phantom_data_implements_light_clone() {
    let phantom: PhantomData<String> = PhantomData;
    let cloned = phantom.light_clone();
    assert_eq!(phantom, cloned);
}

#[test]
fn phantom_data_with_non_light_clone_inner_type() {
    // PhantomData<T> should work even if T doesn't implement LightClone
    // because PhantomData doesn't actually contain T
    let phantom: PhantomData<Vec<String>> = PhantomData;
    let cloned = phantom.light_clone();
    assert_eq!(phantom, cloned);
}

#[test]
fn option_some_with_arc_str() {
    let value: Option<Arc<str>> = Some(Arc::from("hello"));
    let cloned = value.light_clone();
    assert_eq!(value, cloned);

    // Verify Arc is shared (strong_count increases)
    if let (Some(original), Some(clone)) = (&value, &cloned) {
        assert!(Arc::ptr_eq(original, clone));
    }
}

#[test]
fn option_some_with_primitive() {
    let value: Option<i32> = Some(42);
    let cloned = value.light_clone();
    assert_eq!(value, cloned);
    assert_eq!(cloned, Some(42));
}

#[test]
fn option_none() {
    let value: Option<Arc<str>> = None;
    let cloned = value.light_clone();
    assert_eq!(value, cloned);
    assert!(cloned.is_none());
}

#[test]
fn result_ok_with_arc_str() {
    let value: Result<Arc<str>, String> = Ok(Arc::from("success"));
    let cloned = value.light_clone();

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
    let cloned = value.light_clone();

    match (&value, &cloned) {
        (Err(original), Err(clone)) => {
            assert_eq!(original, clone);
        }
        _ => panic!("Expected Err variants"),
    }
}

#[test]
fn result_with_clone_only_error_type() {
    // String implements Clone but not LightClone, which is allowed for E
    let value: Result<i32, String> = Err("error message".to_string());
    let cloned = value.light_clone();
    assert_eq!(value, cloned);
}

#[test]
fn array_of_primitives() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let cloned = arr.light_clone();
    assert_eq!(arr, cloned);
}

#[test]
fn array_of_bools() {
    let arr: [bool; 3] = [true, false, true];
    let cloned = arr.light_clone();
    assert_eq!(arr, cloned);
}

#[test]
fn array_empty() {
    let arr: [i32; 0] = [];
    let cloned = arr.light_clone();
    assert_eq!(arr, cloned);
}

#[test]
fn array_single_element() {
    let arr: [char; 1] = ['🦀'];
    let cloned = arr.light_clone();
    assert_eq!(arr, cloned);
}

#[test]
fn array_nested() {
    let arr: [[i32; 2]; 3] = [[1, 2], [3, 4], [5, 6]];
    let cloned = arr.light_clone();
    assert_eq!(arr, cloned);
}
