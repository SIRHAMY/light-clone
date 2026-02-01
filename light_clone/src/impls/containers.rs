use crate::LightClone;
use std::cell::Cell;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ops::Bound;
use std::pin::Pin;
use std::ptr::NonNull;
use std::task::Poll;

impl LightClone for () {}

impl<T> LightClone for PhantomData<T> {}

impl<T: LightClone> LightClone for Option<T> {}

impl<T: LightClone, E: Clone> LightClone for Result<T, E> {}

impl<T: LightClone + Copy, const N: usize> LightClone for [T; N] {}

// Wrapper types
impl<T: LightClone> LightClone for Bound<T> {}

impl<T: LightClone> LightClone for Pin<T> {}

impl<T: ?Sized> LightClone for NonNull<T> {}

impl<T: LightClone> LightClone for Poll<T> {}

impl<T: LightClone + Copy> LightClone for Cell<T> {}

impl<T: LightClone> LightClone for ManuallyDrop<T> {}
