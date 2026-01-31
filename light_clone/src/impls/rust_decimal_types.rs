//! LightClone implementation for the `rust_decimal` crate.
//!
//! This implementation is behind the `rust_decimal` feature flag.
//!
//! `Decimal` is a 128-bit decimal type that implements `Copy`,
//! so cloning is always O(1).

use crate::LightClone;

impl LightClone for rust_decimal::Decimal {
    #[inline]
    fn light_clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use std::str::FromStr;

    #[test]
    fn decimal_implements_light_clone() {
        let value = Decimal::from_str("123.456").unwrap();
        let cloned = value.light_clone();
        assert_eq!(value, cloned);
    }

    #[test]
    fn decimal_from_i64_implements_light_clone() {
        let value = Decimal::from(42i64);
        let cloned = value.light_clone();
        assert_eq!(value, cloned);
    }
}
