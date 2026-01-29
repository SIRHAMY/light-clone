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
