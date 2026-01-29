//! LcClone implementations for the `rpds` crate's persistent collections.
//!
//! These implementations are behind the `rpds` feature flag.
//!
//! Note: rpds data structures are parameterized by a shared pointer kind (Arc or Rc).
//! The default type aliases (e.g., `Vector<T>`) use Arc, which is thread-safe.
//! We implement LcClone for the sync (Arc-based) variants.

use crate::LcClone;

impl<T: Clone> LcClone for rpds::Vector<T> {
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<K, V> LcClone for rpds::HashTrieMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<K, V> LcClone for rpds::RedBlackTreeMap<K, V>
where
    K: Clone + Ord,
    V: Clone,
{
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<T> LcClone for rpds::HashTrieSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<T> LcClone for rpds::RedBlackTreeSet<T>
where
    T: Clone + Ord,
{
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<T: Clone> LcClone for rpds::List<T> {
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<T: Clone> LcClone for rpds::Queue<T> {
    #[inline]
    fn lc(&self) -> Self {
        self.clone()
    }
}

impl<T: Clone> LcClone for rpds::Stack<T> {
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
        let v: rpds::Vector<i32> = rpds::Vector::new().push_back(1).push_back(2).push_back(3);
        let cloned = v.lc();
        assert_eq!(v, cloned);
    }

    #[test]
    fn hash_trie_map_implements_lc_clone() {
        let m: rpds::HashTrieMap<&str, i32> = rpds::HashTrieMap::new().insert("key", 42);
        let cloned = m.lc();
        assert_eq!(m, cloned);
    }

    #[test]
    fn red_black_tree_map_implements_lc_clone() {
        let m: rpds::RedBlackTreeMap<&str, i32> = rpds::RedBlackTreeMap::new().insert("key", 42);
        let cloned = m.lc();
        assert_eq!(m, cloned);
    }

    #[test]
    fn hash_trie_set_implements_lc_clone() {
        let s: rpds::HashTrieSet<i32> = rpds::HashTrieSet::new().insert(42);
        let cloned = s.lc();
        assert_eq!(s, cloned);
    }

    #[test]
    fn red_black_tree_set_implements_lc_clone() {
        let s: rpds::RedBlackTreeSet<i32> = rpds::RedBlackTreeSet::new().insert(42);
        let cloned = s.lc();
        assert_eq!(s, cloned);
    }

    #[test]
    fn list_implements_lc_clone() {
        let l: rpds::List<i32> = rpds::List::new().push_front(1).push_front(2);
        let cloned = l.lc();
        assert_eq!(l, cloned);
    }

    #[test]
    fn queue_implements_lc_clone() {
        let q: rpds::Queue<i32> = rpds::Queue::new().enqueue(1).enqueue(2);
        let cloned = q.lc();
        assert_eq!(q, cloned);
    }

    #[test]
    fn stack_implements_lc_clone() {
        let s: rpds::Stack<i32> = rpds::Stack::new().push(1).push(2);
        let cloned = s.lc();
        assert_eq!(s, cloned);
    }
}
