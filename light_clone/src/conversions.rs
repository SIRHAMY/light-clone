use crate::aliases::LightStr;
use std::sync::Arc;

/// Ergonomic conversion trait for creating [`LightStr`] values.
///
/// This trait provides an `into_light_str()` method that converts string types into `LightStr`.
/// It mirrors the standard `Into` trait pattern but with a more explicit name that
/// signals the intent: converting to a light-clone-able type.
///
/// # Examples
///
/// ```
/// use light_clone::IntoLightStr;
///
/// // From a string literal
/// let s = "hello".into_light_str();
///
/// // From an owned String
/// let owned = String::from("world");
/// let s2 = owned.into_light_str();
/// ```
///
/// # Why not just use `Into<Arc<str>>`?
///
/// While `Arc::from()` works fine, `into_light_str()` makes the intent explicit at the call site:
/// you're converting to a light-clone-able string type. This helps with code readability
/// and signals that the resulting value is cheap to clone.
pub trait IntoLightStr {
    /// Converts this value into a [`LightStr`].
    #[must_use]
    fn into_light_str(self) -> LightStr;
}

impl IntoLightStr for &str {
    /// Converts a string slice into a [`LightStr`].
    ///
    /// This allocates a new `Arc<str>` containing a copy of the string data.
    ///
    /// # Examples
    ///
    /// ```
    /// use light_clone::IntoLightStr;
    ///
    /// let s = "hello".into_light_str();
    /// assert_eq!(&*s, "hello");
    /// ```
    fn into_light_str(self) -> LightStr {
        Arc::from(self)
    }
}

impl IntoLightStr for String {
    /// Converts an owned `String` into a [`LightStr`].
    ///
    /// This consumes the `String` and creates an `Arc<str>` from it.
    ///
    /// # Examples
    ///
    /// ```
    /// use light_clone::IntoLightStr;
    ///
    /// let owned = String::from("hello");
    /// let s = owned.into_light_str();
    /// assert_eq!(&*s, "hello");
    /// ```
    fn into_light_str(self) -> LightStr {
        Arc::from(self)
    }
}

impl IntoLightStr for &String {
    /// Converts a reference to a `String` into a [`LightStr`].
    ///
    /// This allocates a new `Arc<str>` containing a copy of the string data.
    ///
    /// # Examples
    ///
    /// ```
    /// use light_clone::IntoLightStr;
    ///
    /// let owned = String::from("hello");
    /// let s = (&owned).into_light_str();
    /// assert_eq!(&*s, "hello");
    /// ```
    fn into_light_str(self) -> LightStr {
        Arc::from(self.as_str())
    }
}

impl IntoLightStr for LightStr {
    /// Returns the [`LightStr`] unchanged.
    ///
    /// This makes `IntoLightStr` idempotent, allowing generic code that accepts
    /// `impl IntoLightStr` to work with existing `LightStr` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use light_clone::{IntoLightStr, LightStr};
    /// use std::sync::Arc;
    ///
    /// fn takes_into_light_str(s: impl IntoLightStr) -> LightStr {
    ///     s.into_light_str()
    /// }
    ///
    /// let s: LightStr = Arc::from("hello");
    /// let s2 = takes_into_light_str(s.clone());
    /// assert_eq!(&*s2, "hello");
    /// ```
    fn into_light_str(self) -> LightStr {
        self
    }
}
