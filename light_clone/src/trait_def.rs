/// Marker trait for types that are O(1) to clone.
///
/// `LightClone` is a marker trait that asserts a type's `Clone` implementation is cheap.
/// Cloning involves only:
/// - Atomic refcount increments (Arc)
/// - Non-atomic refcount increments (Rc)
/// - Bitwise copy (Copy types)
/// - Persistent data structure cloning (im, imbl, rpds)
///
/// This trait is similar to Facebook's [Dupe](https://github.com/facebookincubator/gazebo)
/// crate but provides a `.light_clone()` method that delegates to `clone()`.
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
/// Use `#[derive(Clone, LightClone)]` on structs to get compile-time enforcement that all
/// fields are cheap to clone. The derive macro requires `Clone` to be implemented separately
/// (either via derive or manual impl) and generates a `LightClone` impl with bounds that
/// ensure all fields implement `LightClone`:
///
/// ```ignore
/// use light_clone::LightClone;
/// use std::sync::Arc;
///
/// #[derive(Clone, LightClone)]
/// struct Person {
///     id: i64,
///     name: Arc<str>,
/// }
/// ```
///
/// The compile-time enforcement comes from the generated bounds - if any field doesn't
/// implement `LightClone`, compilation will fail.
pub trait LightClone: Clone {
    /// Returns a light clone of the value.
    ///
    /// This operation is guaranteed to be O(1), involving only:
    /// - Atomic or non-atomic refcount increments
    /// - Bitwise copies for Copy types
    ///
    /// The default implementation simply calls `clone()`, which is correct for all
    /// `LightClone` types since their `Clone` is guaranteed to be O(1).
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }

    /// Shorthand for [`light_clone()`](LightClone::light_clone).
    #[inline]
    fn lc(&self) -> Self {
        self.light_clone()
    }
}
