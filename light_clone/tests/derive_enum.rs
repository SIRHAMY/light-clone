use light_clone::LightClone;
use std::sync::Arc;

// Unit variant enum
#[derive(Clone, LightClone, PartialEq, Debug)]
#[allow(dead_code)]
enum Status {
    Pending,
    Active,
    Completed,
}

#[test]
fn test_unit_variants() {
    let s = Status::Pending;
    let cloned = s.light_clone();
    assert_eq!(cloned, Status::Pending);

    let s = Status::Active;
    let cloned = s.light_clone();
    assert_eq!(cloned, Status::Active);
}

// Tuple variant enum
#[derive(Clone, LightClone)]
enum Message {
    Quit,
    Move(i32, i32),
    Data(Arc<str>),
}

#[test]
fn test_tuple_variants() {
    let msg = Message::Quit;
    let _cloned = msg.light_clone();

    let msg = Message::Move(10, 20);
    let cloned = msg.light_clone();
    match cloned {
        Message::Move(x, y) => {
            assert_eq!(x, 10);
            assert_eq!(y, 20);
        }
        _ => panic!("Expected Move variant"),
    }
}

#[test]
fn test_tuple_variant_arc_sharing() {
    let data = Arc::from("shared data");
    let msg = Message::Data(Arc::clone(&data));
    let cloned = msg.light_clone();

    match (&msg, &cloned) {
        (Message::Data(orig), Message::Data(clone)) => {
            assert!(Arc::ptr_eq(orig, clone));
            assert_eq!(Arc::strong_count(orig), 3); // data + msg + cloned
        }
        _ => panic!("Expected Data variant"),
    }
}

// Struct variant enum
#[derive(Clone, LightClone)]
enum Event {
    Click { x: i32, y: i32 },
    KeyPress { key: Arc<str>, modifiers: u8 },
}

#[test]
fn test_struct_variants() {
    let event = Event::Click { x: 100, y: 200 };
    let cloned = event.light_clone();

    match cloned {
        Event::Click { x, y } => {
            assert_eq!(x, 100);
            assert_eq!(y, 200);
        }
        _ => panic!("Expected Click variant"),
    }
}

#[test]
fn test_struct_variant_arc_sharing() {
    let key = Arc::from("Enter");
    let event = Event::KeyPress {
        key: Arc::clone(&key),
        modifiers: 0b0011,
    };
    let cloned = event.light_clone();

    match (&event, &cloned) {
        (
            Event::KeyPress {
                key: orig_key,
                modifiers: orig_mod,
            },
            Event::KeyPress {
                key: clone_key,
                modifiers: clone_mod,
            },
        ) => {
            assert!(Arc::ptr_eq(orig_key, clone_key));
            assert_eq!(*orig_mod, *clone_mod);
        }
        _ => panic!("Expected KeyPress variant"),
    }
}

// Mixed variant enum
#[derive(Clone, LightClone)]
enum MixedEnum {
    Unit,
    Tuple(i32, Arc<str>),
    Struct { id: u64, name: Arc<str> },
}

#[test]
fn test_mixed_enum() {
    let unit = MixedEnum::Unit;
    let _cloned = unit.light_clone();

    let name = Arc::from("tuple");
    let tuple = MixedEnum::Tuple(42, Arc::clone(&name));
    let cloned = tuple.light_clone();
    match (&tuple, &cloned) {
        (MixedEnum::Tuple(_, orig), MixedEnum::Tuple(v, clone)) => {
            assert_eq!(*v, 42);
            assert!(Arc::ptr_eq(orig, clone));
        }
        _ => panic!("Expected Tuple variant"),
    }

    let name = Arc::from("struct");
    let s = MixedEnum::Struct {
        id: 123,
        name: Arc::clone(&name),
    };
    let cloned = s.light_clone();
    match (&s, &cloned) {
        (MixedEnum::Struct { name: orig, .. }, MixedEnum::Struct { id, name: clone }) => {
            assert_eq!(*id, 123);
            assert!(Arc::ptr_eq(orig, clone));
        }
        _ => panic!("Expected Struct variant"),
    }
}

// Generic enum
#[derive(Clone, LightClone)]
enum Option2<T: LightClone> {
    None,
    Some(T),
}

#[test]
fn test_generic_enum() {
    let opt: Option2<i32> = Option2::None;
    let _cloned = opt.light_clone();

    let opt = Option2::Some(42);
    let cloned = opt.light_clone();
    match cloned {
        Option2::Some(v) => assert_eq!(v, 42),
        _ => panic!("Expected Some variant"),
    }
}

#[test]
fn test_generic_enum_with_arc() {
    let data: Arc<str> = Arc::from("generic");
    let opt = Option2::Some(Arc::clone(&data));
    let cloned = opt.light_clone();

    match (&opt, &cloned) {
        (Option2::Some(orig), Option2::Some(clone)) => {
            assert!(Arc::ptr_eq(orig, clone));
        }
        _ => panic!("Expected Some variant"),
    }
}

// Clone and light_clone equivalence test
#[derive(Clone, LightClone)]
#[allow(dead_code)]
enum SimpleEnum {
    A(i32),
    B,
}

#[test]
fn test_clone_and_light_clone_equivalent() {
    let e = SimpleEnum::A(42);
    let cloned_via_clone = e.clone();
    let cloned_via_lc = e.light_clone();

    match (cloned_via_clone, cloned_via_lc) {
        (SimpleEnum::A(v1), SimpleEnum::A(v2)) => assert_eq!(v1, v2),
        _ => panic!("Expected A variant"),
    }
}

// Single-field tuple variant
#[derive(Clone, LightClone)]
enum SingleField {
    Value(Arc<str>),
}

#[test]
fn test_single_field_tuple_variant() {
    let data = Arc::from("single");
    let e = SingleField::Value(Arc::clone(&data));
    let cloned = e.light_clone();

    match (&e, &cloned) {
        (SingleField::Value(orig), SingleField::Value(clone)) => {
            assert!(Arc::ptr_eq(orig, clone));
        }
    }
}

// Empty enum (uninhabited type)
#[derive(Clone, LightClone)]
enum Empty {}

#[test]
fn test_empty_enum_compiles() {
    // Empty enums cannot be instantiated, but the derive should compile
    // and generate a valid impl
    fn _assert_light_clone<T: LightClone>() {}
    _assert_light_clone::<Empty>();
}
