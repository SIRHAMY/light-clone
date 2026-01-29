use lc_clone::{IntoLcStr, LcClone, LcStr};
use std::sync::Arc;

#[test]
fn into_lc_from_str_literal() {
    let s = "hello".into_lc();
    assert_eq!(&*s, "hello");
}

#[test]
fn into_lc_from_owned_string() {
    let owned = String::from("world");
    let s = owned.into_lc();
    assert_eq!(&*s, "world");
}

#[test]
fn into_lc_from_string_ref() {
    let owned = String::from("test");
    let s = (&owned).into_lc();
    assert_eq!(&*s, "test");
    // owned is still available
    assert_eq!(owned, "test");
}

#[test]
fn into_lc_returns_lc_str_type() {
    let s: LcStr = "hello".into_lc();
    let _arc: Arc<str> = s; // Compiles because LcStr is Arc<str>
}

#[test]
fn into_lc_result_is_lc_clone() {
    let s = "hello".into_lc();
    let cloned = s.lc();
    assert_eq!(&*s, &*cloned);
    assert_eq!(Arc::strong_count(&s), 2);
}

#[test]
fn into_lc_str_in_derived_struct() {
    #[derive(LcClone, Debug, PartialEq)]
    struct Config {
        host: LcStr,
        port: u16,
    }

    let config = Config {
        host: "localhost".into_lc(),
        port: 8080,
    };

    let cloned = config.lc();
    assert_eq!(config, cloned);
}

#[test]
fn into_lc_empty_string() {
    let s = "".into_lc();
    assert_eq!(s.len(), 0);
    assert_eq!(&*s, "");
}

#[test]
fn into_lc_unicode_string() {
    let s = "こんにちは".into_lc();
    assert_eq!(&*s, "こんにちは");
    assert_eq!(s.chars().count(), 5);
}

#[test]
fn into_lc_long_string() {
    let long = "x".repeat(10_000);
    let s = long.into_lc();
    assert_eq!(s.len(), 10_000);
}

#[test]
fn multiple_into_lc_calls() {
    let s1 = "first".into_lc();
    let s2 = "second".into_lc();
    let s3 = "third".into_lc();

    assert_eq!(&*s1, "first");
    assert_eq!(&*s2, "second");
    assert_eq!(&*s3, "third");
}

#[test]
fn into_lc_from_lc_str_is_idempotent() {
    let s: LcStr = Arc::from("hello");
    let s2 = s.clone().into_lc();
    assert_eq!(&*s2, "hello");
    // Both refer to the same allocation
    assert!(Arc::ptr_eq(&s, &s2));
}

#[test]
fn into_lc_works_in_generic_context() {
    fn takes_into_lc(s: impl IntoLcStr) -> LcStr {
        s.into_lc()
    }

    // Works with &str
    let s1 = takes_into_lc("hello");
    assert_eq!(&*s1, "hello");

    // Works with String
    let s2 = takes_into_lc(String::from("world"));
    assert_eq!(&*s2, "world");

    // Works with LcStr
    let s3: LcStr = Arc::from("existing");
    let s4 = takes_into_lc(s3.clone());
    assert_eq!(&*s4, "existing");
    assert!(Arc::ptr_eq(&s3, &s4));
}
