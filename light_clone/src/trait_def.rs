/// Marker trait for types that are O(1) to clone.
///
/// Cloning involves only:
/// - Atomic refcount increments (Arc)
/// - Non-atomic refcount increments (Rc)
/// - Bitwise copy (Copy types)
///
/// Types implementing this trait must guarantee that calling `.light_clone()` or `.clone()`
/// is a constant-time operation regardless of the size of any data the type may reference.
///
/// # Usage
///
/// Prefer `.light_clone()` over `.clone()` when you want to make the cheap clone explicit:
///
/// ```
/// use light_clone::LightClone;
///
/// let x: i32 = 42;
/// let y = x.light_clone();  // Explicit: this is a light clone
/// assert_eq!(x, y);
/// ```
///
/// A shorthand `.lc()` method is also available:
///
/// ```
/// use light_clone::LightClone;
///
/// let x: i32 = 42;
/// let y = x.lc();  // Shorthand for light_clone()
/// assert_eq!(x, y);
/// ```
///
/// # Derive Macro
///
/// Use `#[derive(LightClone)]` on structs to get compile-time enforcement that all fields
/// are cheap to clone:
///
/// ```ignore
/// use light_clone::LightClone;
/// use std::sync::Arc;
///
/// #[derive(LightClone)]
/// struct Person {
///     id: i64,
///     name: Arc<str>,
/// }
/// ```
pub trait LightClone: Clone {
    /// Returns a light clone of the value.
    ///
    /// This operation is guaranteed to be O(1), involving only:
    /// - Atomic or non-atomic refcount increments
    /// - Bitwise copies for Copy types
    fn light_clone(&self) -> Self;

    /// Shorthand for [`light_clone()`](LightClone::light_clone).
    #[inline]
    fn lc(&self) -> Self {
        self.light_clone()
    }
}
