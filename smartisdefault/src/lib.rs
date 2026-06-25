extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(SmartIsDefault, attributes(default))]
pub fn derive_smart_is_default(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut helper_fns = Vec::new();

    if let Data::Struct(data) = input.data {
        if let Fields::Named(fields) = data.fields {
            for field in fields.named {
                let field_name = field.ident.as_ref().unwrap();
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
    }

    let expanded = quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #( #helper_fns )*
        }
    };
    expanded.into()
}
