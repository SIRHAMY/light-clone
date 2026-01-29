use crate::aliases::LcStr;
use std::sync::Arc;

/// Ergonomic conversion trait for creating [`LcStr`] values.
///
/// This trait provides an `into_lc()` method that converts string types into `LcStr`.
/// It mirrors the standard `Into` trait pattern but with a more explicit name that
/// signals the intent: converting to a light-clone-able type.
///
/// # Examples
///
/// ```
/// use lc_clone::IntoLcStr;
///
/// // From a string literal
/// let s = "hello".into_lc();
///
/// // From an owned String
/// let owned = String::from("world");
/// let s2 = owned.into_lc();
/// ```
///
/// # Why not just use `Into<Arc<str>>`?
///
/// While `Arc::from()` works fine, `into_lc()` makes the intent explicit at the call site:
/// you're converting to a light-clone-able string type. This helps with code readability
/// and signals that the resulting value is cheap to clone.
pub trait IntoLcStr {
    /// Converts this value into an [`LcStr`].
    #[must_use]
    fn into_lc(self) -> LcStr;
}

impl IntoLcStr for &str {
    /// Converts a string slice into an [`LcStr`].
    ///
    /// This allocates a new `Arc<str>` containing a copy of the string data.
    ///
    /// # Examples
    ///
    /// ```
    /// use lc_clone::IntoLcStr;
    ///
    /// let s = "hello".into_lc();
    /// assert_eq!(&*s, "hello");
    /// ```
    fn into_lc(self) -> LcStr {
        Arc::from(self)
    }
}

impl IntoLcStr for String {
    /// Converts an owned `String` into an [`LcStr`].
    ///
    /// This consumes the `String` and creates an `Arc<str>` from it.
    ///
    /// # Examples
    ///
    /// ```
    /// use lc_clone::IntoLcStr;
    ///
    /// let owned = String::from("hello");
    /// let s = owned.into_lc();
    /// assert_eq!(&*s, "hello");
    /// ```
    fn into_lc(self) -> LcStr {
        Arc::from(self)
    }
}

impl IntoLcStr for &String {
    /// Converts a reference to a `String` into an [`LcStr`].
    ///
    /// This allocates a new `Arc<str>` containing a copy of the string data.
    ///
    /// # Examples
    ///
    /// ```
    /// use lc_clone::IntoLcStr;
    ///
    /// let owned = String::from("hello");
    /// let s = (&owned).into_lc();
    /// assert_eq!(&*s, "hello");
    /// ```
    fn into_lc(self) -> LcStr {
        Arc::from(self.as_str())
    }
}

impl IntoLcStr for LcStr {
    /// Returns the [`LcStr`] unchanged.
    ///
    /// This makes `IntoLcStr` idempotent, allowing generic code that accepts
    /// `impl IntoLcStr` to work with existing `LcStr` values.
    ///
    /// # Examples
    ///
    /// ```
    /// use lc_clone::{IntoLcStr, LcStr};
    /// use std::sync::Arc;
    ///
    /// fn takes_into_lc(s: impl IntoLcStr) -> LcStr {
    ///     s.into_lc()
    /// }
    ///
    /// let s: LcStr = Arc::from("hello");
    /// let s2 = takes_into_lc(s.clone());
    /// assert_eq!(&*s2, "hello");
    /// ```
    fn into_lc(self) -> LcStr {
        self
    }
}
