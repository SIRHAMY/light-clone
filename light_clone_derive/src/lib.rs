use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident, Index};

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

/// Generates clone expressions for named struct fields accessed via `self.field`.
fn generate_named_struct_clones(fields: &FieldsNamed) -> TokenStream2 {
    let field_clones = fields.named.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        quote! { #ident: light_clone::LightClone::light_clone(&self.#ident) }
    });
    quote! {
        Self {
            #(#field_clones),*
        }
    }
}

/// Generates clone expressions for unnamed struct fields accessed via `self.0`, `self.1`, etc.
fn generate_unnamed_struct_clones(fields: &FieldsUnnamed) -> TokenStream2 {
    let field_clones = fields.unnamed.iter().enumerate().map(|(index, _field)| {
        let index = Index::from(index);
        quote! { light_clone::LightClone::light_clone(&self.#index) }
    });
    quote! {
        Self(#(#field_clones),*)
    }
}

/// Generates a match arm for an enum variant with named fields.
fn generate_named_variant_arm(variant_ident: &Ident, fields: &FieldsNamed) -> TokenStream2 {
    let field_names: Vec<_> = fields
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap())
        .collect();
    let field_clones = field_names.iter().map(|name| {
        quote! { #name: light_clone::LightClone::light_clone(#name) }
    });
    quote! {
        Self::#variant_ident { #(#field_names),* } => Self::#variant_ident { #(#field_clones),* }
    }
}

/// Generates a match arm for an enum variant with unnamed (tuple) fields.
fn generate_unnamed_variant_arm(variant_ident: &Ident, fields: &FieldsUnnamed) -> TokenStream2 {
    let bindings: Vec<_> = (0..fields.unnamed.len())
        .map(|index| format_ident!("__field_{}", index))
        .collect();
    let clones = bindings.iter().map(|binding| {
        quote! { light_clone::LightClone::light_clone(#binding) }
    });
    quote! {
        Self::#variant_ident(#(#bindings),*) => Self::#variant_ident(#(#clones),*)
    }
}

fn derive_light_clone_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let light_clone_impl = match &input.data {
        Data::Struct(data_struct) => {
            let light_clone_body = match &data_struct.fields {
                Fields::Named(fields) => generate_named_struct_clones(fields),
                Fields::Unnamed(fields) => generate_unnamed_struct_clones(fields),
                Fields::Unit => quote! { Self },
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
        Data::Enum(data_enum) => {
            // Handle empty enums specially - they're uninhabited types
            if data_enum.variants.is_empty() {
                return quote! {
                    impl #impl_generics light_clone::LightClone for #name #ty_generics #where_clause {
                        fn light_clone(&self) -> Self {
                            match *self {}
                        }
                    }

                    impl #impl_generics Clone for #name #ty_generics #where_clause {
                        fn clone(&self) -> Self {
                            match *self {}
                        }
                    }
                }
                .into();
            }

            let match_arms = data_enum.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;

                match &variant.fields {
                    Fields::Unit => {
                        quote! { Self::#variant_ident => Self::#variant_ident }
                    }
                    Fields::Unnamed(fields) => generate_unnamed_variant_arm(variant_ident, fields),
                    Fields::Named(fields) => generate_named_variant_arm(variant_ident, fields),
                }
            });

            quote! {
                impl #impl_generics light_clone::LightClone for #name #ty_generics #where_clause {
                    fn light_clone(&self) -> Self {
                        match self {
                            #(#match_arms),*
                        }
                    }
                }

                impl #impl_generics Clone for #name #ty_generics #where_clause {
                    fn clone(&self) -> Self {
                        light_clone::LightClone::light_clone(self)
                    }
                }
            }
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
