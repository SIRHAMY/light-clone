mod containers;
mod fn_pointers;
mod primitives;
mod smart_pointers;
mod tuples;

#[cfg(feature = "im")]
mod im_collections;

#[cfg(feature = "imbl")]
mod imbl_collections;

#[cfg(feature = "rpds")]
mod rpds_collections;

#[cfg(feature = "uuid")]
mod uuid;

#[cfg(feature = "chrono")]
mod chrono_types;

#[cfg(feature = "bytes")]
mod bytes_types;

#[cfg(feature = "rust_decimal")]
mod rust_decimal_types;

#[cfg(feature = "ordered-float")]
mod ordered_float_types;

#[cfg(feature = "smol_str")]
mod smol_str_types;

#[cfg(feature = "time")]
mod time_types;
