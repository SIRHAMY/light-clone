use lc_clone::{LcClone, LcStr};
use std::sync::Arc;

#[test]
fn lc_str_is_arc_str() {
    // LcStr is a type alias for Arc<str>
    let s: LcStr = Arc::from("hello");
    let _arc: Arc<str> = s; // This compiles because they're the same type
}

#[test]
fn lc_str_implements_lc_clone() {
    let s: LcStr = Arc::from("hello");
    let cloned = s.lc();
    assert_eq!(&*s, &*cloned);
    assert_eq!(Arc::strong_count(&s), 2);
}

#[test]
fn lc_str_clone_is_o1() {
    let s: LcStr = Arc::from("hello");
    assert_eq!(Arc::strong_count(&s), 1);

    let s2 = s.clone();
    assert_eq!(Arc::strong_count(&s), 2);
    assert_eq!(Arc::strong_count(&s2), 2);

    let s3 = s.lc();
    assert_eq!(Arc::strong_count(&s), 3);
    assert_eq!(&*s, &*s3);
}

#[test]
fn lc_str_can_be_used_in_derived_struct() {
    #[derive(LcClone, Debug, PartialEq)]
    struct Person {
        name: LcStr,
        email: LcStr,
    }

    let person = Person {
        name: Arc::from("Alice"),
        email: Arc::from("alice@example.com"),
    };

    let cloned = person.lc();
    assert_eq!(person, cloned);
}

#[test]
fn lc_str_works_with_arc_methods() {
    let s: LcStr = Arc::from("hello world");

    // Can use Arc methods
    assert_eq!(Arc::strong_count(&s), 1);

    // Can use str methods
    assert!(s.contains("world"));
    assert_eq!(s.len(), 11);
    assert_eq!(&s[0..5], "hello");
}

#[test]
fn lc_str_accepts_arc_str_functions() {
    fn accepts_arc_str(s: &Arc<str>) -> usize {
        s.len()
    }

    let s: LcStr = Arc::from("test");
    assert_eq!(accepts_arc_str(&s), 4);
}
