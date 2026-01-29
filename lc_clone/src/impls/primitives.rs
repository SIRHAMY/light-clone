use crate::LcClone;

/// Macro to implement LcClone for Copy types.
/// For Copy types, `.lc()` is simply `*self` (bitwise copy).
macro_rules! impl_lc_clone_for_copy {
    ($($t:ty),* $(,)?) => {
        $(
            impl LcClone for $t {
                #[inline]
                fn lc(&self) -> Self {
                    *self
                }
            }
        )*
    };
}

impl_lc_clone_for_copy!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char,
);
