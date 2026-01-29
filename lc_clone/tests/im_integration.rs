//! Integration tests for the `im` feature flag.

#![cfg(feature = "im")]

use lc_clone::{LcClone, LcList, LcMap, LcOrdMap, LcSet};

#[test]
fn lc_list_works_as_im_vector() {
    let list: LcList<i32> = im::vector![1, 2, 3];
    let cloned = list.lc();
    assert_eq!(list, cloned);
    assert_eq!(list.len(), 3);
}

#[test]
fn lc_map_works_as_im_hash_map() {
    let mut map: LcMap<&str, i32> = im::HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    let cloned = map.lc();
    assert_eq!(map, cloned);
    assert_eq!(map.get("one"), Some(&1));
}

#[test]
fn lc_set_works_as_im_hash_set() {
    let mut set: LcSet<i32> = im::HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let cloned = set.lc();
    assert_eq!(set, cloned);
    assert!(set.contains(&1));
}

#[test]
fn lc_ord_map_works_as_im_ord_map() {
    let mut map: LcOrdMap<&str, i32> = im::OrdMap::new();
    map.insert("apple", 1);
    map.insert("banana", 2);
    let cloned = map.lc();
    assert_eq!(map, cloned);
    assert_eq!(map.get("apple"), Some(&1));
}

#[test]
fn im_vector_clone_shares_structure() {
    let list: LcList<i32> = im::vector![1, 2, 3, 4, 5];
    let cloned = list.lc();
    // Both should be equal and usable independently
    assert_eq!(list, cloned);

    // Modifying the clone shouldn't affect the original (structural sharing)
    let mut modified = cloned;
    modified.push_back(6);
    assert_eq!(list.len(), 5);
    assert_eq!(modified.len(), 6);
}

#[test]
fn im_hash_map_operations() {
    let mut map: LcMap<String, i32> = im::HashMap::new();
    map.insert("key1".to_string(), 100);
    map.insert("key2".to_string(), 200);

    let cloned = map.lc();

    // Both should have the same values
    assert_eq!(map.get("key1"), cloned.get("key1"));
    assert_eq!(map.get("key2"), cloned.get("key2"));
}

#[test]
fn im_ord_map_maintains_order() {
    let mut map: LcOrdMap<i32, &str> = im::OrdMap::new();
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(2, "two");

    let cloned = map.lc();

    // Keys should be in sorted order
    let keys: Vec<_> = cloned.keys().copied().collect();
    assert_eq!(keys, vec![1, 2, 3]);
}

#[test]
fn im_ord_set_operations() {
    let mut set: im::OrdSet<i32> = im::OrdSet::new();
    set.insert(3);
    set.insert(1);
    set.insert(2);

    let cloned = set.lc();

    // Elements should be in sorted order
    let elements: Vec<_> = cloned.iter().copied().collect();
    assert_eq!(elements, vec![1, 2, 3]);
}

#[derive(lc_clone::LcClone)]
struct PersonWithList {
    id: i32,
    name: lc_clone::LcStr,
    tags: LcList<lc_clone::LcStr>,
}

#[test]
fn struct_with_lc_list_field_compiles_and_clones() {
    use lc_clone::IntoLcStr;

    let person = PersonWithList {
        id: 42,
        name: "Alice".into_lc(),
        tags: im::vector!["developer".into_lc(), "rust".into_lc()],
    };

    let cloned = person.lc();

    assert_eq!(cloned.id, 42);
    assert_eq!(&*cloned.name, "Alice");
    assert_eq!(cloned.tags.len(), 2);
}
