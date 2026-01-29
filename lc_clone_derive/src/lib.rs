use proc_macro::TokenStream;

#[proc_macro_derive(LcClone)]
pub fn derive_lc_clone(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
