//! LightClone implementations for the `im` crate's persistent collections.
//!
//! These implementations are behind the `im` feature flag.

use crate::LightClone;

impl<T: Clone> LightClone for im::Vector<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<K, V> LightClone for im::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<K, V> LightClone for im::OrdMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<T> LightClone for im::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<T> LightClone for im::OrdSet<T>
where
    T: Clone + Ord,
{
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_implements_lc_clone() {
        let v: im::Vector<i32> = im::vector![1, 2, 3];
        let cloned = v.light_clone();
        assert_eq!(v, cloned);
    }

    #[test]
    fn hash_map_implements_lc_clone() {
        let mut m = im::HashMap::new();
        m.insert("key", 42);
        let cloned = m.light_clone();
        assert_eq!(m, cloned);
    }

    #[test]
    fn ord_map_implements_lc_clone() {
        let mut m = im::OrdMap::new();
        m.insert("key", 42);
        let cloned = m.light_clone();
        assert_eq!(m, cloned);
    }

    #[test]
    fn hash_set_implements_lc_clone() {
        let mut s = im::HashSet::new();
        s.insert(42);
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }

    #[test]
    fn ord_set_implements_lc_clone() {
        let mut s = im::OrdSet::new();
        s.insert(42);
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }
}
