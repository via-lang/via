/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input, spanned::Spanned};

struct VariantInfo {
    pattern: proc_macro2::TokenStream,
    number: u32,
    code: String,
}

fn parse_enum_config(input: &DeriveInput) -> Result<(String, u32), syn::Error> {
    let mut prefix = "E".to_string();
    let mut start: u32 = 0;

    for attr in &input.attrs {
        if attr.path().is_ident("diag") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("prefix") {
                    if let syn::Lit::Str(s) = meta.value()?.parse()? {
                        prefix = s.value();
                    }
                } else if meta.path.is_ident("start")
                    && let syn::Lit::Int(i) = meta.value()?.parse()?
                {
                    start = i.base10_parse()?;
                }
                Ok(())
            })?;
        }
    }

    Ok((prefix, start))
}

fn collect_variants(
    enum_name: &syn::Ident,
    data: &syn::DataEnum,
    prefix: &str,
    start: u32,
) -> Vec<VariantInfo> {
    data.variants
        .iter()
        .enumerate()
        .map(|(i, variant)| {
            let ident = &variant.ident;
            let number = start + i as u32;
            let code = format!("{prefix}{number:04}");

            let pattern = match &variant.fields {
                Fields::Unit => quote! { #enum_name::#ident },
                Fields::Unnamed(_) => quote! { #enum_name::#ident(_) },
                Fields::Named(_) => quote! { #enum_name::#ident { .. } },
            };

            VariantInfo {
                pattern,
                number,
                code,
            }
        })
        .collect()
}

fn emit_impl(enum_name: &syn::Ident, variants: &[VariantInfo]) -> proc_macro2::TokenStream {
    let number_arms = variants.iter().map(|v| {
        let pat = &v.pattern;
        let n = v.number;
        quote! { #pat => #n }
    });

    let code_arms = variants.iter().map(|v| {
        let pat = &v.pattern;
        let code = &v.code;
        quote! { #pat => #code }
    });

    quote! {
        impl #enum_name {
            pub fn number(&self) -> u32 {
                match self {
                    #( #number_arms, )*
                }
            }

            pub fn code(&self) -> &'static str {
                match self {
                    #( #code_arms, )*
                }
            }
        }
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;

    let data = match &input.data {
        Data::Enum(data) => data,
        _ => {
            return syn::Error::new(input.span(), "DiagCode can only be derived for enums")
                .to_compile_error()
                .into();
        }
    };

    let (prefix, start) = match parse_enum_config(&input) {
        Ok(v) => v,
        Err(e) => return e.to_compile_error().into(),
    };

    let variants = collect_variants(enum_name, data, &prefix, start);
    emit_impl(enum_name, &variants).into()
}
