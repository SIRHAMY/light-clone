//! LightClone implementations for the `imbl` crate's persistent collections.
//!
//! These implementations are behind the `imbl` feature flag.
//!
//! `imbl` is a maintained fork of `im` with improved performance and
//! better compatibility with newer Rust versions.

use crate::LightClone;

impl<T: Clone> LightClone for imbl::Vector<T> {}

impl<K, V> LightClone for imbl::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
}

impl<K, V> LightClone for imbl::OrdMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
}

impl<T> LightClone for imbl::HashSet<T> where T: Clone + std::hash::Hash + Eq {}

impl<T> LightClone for imbl::OrdSet<T> where T: Clone + Ord {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_implements_light_clone() {
        let v: imbl::Vector<i32> = imbl::vector![1, 2, 3];
        let cloned = v.light_clone();
        assert_eq!(v, cloned);
    }

    #[test]
    fn hash_map_implements_light_clone() {
        let mut m = imbl::HashMap::new();
        m.insert("key", 42);
        let cloned = m.light_clone();
        assert_eq!(m, cloned);
    }

    #[test]
    fn ord_map_implements_light_clone() {
        let mut m = imbl::OrdMap::new();
        m.insert("key", 42);
        let cloned = m.light_clone();
        assert_eq!(m, cloned);
    }

    #[test]
    fn hash_set_implements_light_clone() {
        let mut s = imbl::HashSet::new();
        s.insert(42);
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }

    #[test]
    fn ord_set_implements_light_clone() {
        let mut s = imbl::OrdSet::new();
        s.insert(42);
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }
}
