use light_clone::LightClone;

#[test]
fn shared_ref_to_str() {
    // &str is an unsized type, so there's no auto-deref issue
    let s: &str = "hello world";
    let cloned: &str = s.light_clone();
    assert_eq!(s, cloned);
    assert!(std::ptr::eq(s, cloned));
}

#[test]
fn shared_ref_to_slice() {
    let arr = [1, 2, 3, 4, 5];
    let slice: &[i32] = &arr;
    let cloned: &[i32] = slice.light_clone();
    assert_eq!(slice, cloned);
    assert!(std::ptr::eq(slice.as_ptr(), cloned.as_ptr()));
}

#[test]
fn shared_ref_to_unsized_type() {
    // References to unsized types like [T] and str
    let data: &[u8] = &[1, 2, 3];
    let cloned: &[u8] = data.light_clone();
    assert_eq!(data, cloned);
}

#[test]
fn shared_ref_via_explicit_trait_call() {
    // For sized types, auto-deref prefers the inner type's impl.
    // We can test the reference impl explicitly using turbofish.
    let value: i32 = 42;
    let reference: &i32 = &value;
    let cloned: &i32 = <&i32 as LightClone>::light_clone(&reference);
    assert_eq!(*reference, *cloned);
    assert!(std::ptr::eq(reference, cloned));
}
