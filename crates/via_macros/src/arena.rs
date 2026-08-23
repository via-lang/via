use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    Data, DeriveInput, Fields, GenericArgument, Meta, PathArguments, Type, parse_macro_input,
    spanned::Spanned,
};

fn extract_vec_inner(ty: &Type) -> Option<&Type> {
    let Type::Path(tp) = ty else { return None };
    let seg = tp.path.segments.last()?;
    if seg.ident != "Vec" {
        return None;
    }
    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return None;
    };
    if let Some(GenericArgument::Type(inner)) = args.args.first() {
        Some(inner)
    } else {
        None
    }
}

fn validate_map_type(
    map_ty: &Type,
    expected_inner: &Type,
    map_field_span: Span,
) -> Result<(), syn::Error> {
    let Type::Path(tp) = map_ty else {
        return Err(syn::Error::new(
            map_field_span,
            "map field type must be a HashMap<T, NodeId<T>>",
        ));
    };

    let seg = tp.path.segments.last().ok_or_else(|| {
        syn::Error::new(
            map_field_span,
            "map field type must be a HashMap<T, NodeId<T>>",
        )
    })?;

    let PathArguments::AngleBracketed(args) = &seg.arguments else {
        return Err(syn::Error::new(
            map_field_span,
            "map field type must be a HashMap<T, NodeId<T>>",
        ));
    };

    let tyargs: Vec<&Type> = args
        .args
        .iter()
        .filter_map(|a| {
            if let GenericArgument::Type(t) = a {
                Some(t)
            } else {
                None
            }
        })
        .collect();

    if tyargs.len() < 2 {
        return Err(syn::Error::new(
            map_field_span,
            "map field must have at least two type arguments: HashMap<T, NodeId<T>>",
        ));
    }

    let key_ty = tyargs[0];
    let val_ty = tyargs[1];

    let key_str = quote!(#key_ty).to_string();
    let expected_str = quote!(#expected_inner).to_string();
    if key_str != expected_str {
        return Err(syn::Error::new(
            map_field_span,
            format!(
                "map key type `{}` does not match vec inner type `{}`",
                key_str, expected_str
            ),
        ));
    }

    let Type::Path(val_tp) = val_ty else {
        return Err(syn::Error::new(
            map_field_span,
            "map value type must be NodeId<T>",
        ));
    };
    let val_seg = val_tp
        .path
        .segments
        .last()
        .ok_or_else(|| syn::Error::new(map_field_span, "map value type must be NodeId<T>"))?;
    if val_seg.ident != "NodeId" {
        return Err(syn::Error::new(
            map_field_span,
            format!(
                "map value type must be `NodeId<T>`, found `{}`",
                val_seg.ident
            ),
        ));
    }
    let PathArguments::AngleBracketed(val_args) = &val_seg.arguments else {
        return Err(syn::Error::new(
            map_field_span,
            "NodeId must have a type argument: NodeId<T>",
        ));
    };
    let Some(GenericArgument::Type(node_id_inner)) = val_args.args.first() else {
        return Err(syn::Error::new(
            map_field_span,
            "NodeId must have a type argument: NodeId<T>",
        ));
    };
    let node_id_inner_str = quote!(#node_id_inner).to_string();
    if node_id_inner_str != expected_str {
        return Err(syn::Error::new(
            map_field_span,
            format!(
                "NodeId inner type `{}` does not match vec inner type `{}`",
                node_id_inner_str, expected_str
            ),
        ));
    }

    Ok(())
}

fn extract_map_arg(attr: &syn::Attribute) -> Result<syn::Ident, syn::Error> {
    let Meta::List(list) = &attr.meta else {
        return Err(syn::Error::new(
            attr.span(),
            "expected `#[interner(map = \"field_name\")]`",
        ));
    };

    let mut map_ident: Option<syn::Ident> = None;

    list.parse_nested_meta(|meta| {
        if meta.path.is_ident("map") {
            let value = meta.value().map_err(|_| {
                syn::Error::new(meta.path.span(), "expected `map = \"field_name\"`")
            })?;
            let lit: syn::LitStr = value.parse()?;
            map_ident = Some(syn::Ident::new(&lit.value(), lit.span()));
            Ok(())
        } else {
            Err(syn::Error::new(
                meta.path.span(),
                format!(
                    "unknown argument `{}`, expected `map`",
                    meta.path
                        .get_ident()
                        .map(|i| i.to_string())
                        .unwrap_or_default()
                ),
            ))
        }
    })?;

    map_ident.ok_or_else(|| {
        syn::Error::new(attr.span(), "missing `map = \"field_name\"` in #[interner]")
    })
}

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(f) => &f.named,
            _ => {
                return syn::Error::new(Span::call_site(), "Arena requires named fields")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new(Span::call_site(), "Arena can only be derived on structs")
                .to_compile_error()
                .into();
        }
    };

    let all_field_names: Vec<String> = fields
        .iter()
        .filter_map(|f| f.ident.as_ref().map(|i| i.to_string()))
        .collect();

    let mut alloc_methods = vec![];
    let mut intern_methods = vec![];
    let mut index_impls = vec![];
    let mut index_mut_impls = vec![];
    let mut errors = vec![];

    for field in fields {
        let field_name = field.ident.as_ref().unwrap();

        let has_arena = field.attrs.iter().any(|a| a.path().is_ident("allocator"));
        let interned_attr = field.attrs.iter().find(|a| a.path().is_ident("interner"));

        if has_arena && interned_attr.is_some() {
            errors.push(syn::Error::new(
                field_name.span(),
                "a field cannot have both `#[allocator]` and `#[interner]`",
            ));
            continue;
        }

        if !has_arena && interned_attr.is_none() {
            continue;
        }

        let inner_ty = match extract_vec_inner(&field.ty) {
            Some(t) => t,
            None => {
                errors.push(syn::Error::new(
                    field.ty.span(),
                    "allocator fields must be `Vec<T>`",
                ));
                continue;
            }
        };

        if has_arena {
            let alloc_name = syn::Ident::new(&format!("alloc_{}", field_name), field_name.span());

            alloc_methods.push(quote! {
                pub fn #alloc_name(&mut self, value: #inner_ty) -> crate::node::NodeId<#inner_ty> {
                    let index = self.#field_name.len() as u32;
                    self.#field_name.push(value);
                    crate::node::NodeId::new(index)
                }
            });

            index_impls.push(quote! {
                impl #impl_generics std::ops::Index<crate::node::NodeId<#inner_ty>>
                    for #name #ty_generics #where_clause
                {
                    type Output = #inner_ty;
                    fn index(&self, id: crate::node::NodeId<#inner_ty>) -> &#inner_ty {
                        &self.#field_name[id.index() as usize]
                    }
                }
            });

            index_mut_impls.push(quote! {
                impl #impl_generics std::ops::IndexMut<crate::node::NodeId<#inner_ty>>
                    for #name #ty_generics #where_clause
                {
                    fn index_mut(&mut self, id: crate::node::NodeId<#inner_ty>) -> &mut #inner_ty {
                        &mut self.#field_name[id.index() as usize]
                    }
                }
            });
        }

        if let Some(attr) = interned_attr {
            let map_ident = match extract_map_arg(attr) {
                Ok(id) => id,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            if !all_field_names.contains(&map_ident.to_string()) {
                errors.push(syn::Error::new(
                    map_ident.span(),
                    format!("no field `{}` found on `{}`", map_ident, name),
                ));
                continue;
            }

            let map_field = fields
                .iter()
                .find(|f| f.ident.as_ref().map(|i| i == &map_ident).unwrap_or(false))
                .unwrap();

            if let Err(e) = validate_map_type(&map_field.ty, inner_ty, map_field.span()) {
                errors.push(e);
                continue;
            }

            let intern_name = syn::Ident::new(&format!("intern_{}", field_name), field_name.span());

            intern_methods.push(quote! {
                pub fn #intern_name(&mut self, value: #inner_ty) -> crate::node::NodeId<#inner_ty>
                where
                    #inner_ty: std::hash::Hash + Eq + Clone,
                {
                    if let Some(&existing) = self.#map_ident.get(&value) {
                        return existing;
                    }
                    let index = self.#field_name.len() as u32;
                    let id = crate::node::NodeId::new(index);
                    self.#field_name.push(value.clone());
                    self.#map_ident.insert(value, id);
                    id
                }
            });

            index_impls.push(quote! {
                impl #impl_generics std::ops::Index<crate::node::NodeId<#inner_ty>>
                    for #name #ty_generics #where_clause
                {
                    type Output = #inner_ty;
                    fn index(&self, id: crate::node::NodeId<#inner_ty>) -> &#inner_ty {
                        &self.#field_name[id.index() as usize]
                    }
                }
            });
        }
    }

    if !errors.is_empty() {
        return errors
            .into_iter()
            .reduce(|mut a, b| {
                a.combine(b);
                a
            })
            .unwrap()
            .to_compile_error()
            .into();
    }

    quote! {
        impl #impl_generics #name #ty_generics #where_clause {
            #(#alloc_methods)*
            #(#intern_methods)*
        }

        #(#index_impls)*
        #(#index_mut_impls)*
    }
    .into()
}
