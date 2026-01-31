//! LightClone implementation for the `bytes` crate.
//!
//! This implementation is behind the `bytes` feature flag.
//!
//! `Bytes` is a reference-counted byte buffer that clones in O(1) time
//! by incrementing an atomic reference count.
//!
//! Note: `BytesMut` is NOT implemented as it is uniquely owned and
//! cloning requires copying the underlying data.

use crate::LightClone;

impl LightClone for bytes::Bytes {
    #[inline]
    fn light_clone(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn bytes_implements_light_clone() {
        let data = Bytes::from_static(b"hello world");
        let cloned = data.light_clone();
        assert_eq!(data, cloned);
    }

    #[test]
    fn bytes_from_vec_implements_light_clone() {
        let data = Bytes::from(vec![1, 2, 3, 4, 5]);
        let cloned = data.light_clone();
        assert_eq!(data, cloned);
    }
}
