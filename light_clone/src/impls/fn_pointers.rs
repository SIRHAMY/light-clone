use crate::LightClone;

macro_rules! impl_light_clone_for_fn {
    () => {
        impl<Ret> LightClone for fn() -> Ret {}
    };
    ($($arg:ident),+) => {
        impl<Ret, $($arg),+> LightClone for fn($($arg),+) -> Ret {}
    };
}

impl_light_clone_for_fn!();
impl_light_clone_for_fn!(A);
impl_light_clone_for_fn!(A, B);
impl_light_clone_for_fn!(A, B, C);
impl_light_clone_for_fn!(A, B, C, D);
impl_light_clone_for_fn!(A, B, C, D, E);
impl_light_clone_for_fn!(A, B, C, D, E, F);
impl_light_clone_for_fn!(A, B, C, D, E, F, G);
impl_light_clone_for_fn!(A, B, C, D, E, F, G, H);
impl_light_clone_for_fn!(A, B, C, D, E, F, G, H, I);
impl_light_clone_for_fn!(A, B, C, D, E, F, G, H, I, J);
impl_light_clone_for_fn!(A, B, C, D, E, F, G, H, I, J, K);
impl_light_clone_for_fn!(A, B, C, D, E, F, G, H, I, J, K, L);
