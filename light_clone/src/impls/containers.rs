use crate::LightClone;
use std::marker::PhantomData;

impl LightClone for () {}

impl<T> LightClone for PhantomData<T> {}

impl<T: LightClone> LightClone for Option<T> {}

impl<T: LightClone, E: Clone> LightClone for Result<T, E> {}

impl<T: LightClone + Copy, const N: usize> LightClone for [T; N] {}
