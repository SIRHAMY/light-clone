//! LightClone implementations for the `ordered-float` crate.
//!
//! These implementations are behind the `ordered-float` feature flag.
//!
//! Both `OrderedFloat<T>` and `NotNan<T>` are `Copy` when `T` is `Copy`,
//! so cloning is always O(1).

use crate::LightClone;

impl<T: Copy> LightClone for ordered_float::OrderedFloat<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        *self
    }
}

impl<T: Copy> LightClone for ordered_float::NotNan<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ordered_float::{NotNan, OrderedFloat};

    #[test]
    fn ordered_float_f32_implements_light_clone() {
        let value = OrderedFloat(1.5f32);
        let cloned = value.light_clone();
        assert_eq!(value, cloned);
    }

    #[test]
    fn ordered_float_f64_implements_light_clone() {
        let value = OrderedFloat(1.5f64);
        let cloned = value.light_clone();
        assert_eq!(value, cloned);
    }

    #[test]
    fn not_nan_f32_implements_light_clone() {
        let value = NotNan::new(1.5f32).unwrap();
        let cloned = value.light_clone();
        assert_eq!(value, cloned);
    }

    #[test]
    fn not_nan_f64_implements_light_clone() {
        let value = NotNan::new(1.5f64).unwrap();
        let cloned = value.light_clone();
        assert_eq!(value, cloned);
    }
}
