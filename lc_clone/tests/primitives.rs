use lc_clone::LcClone;

/// Test that verifies LcClone: Clone bound is satisfied
fn assert_lc_clone_implies_clone<T: LcClone>() {}

#[test]
fn lc_clone_requires_clone() {
    assert_lc_clone_implies_clone::<i32>();
    assert_lc_clone_implies_clone::<u64>();
    assert_lc_clone_implies_clone::<f64>();
    assert_lc_clone_implies_clone::<bool>();
    assert_lc_clone_implies_clone::<char>();
}

#[test]
fn test_signed_integers() {
    let i8_val: i8 = -42;
    assert_eq!(i8_val.lc(), i8_val);

    let i16_val: i16 = -1000;
    assert_eq!(i16_val.lc(), i16_val);

    let i32_val: i32 = -100_000;
    assert_eq!(i32_val.lc(), i32_val);

    let i64_val: i64 = -10_000_000_000;
    assert_eq!(i64_val.lc(), i64_val);

    let i128_val: i128 = -170_141_183_460_469_231_731_687_303_715_884_105_727;
    assert_eq!(i128_val.lc(), i128_val);

    let isize_val: isize = -1024;
    assert_eq!(isize_val.lc(), isize_val);
}

#[test]
fn test_unsigned_integers() {
    let u8_val: u8 = 255;
    assert_eq!(u8_val.lc(), u8_val);

    let u16_val: u16 = 65535;
    assert_eq!(u16_val.lc(), u16_val);

    let u32_val: u32 = 4_294_967_295;
    assert_eq!(u32_val.lc(), u32_val);

    let u64_val: u64 = 18_446_744_073_709_551_615;
    assert_eq!(u64_val.lc(), u64_val);

    let u128_val: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    assert_eq!(u128_val.lc(), u128_val);

    let usize_val: usize = 1024;
    assert_eq!(usize_val.lc(), usize_val);
}

#[test]
fn test_floats() {
    let f32_val: f32 = 1.5;
    assert_eq!(f32_val.lc(), f32_val);

    let f64_val: f64 = 1.5;
    assert_eq!(f64_val.lc(), f64_val);

    // Test special float values
    let nan_f32: f32 = f32::NAN;
    assert!(nan_f32.lc().is_nan());

    let nan_f64: f64 = f64::NAN;
    assert!(nan_f64.lc().is_nan());

    let inf_f32: f32 = f32::INFINITY;
    assert_eq!(inf_f32.lc(), f32::INFINITY);

    let neg_inf_f64: f64 = f64::NEG_INFINITY;
    assert_eq!(neg_inf_f64.lc(), f64::NEG_INFINITY);
}

#[test]
fn test_bool() {
    let true_val = true;
    assert!(true_val.lc());

    let false_val = false;
    assert!(!false_val.lc());
}

#[test]
fn test_char() {
    let ascii = 'A';
    assert_eq!(ascii.lc(), 'A');

    let unicode = '日';
    assert_eq!(unicode.lc(), '日');

    let emoji = '🦀';
    assert_eq!(emoji.lc(), '🦀');
}

#[test]
fn test_lc_returns_independent_copy() {
    // For Copy types, the returned value should be independent
    let mut original: i32 = 42;
    let cloned = original.lc();
    original = 100;
    assert_eq!(cloned, 42);
    assert_eq!(original, 100);
}
