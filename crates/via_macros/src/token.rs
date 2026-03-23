use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

fn parse_prec(variant: &syn::Variant) -> Result<Option<syn::Expr>, syn::Error> {
    for attr in &variant.attrs {
        if attr.path().is_ident("prec") {
            return Ok(Some(attr.parse_args::<syn::Expr>()?));
        }
    }
    Ok(None)
}

fn parse_prec_type(input: &DeriveInput) -> Result<syn::Type, syn::Error> {
    for attr in &input.attrs {
        if attr.path().is_ident("token_kind") {
            return attr.parse_args::<syn::Type>();
        }
    }
    Ok(syn::parse_quote!(u32))
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let ret_ty = match parse_prec_type(&input) {
        Ok(t) => t,
        Err(e) => return e.to_compile_error().into(),
    };

    let enum_data = match &input.data {
        syn::Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(enum_name, "Token only works on enums")
                .to_compile_error()
                .into();
        }
    };

    let mut arms = Vec::new();

    for variant in &enum_data.variants {
        let name = &variant.ident;

        let value = match parse_prec(variant) {
            Ok(v) => v,
            Err(e) => return e.to_compile_error().into(),
        };

        let arm = match (&variant.fields, value) {
            (syn::Fields::Unit, Some(expr)) => {
                quote! { Self::#name => Some(#expr) }
            }
            (syn::Fields::Unit, None) => {
                quote! { Self::#name => None }
            }
            (_, Some(expr)) => {
                quote! { Self::#name { .. } => Some(#expr) }
            }
            (_, None) => {
                quote! { Self::#name { .. } => None }
            }
        };

        arms.push(arm);
    }

    quote! {
        impl #enum_name {
            pub fn prec(&self) -> Option<#ret_ty> {
                match self {
                    #(#arms),*
                }
            }
        }
    }
    .into()
}
