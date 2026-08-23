use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

fn parse_keyword(variant: &syn::Variant) -> Result<Option<syn::LitStr>, syn::Error> {
    for attr in &variant.attrs {
        if attr.path().is_ident("keyword") {
            return Ok(Some(attr.parse_args::<syn::LitStr>()?));
        }
    }
    Ok(None)
}

fn parse_operator(variant: &syn::Variant) -> Result<Option<syn::LitStr>, syn::Error> {
    for attr in &variant.attrs {
        if attr.path().is_ident("operator") {
            return Ok(Some(attr.parse_args::<syn::LitStr>()?));
        }
    }
    Ok(None)
}

fn make_arm(variant: &syn::Variant, body: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let name = &variant.ident;
    match &variant.fields {
        syn::Fields::Unit => quote! { Self::#name => #body },
        _ => quote! { Self::#name { .. } => #body },
    }
}

// Only unit variants can be used as map values (no data to construct)
fn is_unit(variant: &syn::Variant) -> bool {
    matches!(variant.fields, syn::Fields::Unit)
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_name = &input.ident;
    let enum_data = match &input.data {
        syn::Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(enum_name, "Syntax only allowed on enums")
                .to_compile_error()
                .into();
        }
    };

    let mut keyword_arms = Vec::new();
    let mut operator_arms = Vec::new();

    // For reverse maps: (literal, variant ident) pairs
    let mut keyword_map_entries: Vec<(syn::LitStr, syn::Ident)> = Vec::new();
    let mut operator_map_entries: Vec<(syn::LitStr, syn::Ident)> = Vec::new();

    for variant in &enum_data.variants {
        let name = &variant.ident;

        // --- keyword ---
        let kw_val = match parse_keyword(variant) {
            Ok(v) => v,
            Err(e) => return e.to_compile_error().into(),
        };
        let kw_body = match &kw_val {
            Some(lit) => quote! { Some(#lit) },
            None => quote! { None },
        };
        keyword_arms.push(make_arm(variant, kw_body));

        if let Some(lit) = kw_val
            && is_unit(variant)
        {
            keyword_map_entries.push((lit, name.clone()));
        }

        // --- operator ---
        let op_val = match parse_operator(variant) {
            Ok(v) => v,
            Err(e) => return e.to_compile_error().into(),
        };
        let op_body = match &op_val {
            Some(lit) => quote! { Some(#lit) },
            None => quote! { None },
        };
        operator_arms.push(make_arm(variant, op_body));

        if let Some(lit) = op_val
            && is_unit(variant)
        {
            operator_map_entries.push((lit, name.clone()));
        }
    }

    keyword_map_entries.sort_by_key(|(lit, _)| std::cmp::Reverse(lit.value().len()));
    operator_map_entries.sort_by_key(|(lit, _)| std::cmp::Reverse(lit.value().len()));

    let kw_match_tokens = keyword_map_entries.iter().map(|(lit, ident)| {
        quote! { #lit => Some(#enum_name::#ident) }
    });

    let op_match_tokens = operator_map_entries.iter().map(|(lit, ident)| {
        quote! { #lit => Some(#enum_name::#ident) }
    });

    quote! {
        impl #enum_name {
            pub fn keyword(&self) -> Option<&'static str> {
                match self {
                    #(#keyword_arms),*
                }
            }

            pub fn operator(&self) -> Option<&'static str> {
                match self {
                    #(#operator_arms),*
                }
            }

            pub fn from_keyword(s: &str) -> Option<Self> {
                match s {
                    #(#kw_match_tokens,)*
                    _ => None,
                }
            }

            pub fn from_operator(s: &str) -> Option<Self> {
                match s {
                    #(#op_match_tokens,)*
                    _ => None,
                }
            }
        }
    }
    .into()
}
