use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, GenericParam, WhereClause, WherePredicate};

/// Derive macro for `LightClone` trait.
///
/// This macro generates a `LightClone` implementation for structs and enums.
/// The generated impl uses the default `light_clone()` method (which calls `clone()`),
/// but adds `LightClone` bounds for all field types to ensure compile-time enforcement
/// that all fields are O(1) to clone.
///
/// # Usage
///
/// ```ignore
/// use light_clone::LightClone;
///
/// #[derive(Clone, LightClone)]
/// struct Person {
///     id: i64,
///     name: Arc<str>,
/// }
/// ```
///
/// **Important:** You must also derive or implement `Clone` separately. The derive macro
/// no longer generates a `Clone` impl - it only generates `LightClone`.
///
/// # How it works
///
/// The macro generates an impl with where clause bounds that require all field types
/// to implement `LightClone`. This provides compile-time enforcement that all fields
/// are cheap to clone, while the actual cloning is delegated to the `Clone` impl.
///
/// For generic types, the macro adds `LightClone` bounds to ensure the generic
/// parameters also satisfy the O(1) clone requirement.
#[proc_macro_derive(LightClone)]
pub fn derive_light_clone(input: TokenStream) -> TokenStream {
    derive_light_clone_impl(input)
}

/// Collects all field types from a struct or enum for generating where clause bounds.
fn collect_field_types(data: &Data) -> Vec<syn::Type> {
    let mut types = Vec::new();

    match data {
        Data::Struct(data_struct) => {
            for field in &data_struct.fields {
                types.push(field.ty.clone());
            }
        }
        Data::Enum(data_enum) => {
            for variant in &data_enum.variants {
                for field in &variant.fields {
                    types.push(field.ty.clone());
                }
            }
        }
        Data::Union(_) => {
            // Unions are handled separately with an error
        }
    }

    types
}

/// Builds the where clause with LightClone bounds for all field types.
fn build_where_clause(
    existing: Option<&WhereClause>,
    field_types: &[syn::Type],
    generics: &syn::Generics,
) -> TokenStream2 {
    // Collect existing predicates
    let mut predicates: Vec<WherePredicate> = existing
        .map(|w| w.predicates.iter().cloned().collect())
        .unwrap_or_default();

    // Get type parameter names for adding LightClone bounds
    let type_params: Vec<_> = generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(type_param) = param {
                Some(type_param.ident.clone())
            } else {
                None
            }
        })
        .collect();

    // Add LightClone bounds for all type parameters
    for type_param in &type_params {
        let predicate: WherePredicate = syn::parse_quote!(#type_param: light_clone::LightClone);
        predicates.push(predicate);
    }

    // Add LightClone bounds for all field types
    for ty in field_types {
        let predicate: WherePredicate = syn::parse_quote!(#ty: light_clone::LightClone);
        predicates.push(predicate);
    }

    if predicates.is_empty() {
        quote! {}
    } else {
        quote! { where #(#predicates),* }
    }
}

fn derive_light_clone_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    // Handle unions with an error
    if let Data::Union(_) = &input.data {
        return syn::Error::new_spanned(
            &input.ident,
            "LightClone derive is not supported for unions.",
        )
        .to_compile_error()
        .into();
    }

    // Collect field types for where clause bounds
    let field_types = collect_field_types(&input.data);

    // Build the where clause with LightClone bounds
    let where_clause =
        build_where_clause(input.generics.where_clause.as_ref(), &field_types, generics);

    // Generate the impl - empty body uses the default implementation
    let light_clone_impl = quote! {
        impl #impl_generics light_clone::LightClone for #name #ty_generics #where_clause {}
    };

    light_clone_impl.into()
}
