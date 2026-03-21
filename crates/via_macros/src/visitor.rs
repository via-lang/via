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
use syn::{Data, DeriveInput, Fields, GenericArgument, PathArguments, Type, parse_macro_input};

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            out.push('_');
        }
        out.push(c.to_lowercase().next().unwrap());
    }
    out
}

fn extract_id_inner(ty: &Type) -> Option<proc_macro2::TokenStream> {
    if let Type::Path(tp) = ty {
        let seg = tp.path.segments.last()?;
        if seg.ident != "Id" {
            return None;
        }
        if let PathArguments::AngleBracketed(args) = &seg.arguments {
            if let Some(GenericArgument::Type(inner)) = args.args.first() {
                return Some(quote! { #inner });
            }
        }
    }
    None
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let variants = match &input.data {
        Data::Enum(e) => &e.variants,
        _ => panic!("Visitor can only be derived on enums"),
    };

    let walk_name = syn::Ident::new(
        &format!("walk_{}", to_snake_case(&name.to_string())),
        name.span(),
    );

    let visit_name = syn::Ident::new(
        &format!("visit_{}", to_snake_case(&name.to_string())),
        name.span(),
    );

    let mut match_arms = vec![];

    for variant in variants {
        let variant_name = &variant.ident;

        let recurse_calls: Vec<_> = match &variant.fields {
            Fields::Unnamed(f) => f
                .unnamed
                .iter()
                .enumerate()
                .filter_map(|(i, field)| {
                    let id_inner = extract_id_inner(&field.ty)?;
                    let visit_method = syn::Ident::new(
                        &format!("visit_{}", to_snake_case(&quote!(#id_inner).to_string())),
                        proc_macro2::Span::call_site(),
                    );
                    let binding =
                        syn::Ident::new(&format!("f{}", i), proc_macro2::Span::call_site());
                    Some((binding, visit_method))
                })
                .collect(),
            Fields::Named(f) => f
                .named
                .iter()
                .filter_map(|field| {
                    let id_inner = extract_id_inner(&field.ty)?;
                    let visit_method = syn::Ident::new(
                        &format!("visit_{}", to_snake_case(&quote!(#id_inner).to_string())),
                        proc_macro2::Span::call_site(),
                    );
                    let binding = field.ident.clone().unwrap();
                    Some((binding, visit_method))
                })
                .collect(),
            Fields::Unit => vec![],
        };

        let pattern = match &variant.fields {
            Fields::Unnamed(f) => {
                let bindings: Vec<_> = f
                    .unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        syn::Ident::new(&format!("f{}", i), proc_macro2::Span::call_site())
                    })
                    .collect();
                quote! { #name::#variant_name(#(#bindings),*) }
            }
            Fields::Named(f) => {
                let bindings: Vec<_> = f.named.iter().map(|f| f.ident.as_ref().unwrap()).collect();
                quote! { #name::#variant_name { #(#bindings),* } }
            }
            Fields::Unit => quote! { #name::#variant_name },
        };

        let calls: Vec<_> = recurse_calls
            .iter()
            .map(|(binding, visit_method)| {
                quote! { visitor.#visit_method(tree, *#binding); }
            })
            .collect();

        match_arms.push(quote! {
            #pattern => { #(#calls)* }
        });
    }

    quote! {
        pub fn #walk_name<V, T>(visitor: &mut V, tree: &T, id: crate::node::NodeId<#name>)
        where
            V: crate::node::Visitor,
            T: crate::node::NodeStore<#name>,
        {
            match &tree[id] {
                #(#match_arms,)*
            }
        }
    }
    .into()
}
