use crate::LightClone;
use std::rc::Rc;
use std::sync::Arc;

impl<T: ?Sized> LightClone for Arc<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        Arc::clone(self)
    }
}

impl<T: ?Sized> LightClone for Rc<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        Rc::clone(self)
    }
}

impl<T: ?Sized> LightClone for std::sync::Weak<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        std::sync::Weak::clone(self)
    }
}

impl<T: ?Sized> LightClone for std::rc::Weak<T> {
    #[inline]
    fn light_clone(&self) -> Self {
        std::rc::Weak::clone(self)
    }
}
