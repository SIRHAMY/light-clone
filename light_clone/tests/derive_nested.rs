use light_clone::LightClone;
use std::sync::Arc;

#[derive(LightClone)]
struct Inner {
    value: i32,
    name: Arc<str>,
}

#[derive(LightClone)]
struct Outer {
    inner: Inner,
    id: u64,
}

#[test]
fn test_nested_struct() {
    let outer = Outer {
        inner: Inner {
            value: 42,
            name: Arc::from("nested"),
        },
        id: 100,
    };

    let cloned = outer.light_clone();
    assert_eq!(cloned.inner.value, 42);
    assert_eq!(&*cloned.inner.name, "nested");
    assert_eq!(cloned.id, 100);

    // Verify the Arc in the nested struct is shared
    assert!(Arc::ptr_eq(&outer.inner.name, &cloned.inner.name));
}

#[derive(LightClone)]
struct DeeplyNested {
    outer: Outer,
    extra: Arc<str>,
}

#[test]
fn test_deeply_nested_struct() {
    let deep = DeeplyNested {
        outer: Outer {
            inner: Inner {
                value: 1,
                name: Arc::from("deep"),
            },
            id: 2,
        },
        extra: Arc::from("extra"),
    };

    let cloned = deep.light_clone();
    assert_eq!(cloned.outer.inner.value, 1);
    assert_eq!(&*cloned.outer.inner.name, "deep");
    assert!(Arc::ptr_eq(
        &deep.outer.inner.name,
        &cloned.outer.inner.name
    ));
    assert!(Arc::ptr_eq(&deep.extra, &cloned.extra));
}

#[derive(LightClone)]
struct WithOption {
    maybe: Option<Arc<str>>,
    definitely: i32,
}

#[test]
fn test_struct_with_option_field() {
    let with_some = WithOption {
        maybe: Some(Arc::from("present")),
        definitely: 42,
    };

    let cloned = with_some.light_clone();
    assert!(cloned.maybe.is_some());
    // Verify Arc sharing (the key semantic of LightClone)
    assert!(Arc::ptr_eq(
        with_some.maybe.as_ref().unwrap(),
        cloned.maybe.as_ref().unwrap()
    ));

    let with_none = WithOption {
        maybe: None,
        definitely: 0,
    };

    let cloned_none = with_none.light_clone();
    assert!(cloned_none.maybe.is_none());
}
