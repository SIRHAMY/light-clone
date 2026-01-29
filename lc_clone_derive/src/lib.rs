use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(LcClone)]
pub fn derive_lc_clone(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let lc_impl = match &input.data {
        Data::Struct(data_struct) => {
            let lc_body = match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_clones = fields.named.iter().map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        quote! { #ident: lc_clone::LcClone::lc(&self.#ident) }
                    });
                    quote! {
                        Self {
                            #(#field_clones),*
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let field_clones = fields.unnamed.iter().enumerate().map(|(i, _)| {
                        let index = Index::from(i);
                        quote! { lc_clone::LcClone::lc(&self.#index) }
                    });
                    quote! {
                        Self(#(#field_clones),*)
                    }
                }
                Fields::Unit => {
                    quote! { Self }
                }
            };

            quote! {
                impl #impl_generics lc_clone::LcClone for #name #ty_generics #where_clause {
                    fn lc(&self) -> Self {
                        #lc_body
                    }
                }

                impl #impl_generics Clone for #name #ty_generics #where_clause {
                    fn clone(&self) -> Self {
                        lc_clone::LcClone::lc(self)
                    }
                }
            }
        }
        Data::Enum(_) => {
            return syn::Error::new_spanned(
                &input.ident,
                "LcClone derive is not yet supported for enums. Consider wrapping in Arc.",
            )
            .to_compile_error()
            .into();
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(
                &input.ident,
                "LcClone derive is not supported for unions.",
            )
            .to_compile_error()
            .into();
        }
    };

    lc_impl.into()
}
