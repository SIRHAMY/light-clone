use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

/// Derive macro for `LightClone` trait.
///
/// Also available as `#[derive(LcClone)]` for backwards compatibility.
#[proc_macro_derive(LightClone)]
pub fn derive_light_clone(input: TokenStream) -> TokenStream {
    derive_light_clone_impl(input)
}

/// Legacy alias for `#[derive(LightClone)]`.
#[proc_macro_derive(LcClone)]
pub fn derive_lc_clone(input: TokenStream) -> TokenStream {
    derive_light_clone_impl(input)
}

fn derive_light_clone_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let light_clone_impl = match &input.data {
        Data::Struct(data_struct) => {
            let light_clone_body = match &data_struct.fields {
                Fields::Named(fields) => {
                    let field_clones = fields.named.iter().map(|f| {
                        let ident = f.ident.as_ref().unwrap();
                        quote! { #ident: light_clone::LightClone::light_clone(&self.#ident) }
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
                        quote! { light_clone::LightClone::light_clone(&self.#index) }
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
                impl #impl_generics light_clone::LightClone for #name #ty_generics #where_clause {
                    fn light_clone(&self) -> Self {
                        #light_clone_body
                    }
                }

                impl #impl_generics Clone for #name #ty_generics #where_clause {
                    fn clone(&self) -> Self {
                        light_clone::LightClone::light_clone(self)
                    }
                }
            }
        }
        Data::Enum(_) => {
            return syn::Error::new_spanned(
                &input.ident,
                "LightClone derive is not yet supported for enums. Consider wrapping in Arc.",
            )
            .to_compile_error()
            .into();
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(
                &input.ident,
                "LightClone derive is not supported for unions.",
            )
            .to_compile_error()
            .into();
        }
    };

    light_clone_impl.into()
}
