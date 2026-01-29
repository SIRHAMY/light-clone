mod aliases;
mod conversions;
mod impls;
mod trait_def;

// Re-export the trait
pub use trait_def::LightClone;

// Re-export the derive macro
pub use light_clone_derive::LightClone;

// Re-export type aliases
pub use aliases::LightStr;

// Re-export conversion traits
pub use conversions::IntoLightStr;

// Legacy aliases for backwards compatibility
#[doc(hidden)]
pub use aliases::LightStr as LcStr;
#[doc(hidden)]
pub use conversions::IntoLightStr as IntoLcStr;
#[doc(hidden)]
pub use trait_def::LightClone as LcClone;
