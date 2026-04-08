use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return syn::Error::new(Span::call_site(), "Access requires named fields")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new(Span::call_site(), "Access can only be derived on structs")
                .to_compile_error()
                .into();
        }
    };

    let mut impls = Vec::new();

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        let has_getter = field.attrs.iter().any(|a| a.path().is_ident("getter"));

        if has_getter {
            impls.push(quote! {
                impl Access<#field_ty> for #struct_name {
                    fn get(&self) -> &#field_ty {
                        &self.#field_name
                    }

                    fn get_mut(&mut self) -> &mut #field_ty {
                        &mut self.#field_name
                    }
                }
            });
        }
    }

    quote! {
        #(#impls)*
    }
    .into()
}
