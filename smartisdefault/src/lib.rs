extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[proc_macro_derive(SmartIsDefault, attributes(default, smart_is_default))]
pub fn derive_smart_is_default(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let no_is_default = container_has_flag(&input, "smart_is_default", "no_is_default");

    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut helper_fns = Vec::new();

    match input.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                for field in fields.named {
                    let field_name = field.ident.as_ref().unwrap();

                    // Per-field opt-out: `#[smart_is_default(skip)]` suppresses the
                    // `is_default__<field>` helper for this field only.
                    if field_has_flag(&field, "smart_is_default", "skip") {
                        continue;
                    }

                    let field_ty = &field.ty;
                    let fn_name = format_ident!("is_default__{}", field_name);

                    helper_fns.push(quote! {
                        #[allow(non_snake_case)]
                        fn #fn_name(v: &#field_ty) -> bool {
                            v == &Self::default().#field_name
                        }
                    });
                }
            }
            Fields::Unnamed(fields) => {
                for (field_index, field) in fields.unnamed.into_iter().enumerate() {
                    if field_has_flag(&field, "smart_is_default", "skip") {
                        continue;
                    }

                    let field_ty = &field.ty;
                    let index = Index::from(field_index);
                    let fn_name = format_ident!("is_default__{}", field_index);

                    helper_fns.push(quote! {
                        #[allow(non_snake_case)]
                        fn #fn_name(v: &#field_ty) -> bool {
                            v == &Self::default().#index
                        }
                    });
                }
            }
            Fields::Unit => {}
        },
        Data::Enum(_) => {}
        Data::Union(_) => {}
    }

    let is_default_method = if no_is_default {
        quote! {}
    } else {
        quote! {
            /// Returns `true` if `self` compares equal to `Self::default()`.
            ///
            /// This is a whole-struct check: every field must match its default
            /// value. Only available when the struct
            /// derives [`PartialEq`] and has [`Default::default()`](https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default) implemented.
            ///
            /// Suppress this method with `#[smart_is_default(no_is_default)]` if
            /// you want to provide your own `is_default`.
            fn is_default(&self) -> bool
            where
                Self: PartialEq,
            {
                self == &Self::default()
            }
        }
    };

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #is_default_method

            #( #helper_fns )*
        }
    };
    expanded.into()
}

/// Scans the container attributes for `#[<attr_path>(<flag>)]` and returns
/// `true` if the given flag is present.
fn container_has_flag(input: &DeriveInput, attr_path: &str, flag: &str) -> bool {
    for attr in &input.attrs {
        if !attr.path().is_ident(attr_path) {
            continue;
        }
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(flag) {
                found = true;
            }
            Ok(())
        });
        if found {
            return true;
        }
    }
    false
}

/// Scans a single field's attributes for `#[<attr_path>(<flag>)]` and returns
/// `true` if the given flag is present.
fn field_has_flag(field: &syn::Field, attr_path: &str, flag: &str) -> bool {
    for attr in &field.attrs {
        if !attr.path().is_ident(attr_path) {
            continue;
        }
        let mut found = false;
        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(flag) {
                found = true;
            }
            Ok(())
        });
        if found {
            return true;
        }
    }
    false
}
