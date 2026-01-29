use std::sync::Arc;

/// An immutable, cheaply-cloneable string.
///
/// `LightStr` is a type alias for `Arc<str>`, providing a string type that is O(1) to clone.
/// Unlike `String`, cloning a `LightStr` only bumps a reference count—it never allocates
/// or copies string data.
///
/// # Examples
///
/// ```
/// use light_clone::{LightStr, IntoLightStr};
///
/// // Create from a string literal
/// let name: LightStr = "Alice".into_light_str();
///
/// // Clone is O(1) - just bumps refcount
/// let name_copy = name.clone();
/// assert_eq!(name, name_copy);
/// ```
///
/// # Underlying Type
///
/// `LightStr` is defined as `Arc<str>`. You can use either type interchangeably:
///
/// ```
/// use std::sync::Arc;
/// use light_clone::LightStr;
///
/// fn accepts_arc_str(s: &Arc<str>) {}
///
/// let name: LightStr = Arc::from("Bob");
/// accepts_arc_str(&name);
/// ```
pub type LightStr = Arc<str>;
