use crate::LightClone;

impl LightClone for uuid::Uuid {
    #[inline]
    fn light_clone(&self) -> Self {
        *self
    }
}
