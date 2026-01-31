//! LightClone implementations for the `rpds` crate's persistent collections.
//!
//! These implementations are behind the `rpds` feature flag.
//!
//! Note: rpds data structures are parameterized by a shared pointer kind (Arc or Rc).
//! The default type aliases (e.g., `Vector<T>`) use Arc, which is thread-safe.
//! We implement LightClone for the sync (Arc-based) variants.

use crate::LightClone;

impl<T: Clone> LightClone for rpds::Vector<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<K, V> LightClone for rpds::HashTrieMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<K, V> LightClone for rpds::RedBlackTreeMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<T> LightClone for rpds::HashTrieSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<T> LightClone for rpds::RedBlackTreeSet<T>
where
    T: Clone + Ord,
{
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<T: Clone> LightClone for rpds::List<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<T: Clone> LightClone for rpds::Queue<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

impl<T: Clone> LightClone for rpds::Stack<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_implements_light_clone() {
        let v: rpds::Vector<i32> = rpds::Vector::new().push_back(1).push_back(2).push_back(3);
        let cloned = v.light_clone();
        assert_eq!(v, cloned);
    }

    #[test]
    fn hash_trie_map_implements_light_clone() {
        let m: rpds::HashTrieMap<&str, i32> = rpds::HashTrieMap::new().insert("key", 42);
        let cloned = m.light_clone();
        assert_eq!(m, cloned);
    }

    #[test]
    fn red_black_tree_map_implements_light_clone() {
        let m: rpds::RedBlackTreeMap<&str, i32> = rpds::RedBlackTreeMap::new().insert("key", 42);
        let cloned = m.light_clone();
        assert_eq!(m, cloned);
    }

    #[test]
    fn hash_trie_set_implements_light_clone() {
        let s: rpds::HashTrieSet<i32> = rpds::HashTrieSet::new().insert(42);
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }

    #[test]
    fn red_black_tree_set_implements_light_clone() {
        let s: rpds::RedBlackTreeSet<i32> = rpds::RedBlackTreeSet::new().insert(42);
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }

    #[test]
    fn list_implements_light_clone() {
        let l: rpds::List<i32> = rpds::List::new().push_front(1).push_front(2);
        let cloned = l.light_clone();
        assert_eq!(l, cloned);
    }

    #[test]
    fn queue_implements_light_clone() {
        let q: rpds::Queue<i32> = rpds::Queue::new().enqueue(1).enqueue(2);
        let cloned = q.light_clone();
        assert_eq!(q, cloned);
    }

    #[test]
    fn stack_implements_light_clone() {
        let s: rpds::Stack<i32> = rpds::Stack::new().push(1).push(2);
        let cloned = s.light_clone();
        assert_eq!(s, cloned);
    }
}
