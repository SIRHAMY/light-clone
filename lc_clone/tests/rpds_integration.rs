//! Integration tests for the `rpds` feature flag.

#![cfg(feature = "rpds")]

use lc_clone::LcClone;

#[test]
fn rpds_vector_implements_lc_clone() {
    let v: rpds::Vector<i32> = rpds::Vector::new().push_back(1).push_back(2).push_back(3);
    let cloned = v.lc();
    assert_eq!(v, cloned);
    assert_eq!(v.len(), 3);
}

#[test]
fn rpds_hash_trie_map_implements_lc_clone() {
    let m: rpds::HashTrieMap<&str, i32> =
        rpds::HashTrieMap::new().insert("one", 1).insert("two", 2);
    let cloned = m.lc();
    assert_eq!(m, cloned);
    assert_eq!(m.get("one"), Some(&1));
}

#[test]
fn rpds_red_black_tree_map_implements_lc_clone() {
    let m: rpds::RedBlackTreeMap<&str, i32> = rpds::RedBlackTreeMap::new()
        .insert("apple", 1)
        .insert("banana", 2);
    let cloned = m.lc();
    assert_eq!(m, cloned);
    assert_eq!(m.get("apple"), Some(&1));
}

#[test]
fn rpds_hash_trie_set_implements_lc_clone() {
    let s: rpds::HashTrieSet<i32> = rpds::HashTrieSet::new().insert(1).insert(2).insert(3);
    let cloned = s.lc();
    assert_eq!(s, cloned);
    assert!(s.contains(&1));
}

#[test]
fn rpds_red_black_tree_set_implements_lc_clone() {
    let s: rpds::RedBlackTreeSet<i32> = rpds::RedBlackTreeSet::new().insert(3).insert(1).insert(2);
    let cloned = s.lc();
    assert_eq!(s, cloned);

    // Elements should be in sorted order
    let elements: Vec<_> = cloned.iter().copied().collect();
    assert_eq!(elements, vec![1, 2, 3]);
}

#[test]
fn rpds_list_implements_lc_clone() {
    let l: rpds::List<i32> = rpds::List::new().push_front(3).push_front(2).push_front(1);
    let cloned = l.lc();
    assert_eq!(l, cloned);
    assert_eq!(l.first(), Some(&1));
}

#[test]
fn rpds_queue_implements_lc_clone() {
    let q: rpds::Queue<i32> = rpds::Queue::new().enqueue(1).enqueue(2).enqueue(3);
    let cloned = q.lc();
    assert_eq!(q, cloned);
    assert_eq!(q.peek(), Some(&1));
}

#[test]
fn rpds_stack_implements_lc_clone() {
    let s: rpds::Stack<i32> = rpds::Stack::new().push(1).push(2).push(3);
    let cloned = s.lc();
    assert_eq!(s, cloned);
    assert_eq!(s.peek(), Some(&3));
}

#[test]
fn rpds_vector_clone_shares_structure() {
    let v: rpds::Vector<i32> = rpds::Vector::new().push_back(1).push_back(2).push_back(3);
    let cloned = v.lc();

    // Both should be equal
    assert_eq!(v, cloned);

    // Modifying the clone shouldn't affect the original (structural sharing)
    let modified = cloned.push_back(4);
    assert_eq!(v.len(), 3);
    assert_eq!(modified.len(), 4);
}

#[test]
fn rpds_hash_trie_map_operations() {
    let m: rpds::HashTrieMap<String, i32> = rpds::HashTrieMap::new()
        .insert("key1".to_string(), 100)
        .insert("key2".to_string(), 200);

    let cloned = m.lc();

    // Both should have the same values
    assert_eq!(m.get("key1"), cloned.get("key1"));
    assert_eq!(m.get("key2"), cloned.get("key2"));
}
