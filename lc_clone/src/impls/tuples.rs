use crate::LcClone;

macro_rules! impl_lc_clone_for_tuple {
    ($($name:ident),+) => {
        impl<$($name: LcClone),+> LcClone for ($($name,)+) {
            #[inline]
            fn lc(&self) -> Self {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                ($($name.lc(),)+)
            }
        }
    };
}

impl_lc_clone_for_tuple!(A);
impl_lc_clone_for_tuple!(A, B);
impl_lc_clone_for_tuple!(A, B, C);
impl_lc_clone_for_tuple!(A, B, C, D);
impl_lc_clone_for_tuple!(A, B, C, D, E);
impl_lc_clone_for_tuple!(A, B, C, D, E, F);
impl_lc_clone_for_tuple!(A, B, C, D, E, F, G);
impl_lc_clone_for_tuple!(A, B, C, D, E, F, G, H);
impl_lc_clone_for_tuple!(A, B, C, D, E, F, G, H, I);
impl_lc_clone_for_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_lc_clone_for_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_lc_clone_for_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
