use crate::LightClone;

impl<T: ?Sized> LightClone for std::sync::Arc<T> {}

impl<T: ?Sized> LightClone for std::rc::Rc<T> {}

impl<T: ?Sized> LightClone for std::sync::Weak<T> {}

impl<T: ?Sized> LightClone for std::rc::Weak<T> {}
