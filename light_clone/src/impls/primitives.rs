use crate::LightClone;
use std::num::{
    NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize,
};
use std::time::{Duration, Instant};

/// Macro to implement LightClone for types.
/// With the marker trait pattern, all impls are empty - the default impl calls `clone()`.
macro_rules! impl_light_clone {
    ($($t:ty),* $(,)?) => {
        $(
            impl LightClone for $t {}
        )*
    };
}

impl_light_clone!(
    // Primitives
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    f32,
    f64,
    bool,
    char,
    // NonZero types
    NonZeroI8,
    NonZeroI16,
    NonZeroI32,
    NonZeroI64,
    NonZeroI128,
    NonZeroIsize,
    NonZeroU8,
    NonZeroU16,
    NonZeroU32,
    NonZeroU64,
    NonZeroU128,
    NonZeroUsize,
    // Time types
    Duration,
    Instant,
);

// Shared references are Copy types
impl<T: ?Sized> LightClone for &T {}
