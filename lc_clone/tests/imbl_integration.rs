//! Integration tests for the `imbl` feature flag.

#![cfg(feature = "imbl")]

use lc_clone::LcClone;

#[test]
fn imbl_vector_implements_lc_clone() {
    let list: imbl::Vector<i32> = imbl::vector![1, 2, 3];
    let cloned = list.lc();
    assert_eq!(list, cloned);
    assert_eq!(list.len(), 3);
}

#[test]
fn imbl_hash_map_implements_lc_clone() {
    let mut map: imbl::HashMap<&str, i32> = imbl::HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    let cloned = map.lc();
    assert_eq!(map, cloned);
    assert_eq!(map.get("one"), Some(&1));
}

#[test]
fn imbl_hash_set_implements_lc_clone() {
    let mut set: imbl::HashSet<i32> = imbl::HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    let cloned = set.lc();
    assert_eq!(set, cloned);
    assert!(set.contains(&1));
}

#[test]
fn imbl_ord_map_implements_lc_clone() {
    let mut map: imbl::OrdMap<&str, i32> = imbl::OrdMap::new();
    map.insert("apple", 1);
    map.insert("banana", 2);
    let cloned = map.lc();
    assert_eq!(map, cloned);
    assert_eq!(map.get("apple"), Some(&1));
}

#[test]
fn imbl_vector_clone_shares_structure() {
    let list: imbl::Vector<i32> = imbl::vector![1, 2, 3, 4, 5];
    let cloned = list.lc();
    assert_eq!(list, cloned);

    // Modifying the clone shouldn't affect the original (structural sharing)
    let mut modified = cloned;
    modified.push_back(6);
    assert_eq!(list.len(), 5);
    assert_eq!(modified.len(), 6);
}

#[derive(lc_clone::LcClone)]
struct PersonWithImblVector {
    id: i32,
    name: lc_clone::LcStr,
    tags: imbl::Vector<lc_clone::LcStr>,
}

#[test]
fn struct_with_imbl_vector_field_compiles_and_clones() {
    use lc_clone::IntoLcStr;

    let person = PersonWithImblVector {
        id: 42,
        name: "Alice".into_lc(),
        tags: imbl::vector!["developer".into_lc(), "rust".into_lc()],
    };

    let cloned = person.lc();

    assert_eq!(cloned.id, 42);
    assert_eq!(&*cloned.name, "Alice");
    assert_eq!(cloned.tags.len(), 2);
}
