use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let mut inner_ty = quote! { u32 };

    for attr in input.attrs {
        if attr.path().is_ident("id") {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("inner") {
                    let value: syn::Type = meta.value()?.parse()?;
                    inner_ty = quote! { #value };
                }
                Ok(())
            });
        }
    }

    let expanded = quote! {
        impl #name {
            pub fn new(inner: #inner_ty) -> Self {
                Self(inner)
            }

            pub fn inner(self) -> #inner_ty {
                self.0
            }
        }

        impl crate::counter::Id for #name {
            type Inner = #inner_ty;

            fn from_inner(inner: Self::Inner) -> Self {
                Self(inner)
            }

            fn inner(self) -> Self::Inner {
                self.0
            }
        }

        impl std::clone::Clone for #name {
            fn clone(&self) -> Self {
                *self
            }
        }

        impl std::marker::Copy for #name {}

        impl std::fmt::Debug for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}({})", stringify!(#name), self.0)
            }
        }

        impl std::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl std::cmp::Eq for #name {}

        impl std::hash::Hash for #name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }
    };

    TokenStream::from(expanded)
}
