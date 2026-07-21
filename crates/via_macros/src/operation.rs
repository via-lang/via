use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Lit, Meta, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let data_enum = match &input.data {
        Data::Enum(data) => data,
        _ => panic!("Operation can only be derived on enums!"),
    };

    let mut from_arms = Vec::new();
    let mut trait_arms = Vec::new();

    for variant in &data_enum.variants {
        let variant_name = &variant.ident;
        let mut from_token = None;
        let mut trait_name = None;
        let mut trait_method = None;

        for attr in &variant.attrs {
            if attr.path().is_ident("from") {
                if let Meta::List(meta_list) = &attr.meta {
                    let tokens = &meta_list.tokens;
                    from_token = Some(quote! { #tokens });
                }
            } else if attr.path().is_ident("trait_info") {
                let _ = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("name") {
                        let value: Expr = meta.value()?.parse()?;
                        if let Expr::Lit(expr_lit) = value
                            && let Lit::Str(lit_str) = expr_lit.lit
                        {
                            trait_name = Some(lit_str.value());
                        }
                    } else if meta.path.is_ident("method") {
                        let value: Expr = meta.value()?.parse()?;
                        if let Expr::Lit(expr_lit) = value
                            && let Lit::Str(lit_str) = expr_lit.lit
                        {
                            trait_method = Some(lit_str.value());
                        }
                    }
                    Ok(())
                });
            }
        }

        let kind_path = match from_token {
            Some(path) => path,
            None => {
                return syn::Error::new_spanned(
                    variant,
                    format!(
                        "Missing required `#[from(...)]` attribute on variant `{}`",
                        variant_name
                    ),
                )
                .to_compile_error()
                .into();
            }
        };

        let (t_name, t_method) = match (trait_name, trait_method) {
            (Some(name), Some(method)) => (name, method),
            _ => {
                return syn::Error::new_spanned(
                    variant,
                    format!(
                        "Missing required `#[trait_info(name = \"...\", method = \"...\")]` on variant `{}`",
                        variant_name
                    ),
                )
                .to_compile_error()
                .into();
            }
        };

        from_arms.push(quote! {
            crate::syntax::SyntaxKind::#kind_path => Some(#name::#variant_name),
        });

        trait_arms.push(quote! {
            #name::#variant_name => (#t_name, #t_method),
        });
    }

    let expanded = quote! {
        impl #name {
            pub fn from_syntax(kind: crate::syntax::SyntaxKind) -> Option<Self> {
                match kind {
                    #(#from_arms)*
                    _ => None,
                }
            }

            pub fn trait_info(&self) -> (&'static str, &'static str) {
                match self {
                    #(#trait_arms)*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
