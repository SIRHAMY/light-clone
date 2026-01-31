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
