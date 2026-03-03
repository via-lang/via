/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident;

    let data_enum = match input.data {
        syn::Data::Enum(e) => e,
        _ => panic!("Diagnostic derive only works on enums"),
    };

    let mut generated = Vec::new();

    for variant in data_enum.variants {
        let variant_ident = variant.ident;
        let fn_ident = format_ident!("{}", variant_ident.to_string().to_lowercase());

        let mut severity = None;

        for attr in variant.attrs {
            if attr.path().is_ident("severity") {
                let ident: syn::Ident = attr.parse_args().unwrap();
                severity = Some(ident.to_string());
            }
        }

        let severity = severity.expect("Missing #[severity(...)]");
        let tokens = match severity.as_str() {
            "Info" => ,
            _ => panic!("Unknown severity type"),
        };

        generated.push(tokens);
    }

    quote! {}.into()
}
