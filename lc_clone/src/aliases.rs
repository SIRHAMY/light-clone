use std::sync::Arc;

/// An immutable, cheaply-cloneable string.
///
/// `LcStr` is a type alias for `Arc<str>`, providing a string type that is O(1) to clone.
/// Unlike `String`, cloning an `LcStr` only bumps a reference count—it never allocates
/// or copies string data.
///
/// # Examples
///
/// ```
/// use lc_clone::{LcStr, IntoLcStr};
///
/// // Create from a string literal
/// let name: LcStr = "Alice".into_lc();
///
/// // Clone is O(1) - just bumps refcount
/// let name_copy = name.clone();
/// assert_eq!(name, name_copy);
/// ```
///
/// # Underlying Type
///
/// `LcStr` is defined as `Arc<str>`. You can use either type interchangeably:
///
/// ```
/// use std::sync::Arc;
/// use lc_clone::LcStr;
///
/// fn accepts_arc_str(s: &Arc<str>) {}
///
/// let name: LcStr = Arc::from("Bob");
/// accepts_arc_str(&name);
/// ```
pub type LcStr = Arc<str>;

/// An immutable, cheaply-cloneable list.
///
/// `LcList<T>` is a type alias for `im::Vector<T>`, providing a persistent vector that is O(1)
/// to clone. Unlike `Vec<T>`, cloning an `LcList` shares structure—it never deep-copies elements.
///
/// # Examples
///
/// ```ignore
/// use lc_clone::LcList;
///
/// let list: LcList<i32> = im::vector![1, 2, 3];
/// let list_copy = list.clone(); // O(1) - shares structure
/// assert_eq!(list, list_copy);
/// ```
///
/// # Underlying Type
///
/// `LcList<T>` is defined as `im::Vector<T>`. Requires the `im` feature.
#[cfg(feature = "im")]
pub type LcList<T> = im::Vector<T>;

/// An immutable, cheaply-cloneable hash map.
///
/// `LcMap<K, V>` is a type alias for `im::HashMap<K, V>`, providing a persistent hash map
/// that is O(1) to clone.
///
/// # Examples
///
/// ```ignore
/// use lc_clone::LcMap;
///
/// let mut map: LcMap<&str, i32> = im::HashMap::new();
/// map.insert("key", 42);
/// let map_copy = map.clone(); // O(1) - shares structure
/// ```
///
/// # Underlying Type
///
/// `LcMap<K, V>` is defined as `im::HashMap<K, V>`. Requires the `im` feature.
#[cfg(feature = "im")]
pub type LcMap<K, V> = im::HashMap<K, V>;

/// An immutable, cheaply-cloneable hash set.
///
/// `LcSet<T>` is a type alias for `im::HashSet<T>`, providing a persistent hash set
/// that is O(1) to clone.
///
/// # Examples
///
/// ```ignore
/// use lc_clone::LcSet;
///
/// let mut set: LcSet<i32> = im::HashSet::new();
/// set.insert(42);
/// let set_copy = set.clone(); // O(1) - shares structure
/// ```
///
/// # Underlying Type
///
/// `LcSet<T>` is defined as `im::HashSet<T>`. Requires the `im` feature.
#[cfg(feature = "im")]
pub type LcSet<T> = im::HashSet<T>;

/// An immutable, cheaply-cloneable ordered map.
///
/// `LcOrdMap<K, V>` is a type alias for `im::OrdMap<K, V>`, providing a persistent
/// ordered map that is O(1) to clone.
///
/// # Examples
///
/// ```ignore
/// use lc_clone::LcOrdMap;
///
/// let mut map: LcOrdMap<&str, i32> = im::OrdMap::new();
/// map.insert("key", 42);
/// let map_copy = map.clone(); // O(1) - shares structure
/// ```
///
/// # Underlying Type
///
/// `LcOrdMap<K, V>` is defined as `im::OrdMap<K, V>`. Requires the `im` feature.
#[cfg(feature = "im")]
pub type LcOrdMap<K, V> = im::OrdMap<K, V>;
