use crate::LightClone;

macro_rules! impl_light_clone_for_tuple {
    ($($name:ident),+) => {
        impl<$($name: LightClone),+> LightClone for ($($name,)+) {
            #[inline]
            fn light_clone(&self) -> Self {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                ($($name.light_clone(),)+)
            }
        }
    };
}

impl_light_clone_for_tuple!(A);
impl_light_clone_for_tuple!(A, B);
impl_light_clone_for_tuple!(A, B, C);
impl_light_clone_for_tuple!(A, B, C, D);
impl_light_clone_for_tuple!(A, B, C, D, E);
impl_light_clone_for_tuple!(A, B, C, D, E, F);
impl_light_clone_for_tuple!(A, B, C, D, E, F, G);
impl_light_clone_for_tuple!(A, B, C, D, E, F, G, H);
impl_light_clone_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_light_clone_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_light_clone_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_light_clone_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
