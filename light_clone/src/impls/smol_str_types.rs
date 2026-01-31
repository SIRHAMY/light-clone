//! LightClone implementation for the `smol_str` crate.
//!
//! This implementation is behind the `smol_str` feature flag.
//!
//! `SmolStr` is an immutable string type that stores small strings inline
//! and larger strings in an `Arc`. Cloning is always O(1) - either a
//! bitwise copy for inline strings or an atomic refcount increment for
//! heap-allocated strings.

use crate::LightClone;

impl LightClone for smol_str::SmolStr {
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smol_str::SmolStr;

    #[test]
    fn smol_str_small_implements_light_clone() {
        // Small strings are stored inline
        let s = SmolStr::new("hello");
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }

    #[test]
    fn smol_str_large_implements_light_clone() {
        // Larger strings use Arc
        let s = SmolStr::new("this is a longer string that exceeds inline capacity");
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }

    #[test]
    fn smol_str_from_static_implements_light_clone() {
        let s = SmolStr::new_static("static string");
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }
}
