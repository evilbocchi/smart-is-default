extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Index};

#[derive(Clone, Copy)]
enum FieldKind {
    Named,
    Unnamed,
}

struct FieldInfo {
    kind: FieldKind,
    accessor: proc_macro2::TokenStream,
    helper_fn: Option<proc_macro2::Ident>,
    label: Option<syn::LitStr>,
}

/// Generates per-field `is_default__<field>` helpers and an optional
/// `serde::Serialize` impl.
///
/// # Container attributes
///
/// * `#[smart_is_default(no_is_default)]`. Suppress the whole-struct
///   `is_default` method so you can provide your own.
/// * `#[smart_is_default(serde)]`. Auto-generate a `serde::Serialize` impl
///   that skips any field whose value equals `Self::default()`. This is the
///   proc-macro equivalent of writing
///   `#[serde(skip_serializing_if = "StructName::is_default__<field>")]`
///   on every field. Requires `serde` to be in the consuming crate's
///   dependencies.
///
///   - For **named structs**, fields are conditionally serialized using
///     `serialize_struct` with a runtime-computed length, equivalent to
///     `#[serde(skip_serializing_if = "...")]`.
///   - For **tuple structs**, a plain `serialize_tuple_struct` impl is
///     emitted (skipping tuple elements would break deserialization, so the
///     flag is effectively a no-op for skipping but still gives you a
///     `Serialize` impl).
///   - For **unit structs / enums / unions**, no `Serialize` impl is emitted.
///
/// # Field attributes
///
/// * `#[smart_is_default(skip)]`. Suppress the per-field
///   `is_default__<field>` helper. In the auto-generated `Serialize` impl
///   the field is always emitted (no skip check is performed, since there is
///   no helper to consult).
/// * `#[smart_is_default(no_pub)]`. Emit per-field helpers without
///   the `pub` qualifier, keeping them private to the module that defines
///   the type. By default, helpers are `pub` so they can be referenced
///   from other modules (e.g. in `#[serde(skip_serializing_if = "...")]`
///   attributes on a separate wrapper type).
#[proc_macro_derive(SmartIsDefault, attributes(default, smart_is_default))]
pub fn derive_smart_is_default(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let no_is_default = container_has_flag(&input, "smart_is_default", "no_is_default");
    let serde_skip = container_has_flag(&input, "smart_is_default", "serde");
    let no_pub = container_has_flag(&input, "smart_is_default", "no_pub");
    let helper_vis: proc_macro2::TokenStream = if no_pub {
        quote! {}
    } else {
        quote! { pub }
    };

    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut field_infos: Vec<FieldInfo> = Vec::new();
    let mut helper_fns = Vec::new();

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => {
                for field in &fields.named {
                    let field_name = field.ident.as_ref().unwrap();

                    // Per-field opt-out: `#[smart_is_default(skip)]` suppresses
                    // the `is_default__<field>` helper for this field only.
                    // In the auto-generated Serialize impl, the field is still
                    // emitted, just without a skip check.
                    let is_skipped = field_has_flag(field, "smart_is_default", "skip");
                    let fn_name = format_ident!("is_default__{}", field_name);

                    if !is_skipped {
                        let field_ty = &field.ty;
                        helper_fns.push(quote! {
                            #[allow(non_snake_case)]
                            #helper_vis fn #fn_name(v: &#field_ty) -> bool {
                                v == &Self::default().#field_name
                            }
                        });
                    }

                    field_infos.push(FieldInfo {
                        kind: FieldKind::Named,
                        accessor: quote! { #field_name },
                        helper_fn: if is_skipped { None } else { Some(fn_name) },
                        label: Some(syn::LitStr::new(
                            &field_name.to_string(),
                            Span::call_site(),
                        )),
                    });
                }
            }
            Fields::Unnamed(fields) => {
                for (field_index, field) in fields.unnamed.iter().enumerate() {
                    let is_skipped = field_has_flag(field, "smart_is_default", "skip");
                    let fn_name = format_ident!("is_default__{}", field_index);

                    if !is_skipped {
                        let field_ty = &field.ty;
                        let index = Index::from(field_index);
                        helper_fns.push(quote! {
                            #[allow(non_snake_case)]
                            #helper_vis fn #fn_name(v: &#field_ty) -> bool {
                                v == &Self::default().#index
                            }
                        });
                    }

                    field_infos.push(FieldInfo {
                        kind: FieldKind::Unnamed,
                        accessor: {
                            let index = Index::from(field_index);
                            quote! { #index }
                        },
                        helper_fn: if is_skipped { None } else { Some(fn_name) },
                        label: None,
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

    let serialize_impl = if serde_skip {
        build_serialize_impl(
            name,
            &impl_generics,
            &ty_generics,
            where_clause,
            &field_infos,
        )
    } else {
        quote! {}
    };

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #is_default_method

            #( #helper_fns )*
        }

        #serialize_impl
    };
    expanded.into()
}

/// Build a `serde::Serialize` impl that skips any field whose value equals
/// the struct's default — i.e. the auto-generated equivalent of writing
/// `#[serde(skip_serializing_if = "Struct::is_default__<field>")]` on every
/// field.
fn build_serialize_impl(
    name: &syn::Ident,
    impl_generics: &syn::ImplGenerics,
    ty_generics: &syn::TypeGenerics,
    where_clause: Option<&syn::WhereClause>,
    field_infos: &[FieldInfo],
) -> proc_macro2::TokenStream {
    if field_infos.is_empty() {
        return quote! {};
    }

    let has_named = field_infos
        .iter()
        .any(|f| matches!(f.kind, FieldKind::Named));
    let has_unnamed = field_infos
        .iter()
        .any(|f| matches!(f.kind, FieldKind::Unnamed));

    // Mixed named/unnamed shouldn't happen for any well-formed struct, and
    // we can't sensibly skip in that case. Bail out silently.
    if has_named && has_unnamed {
        return quote! {};
    }

    let struct_name = syn::LitStr::new(&name.to_string(), Span::call_site());

    if has_named {
        let count_checks = field_infos.iter().map(|info| {
            let accessor = &info.accessor;
            match &info.helper_fn {
                Some(fn_name) => quote! {
                    if !#name::#fn_name(&self.#accessor) {
                        len += 1;
                    }
                },
                // Field opted out of the helper — always count it. The
                // `let _ = ...` consumes the `accessor` binding so the
                // generated code doesn't trip an unused-variable lint.
                None => quote! {
                    let _ = &self.#accessor;
                    len += 1;
                },
            }
        });

        let serialize_fields = field_infos.iter().map(|info| {
            let accessor = &info.accessor;
            let label = info
                .label
                .as_ref()
                .expect("named fields always carry a string label");
            match &info.helper_fn {
                Some(fn_name) => quote! {
                    if !#name::#fn_name(&self.#accessor) {
                        state.serialize_field(#label, &self.#accessor)?;
                    }
                },
                None => quote! {
                    state.serialize_field(#label, &self.#accessor)?;
                },
            }
        });

        quote! {
            #[automatically_derived]
            impl #impl_generics ::serde::Serialize for #name #ty_generics #where_clause {
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    use ::serde::ser::SerializeStruct as _;
                    let mut len = 0usize;
                    #( #count_checks )*
                    let mut state = serializer.serialize_struct(#struct_name, len)?;
                    #( #serialize_fields )*
                    state.end()
                }
            }
        }
    } else {
        // Tuple struct: skipping elements would break deserialization, so
        // emit a plain `serialize_tuple_struct` impl.
        let serialize_fields = field_infos.iter().map(|info| {
            let accessor = &info.accessor;
            quote! {
                state.serialize_field(&self.#accessor)?;
            }
        });
        let count = field_infos.len();

        quote! {
            #[automatically_derived]
            impl #impl_generics ::serde::Serialize for #name #ty_generics #where_clause {
                fn serialize<S>(
                    &self,
                    serializer: S,
                ) -> ::core::result::Result<S::Ok, S::Error>
                where
                    S: ::serde::Serializer,
                {
                    use ::serde::ser::SerializeTupleStruct as _;
                    let mut state = serializer.serialize_tuple_struct(#struct_name, #count)?;
                    #( #serialize_fields )*
                    state.end()
                }
            }
        }
    }
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
