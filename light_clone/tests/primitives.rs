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

#[test]
fn test_nonzero_unsigned() {
    use std::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

    let nz_u8 = NonZeroU8::new(1).unwrap();
    assert_eq!(nz_u8.light_clone(), nz_u8);

    let nz_u16 = NonZeroU16::new(1000).unwrap();
    assert_eq!(nz_u16.light_clone(), nz_u16);

    let nz_u32 = NonZeroU32::new(100_000).unwrap();
    assert_eq!(nz_u32.light_clone(), nz_u32);

    let nz_u64 = NonZeroU64::new(10_000_000_000).unwrap();
    assert_eq!(nz_u64.light_clone(), nz_u64);

    let nz_u128 = NonZeroU128::new(1).unwrap();
    assert_eq!(nz_u128.light_clone(), nz_u128);

    let nz_usize = NonZeroUsize::new(1024).unwrap();
    assert_eq!(nz_usize.light_clone(), nz_usize);
}

#[test]
fn test_nonzero_signed() {
    use std::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};

    let nz_i8 = NonZeroI8::new(-42).unwrap();
    assert_eq!(nz_i8.light_clone(), nz_i8);

    let nz_i16 = NonZeroI16::new(-1000).unwrap();
    assert_eq!(nz_i16.light_clone(), nz_i16);

    let nz_i32 = NonZeroI32::new(-100_000).unwrap();
    assert_eq!(nz_i32.light_clone(), nz_i32);

    let nz_i64 = NonZeroI64::new(-10_000_000_000).unwrap();
    assert_eq!(nz_i64.light_clone(), nz_i64);

    let nz_i128 = NonZeroI128::new(-1).unwrap();
    assert_eq!(nz_i128.light_clone(), nz_i128);

    let nz_isize = NonZeroIsize::new(-1024).unwrap();
    assert_eq!(nz_isize.light_clone(), nz_isize);
}

#[test]
fn test_duration() {
    use std::time::Duration;

    let duration = Duration::from_secs(60);
    assert_eq!(duration.light_clone(), duration);

    let nanos = Duration::from_nanos(12345);
    assert_eq!(nanos.light_clone(), nanos);

    let zero = Duration::ZERO;
    assert_eq!(zero.light_clone(), zero);
}

#[test]
fn test_instant() {
    use std::time::Instant;

    let now = Instant::now();
    let cloned = now.light_clone();
    assert_eq!(now, cloned);
}
