/// Marker trait for types that are O(1) to clone.
///
/// Cloning involves only:
/// - Atomic refcount increments (Arc)
/// - Non-atomic refcount increments (Rc)
/// - Bitwise copy (Copy types)
///
/// Types implementing this trait must guarantee that calling `.lc()` or `.clone()`
/// is a constant-time operation regardless of the size of any data the type may reference.
///
/// # Usage
///
/// Prefer `.lc()` over `.clone()` when you want to make the cheap clone explicit at the call site:
///
/// ```
/// use lc_clone::LcClone;
///
/// let x: i32 = 42;
/// let y = x.lc();  // Explicit: this is a light clone
/// assert_eq!(x, y);
/// ```
///
/// # Derive Macro
///
/// Use `#[derive(LcClone)]` on structs to get compile-time enforcement that all fields are cheap to clone:
///
/// ```ignore
/// use lc_clone::LcClone;
/// use std::sync::Arc;
///
/// #[derive(LcClone)]
/// struct Person {
///     id: i64,
///     name: Arc<str>,
/// }
/// ```
pub trait LcClone: Clone {
    /// Returns a light clone of the value.
    ///
    /// This operation is guaranteed to be O(1), involving only:
    /// - Atomic or non-atomic refcount increments
    /// - Bitwise copies for Copy types
    fn lc(&self) -> Self;
}
