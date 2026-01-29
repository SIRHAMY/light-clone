use crate::LcClone;
use std::rc::Rc;
use std::sync::Arc;

impl<T: ?Sized> LcClone for Arc<T> {
    #[inline]
    fn lc(&self) -> Self {
        Arc::clone(self)
    }
}

impl<T: ?Sized> LcClone for Rc<T> {
    #[inline]
    fn lc(&self) -> Self {
        Rc::clone(self)
    }
}
