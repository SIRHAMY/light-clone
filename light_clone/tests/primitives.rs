use light_clone::LightClone;

/// Test that verifies LightClone: Clone bound is satisfied
fn assert_light_clone_implies_clone<T: LightClone>() {}

#[test]
fn light_clone_requires_clone() {
    assert_light_clone_implies_clone::<i32>();
    assert_light_clone_implies_clone::<u64>();
    assert_light_clone_implies_clone::<f64>();
    assert_light_clone_implies_clone::<bool>();
    assert_light_clone_implies_clone::<char>();
}

#[test]
fn test_signed_integers() {
    let i8_val: i8 = -42;
    assert_eq!(i8_val.light_clone(), i8_val);

    let i16_val: i16 = -1000;
    assert_eq!(i16_val.light_clone(), i16_val);

    let i32_val: i32 = -100_000;
    assert_eq!(i32_val.light_clone(), i32_val);

    let i64_val: i64 = -10_000_000_000;
    assert_eq!(i64_val.light_clone(), i64_val);

    let i128_val: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_727;
    assert_eq!(i128_val.light_clone(), i128_val);

    let isize_val: isize = -1024;
    assert_eq!(isize_val.light_clone(), isize_val);
}

#[test]
fn test_unsigned_integers() {
    let u8_val: u8 = 255;
    assert_eq!(u8_val.light_clone(), u8_val);

    let u16_val: u16 = 65535;
    assert_eq!(u16_val.light_clone(), u16_val);

    let u32_val: u32 = 4_294_967_295;
    assert_eq!(u32_val.light_clone(), u32_val);

    let u64_val: u64 = 18_446_744_073_709_551_615;
    assert_eq!(u64_val.light_clone(), u64_val);

    let u128_val: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    assert_eq!(u128_val.light_clone(), u128_val);

    let usize_val: usize = 1024;
    assert_eq!(usize_val.light_clone(), usize_val);
}

#[test]
fn test_floats() {
    let f32_val: f32 = 1.5;
    assert_eq!(f32_val.light_clone(), f32_val);

    let f64_val: f64 = 1.5;
    assert_eq!(f64_val.light_clone(), f64_val);

    // Test special float values
    let nan_f32: f32 = f32::NAN;
    assert!(nan_f32.light_clone().is_nan());

    let nan_f64: f64 = f64::NAN;
    assert!(nan_f64.light_clone().is_nan());

    let inf_f32: f32 = f32::INFINITY;
    assert_eq!(inf_f32.light_clone(), f32::INFINITY);

    let neg_inf_f64: f64 = f64::NEG_INFINITY;
    assert_eq!(neg_inf_f64.light_clone(), f64::NEG_INFINITY);
}

#[test]
fn test_bool() {
    let true_val = true;
    assert!(true_val.light_clone());

    let false_val = false;
    assert!(!false_val.light_clone());
}

#[test]
fn test_char() {
    let ascii = 'A';
    assert_eq!(ascii.light_clone(), 'A');

    let unicode = '日';
    assert_eq!(unicode.light_clone(), '日');

    let emoji = '🦀';
    assert_eq!(emoji.light_clone(), '🦀');
}

#[test]
fn test_lc_returns_independent_copy() {
    // For Copy types, the returned value should be independent
    let mut original: i32 = 42;
    let cloned = original.light_clone();
    original = 100;
    assert_eq!(cloned, 42);
    assert_eq!(original, 100);
}
