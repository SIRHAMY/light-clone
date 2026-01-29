use crate::LcClone;
use std::marker::PhantomData;

impl LcClone for () {
    #[inline]
    fn lc(&self) -> Self {}
}

impl<T> LcClone for PhantomData<T> {
    #[inline]
    fn lc(&self) -> Self {
        PhantomData
    }
}

impl<T: LcClone> LcClone for Option<T> {
    #[inline]
    fn lc(&self) -> Self {
        self.as_ref().map(|value| value.lc())
    }
}

impl<T: LcClone, E: Clone> LcClone for Result<T, E> {
    #[inline]
    fn lc(&self) -> Self {
        match self {
            Ok(value) => Ok(value.lc()),
            Err(err) => Err(err.clone()),
        }
    }
}
