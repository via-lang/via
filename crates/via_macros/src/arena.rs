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

fn extract_vec_inner(ty: &Type) -> Option<&Type> {
    if let Type::Path(tp) = ty {
        let seg = tp.path.segments.last()?;
        if seg.ident != "Vec" {
            return None;
        }
        if let PathArguments::AngleBracketed(args) = &seg.arguments {
            if let Some(GenericArgument::Type(inner)) = args.args.first() {
                return Some(inner);
            }
        }
    }
    None
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => panic!("Arena requires named fields"),
        },
        _ => panic!("Arena can only be derived on structs"),
    };

    let mut alloc_methods = vec![];
    let mut store_impls = vec![];
    let mut index_impls = vec![];

    for field in fields {
        let has_arena_attr = field.attrs.iter().any(|a| a.path().is_ident("arena"));
        if !has_arena_attr {
            continue;
        }

        let field_name = field.ident.as_ref().unwrap();
        let inner_ty =
            extract_vec_inner(&field.ty).unwrap_or_else(|| panic!("Arena fields must be Vec<T>"));

        let alloc_name = syn::Ident::new(&format!("alloc_{}", field_name), field_name.span());

        alloc_methods.push(quote! {
            pub fn #alloc_name(&mut self, value: #inner_ty) -> crate::node::NodeId<#inner_ty> {
                let index = self.#field_name.len() as u32;
                self.#field_name.push(value);
                crate::node::NodeId::new(index)
            }
        });

        store_impls.push(quote! {
            impl #impl_generics crate::node::NodeStore<#inner_ty> for #name #ty_generics #where_clause {
                fn get(&self, id: crate::node::NodeId<#inner_ty>) -> &#inner_ty {
                    &self.#field_name[id.index() as usize]
                }
            }
        });

        index_impls.push(quote! {
            impl #impl_generics std::ops::Index<crate::node::NodeId<#inner_ty>> for #name #ty_generics #where_clause {
                type Output = #inner_ty;

                fn index(&self, id: crate::node::NodeId<#inner_ty>) -> &#inner_ty {
                    self.get(id)
                }
            }
        });
    }

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#alloc_methods)*
        }

        #(#store_impls)*
        #(#index_impls)*
    }
    .into()
}
