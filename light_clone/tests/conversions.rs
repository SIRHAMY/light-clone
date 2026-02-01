use light_clone::{IntoLightStr, LightClone, LightStr};
use std::sync::Arc;

#[test]
fn into_light_str_from_str_literal() {
    let s = "hello".into_light_str();
    assert_eq!(&*s, "hello");
}

#[test]
fn into_light_str_from_owned_string() {
    let owned = String::from("world");
    let s = owned.into_light_str();
    assert_eq!(&*s, "world");
}

#[test]
fn into_light_str_from_string_ref() {
    let owned = String::from("test");
    let s = (&owned).into_light_str();
    assert_eq!(&*s, "test");
    // owned is still available
    assert_eq!(owned, "test");
}

#[test]
fn into_light_str_returns_light_str_type() {
    let s: LightStr = "hello".into_light_str();
    let _arc: Arc<str> = s; // Compiles because LightStr is Arc<str>
}

#[test]
fn into_light_str_result_is_light_clone() {
    let s = "hello".into_light_str();
    let cloned = s.light_clone();
    assert_eq!(&*s, &*cloned);
    assert_eq!(Arc::strong_count(&s), 2);
}

#[test]
fn into_light_str_str_in_derived_struct() {
    #[derive(Clone, LightClone, Debug, PartialEq)]
    struct Config {
        host: LightStr,
        port: u16,
    }

    let config = Config {
        host: "localhost".into_light_str(),
        port: 8080,
    };

    let cloned = config.light_clone();
    assert_eq!(config, cloned);
}

#[test]
fn into_light_str_empty_string() {
    let s = "".into_light_str();
    assert_eq!(s.len(), 0);
    assert_eq!(&*s, "");
}

#[test]
fn into_light_str_unicode_string() {
    let s = "こんにちは".into_light_str();
    assert_eq!(&*s, "こんにちは");
    assert_eq!(s.chars().count(), 5);
}

#[test]
fn into_light_str_long_string() {
    let long = "x".repeat(10_000);
    let s = long.into_light_str();
    assert_eq!(s.len(), 10_000);
}

#[test]
fn multiple_into_light_str_calls() {
    let s1 = "first".into_light_str();
    let s2 = "second".into_light_str();
    let s3 = "third".into_light_str();

    assert_eq!(&*s1, "first");
    assert_eq!(&*s2, "second");
    assert_eq!(&*s3, "third");
}

#[test]
fn into_light_str_from_light_str_is_idempotent() {
    let s: LightStr = Arc::from("hello");
    let s2 = s.clone().into_light_str();
    assert_eq!(&*s2, "hello");
    // Both refer to the same allocation
    assert!(Arc::ptr_eq(&s, &s2));
}

#[test]
fn into_light_str_works_in_generic_context() {
    fn takes_into_light_str(s: impl IntoLightStr) -> LightStr {
        s.into_light_str()
    }

    // Works with &str
    let s1 = takes_into_light_str("hello");
    assert_eq!(&*s1, "hello");

    // Works with String
    let s2 = takes_into_light_str(String::from("world"));
    assert_eq!(&*s2, "world");

    // Works with LightStr
    let s3: LightStr = Arc::from("existing");
    let s4 = takes_into_light_str(s3.clone());
    assert_eq!(&*s4, "existing");
    assert!(Arc::ptr_eq(&s3, &s4));
}
