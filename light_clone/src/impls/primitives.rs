use crate::LightClone;

/// Macro to implement LightClone for Copy types.
/// For Copy types, `.light_clone()` is simply `*self` (bitwise copy).
macro_rules! impl_light_clone_for_copy {
    ($($t:ty),* $(,)?) => {
        $(
            impl LightClone for $t {
                #[inline]
                fn light_clone(&self) -> Self {
                    *self
                }
            }
        )*
    };
}

impl_light_clone_for_copy!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool, char,
);
