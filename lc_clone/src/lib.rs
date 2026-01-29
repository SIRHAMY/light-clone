mod impls;
mod trait_def;

// Re-export the trait
pub use trait_def::LcClone;

// Re-export the derive macro
pub use lc_clone_derive::LcClone;
