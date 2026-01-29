use crate::LightClone;
use std::marker::PhantomData;

impl LightClone for () {
    #[inline]
    fn light_clone(&self) -> Self {}
}

impl<T> LightClone for PhantomData<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        PhantomData
    }
}

impl<T: LightClone> LightClone for Option<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        self.as_ref().map(|value| value.light_clone())
    }
}

impl<T: LightClone, E: Clone> LightClone for Result<T, E> {
    #[inline]
    fn light_clone(&self) -> Self {
        match self {
            Ok(value) => Ok(value.light_clone()),
            Err(err) => Err(err.clone()),
        }
    }
}
