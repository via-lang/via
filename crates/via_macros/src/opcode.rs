use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

pub fn expand(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = input.ident;

    let data_enum = match input.data {
        syn::Data::Enum(e) => e,
        _ => panic!("Opcode derive only works on enums"),
    };

    let mut generated = Vec::new();

    for variant in data_enum.variants {
        let variant_ident = variant.ident;
        let fn_ident = format_ident!("{}", variant_ident.to_string().to_lowercase());

        let mut layout_name = None;

        for attr in variant.attrs {
            if attr.path().is_ident("layout") {
                let ident: syn::Ident = attr.parse_args().unwrap();
                layout_name = Some(ident.to_string());
            }
        }

        let layout = layout_name.expect("Missing #[layout(...)]");
        let tokens = match layout.as_str() {
            "Rx0" => quote! {
                impl Instruction {
                    pub fn #fn_ident() -> Self {
                        Instruction::new_rx(#enum_ident::#variant_ident, &[])
                    }
                }
            },
            "Rx1" => quote! {
                impl Instruction {
                    pub fn #fn_ident(a: Operand) -> Self {
                        Instruction::new_rx(#enum_ident::#variant_ident, &[a])
                    }
                }
            },
            "Rx2" => quote! {
                impl Instruction {
                    pub fn #fn_ident(a: Operand, b: Operand) -> Self {
                        Instruction::new_rx(#enum_ident::#variant_ident, &[a, b])
                    }
                }
            },
            "Rx3" => quote! {
                impl Instruction {
                    pub fn #fn_ident(a: Operand, b: Operand, c: Operand) -> Self {
                        Instruction::new_rx(#enum_ident::#variant_ident, &[a, b, c])
                    }
                }
            },
            "RIm" => quote! {
                impl Instruction {
                    pub fn #fn_ident(a: Operand, imm: u16) -> Self {
                        Instruction::new_rim(#enum_ident::#variant_ident, a, imm)
                    }
                }
            },
            _ => panic!("Unknown layout type"),
        };

        generated.push(tokens);
    }

    quote! {
        #(#generated)*
    }
    .into()
}
