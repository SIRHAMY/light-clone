//! LcClone implementations for the `imbl` crate's persistent collections.
//!
//! These implementations are behind the `imbl` feature flag.
//!
//! `imbl` is a maintained fork of `im` with improved performance and
//! better compatibility with newer Rust versions.

use crate::LcClone;

impl<T: Clone> LcClone for imbl::Vector<T> {
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<K, V> LcClone for imbl::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<K, V> LcClone for imbl::OrdMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<T> LcClone for imbl::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<T> LcClone for imbl::OrdSet<T>
where
    T: Clone + Ord,
{
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_implements_lc_clone() {
        let v: imbl::Vector<i32> = imbl::vector![1, 2, 3];
        let cloned = v.lc();
        assert_eq!(v, cloned);
    }

    #[test]
    fn hash_map_implements_lc_clone() {
        let mut m = imbl::HashMap::new();
        m.insert("key", 42);
        let cloned = m.lc();
        assert_eq!(m, cloned);
    }

    #[test]
    fn ord_map_implements_lc_clone() {
        let mut m = imbl::OrdMap::new();
        m.insert("key", 42);
        let cloned = m.lc();
        assert_eq!(m, cloned);
    }

    #[test]
    fn hash_set_implements_lc_clone() {
        let mut s = imbl::HashSet::new();
        s.insert(42);
        let cloned = s.lc();
        assert_eq!(s, cloned);
    }

    #[test]
    fn ord_set_implements_lc_clone() {
        let mut s = imbl::OrdSet::new();
        s.insert(42);
        let cloned = s.lc();
        assert_eq!(s, cloned);
    }
}
