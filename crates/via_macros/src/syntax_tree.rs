use std::collections::{BTreeMap, HashMap};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Ident, Result, Token};

syn::custom_keyword!(Leaf);
syn::custom_keyword!(Branch);
syn::custom_keyword!(Group);
syn::custom_keyword!(Delimited);

enum Rule {
    Leaf(Ident),
    Branch(Ident),
    Group(Ident),
    Delimited {
        open: Ident,
        close: Ident,
        target: Box<Rule>,
    },
}

enum Cardinality {
    Required,
    Optional,
    Many,
}

struct Child {
    name: Ident,
    rule: Rule,
    cardinality: Cardinality,
}

struct AstNode {
    name: Ident,
    kind: Option<Ident>,
    group: Option<Ident>,
    children: Vec<Child>,
}

struct Config {
    nodes: Vec<AstNode>,
}

fn parse_simple_rule(input: ParseStream) -> Result<Rule> {
    if input.peek(Leaf) {
        input.parse::<Leaf>()?;

        let content;
        syn::parenthesized!(content in input);

        Ok(Rule::Leaf(content.parse()?))
    } else if input.peek(Branch) {
        input.parse::<Branch>()?;

        let content;
        syn::parenthesized!(content in input);

        Ok(Rule::Branch(content.parse()?))
    } else if input.peek(Group) {
        input.parse::<Group>()?;

        let content;
        syn::parenthesized!(content in input);

        Ok(Rule::Group(content.parse()?))
    } else {
        Err(input.error("Expected 'Leaf', 'Branch', or 'Group'"))
    }
}

impl Parse for Child {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        input.parse::<Token![:]>()?;

        let rule = if input.peek(Delimited) {
            input.parse::<Delimited>()?;

            let content;
            syn::parenthesized!(content in input);

            let target = parse_simple_rule(&content)?;

            content.parse::<Token![,]>()?;
            let open: Ident = content.parse()?;
            content.parse::<Token![,]>()?;
            let close: Ident = content.parse()?;

            Rule::Delimited {
                open,
                close,
                target: Box::new(target),
            }
        } else {
            parse_simple_rule(input).map_err(|_| {
                input.error(
                    "Expected field rule definition: 'Leaf', 'Branch', 'Group', or 'Delimited'",
                )
            })?
        };

        let cardinality = if input.peek(Token![?]) {
            input.parse::<Token![?]>()?;
            Cardinality::Optional
        } else if input.peek(Token![*]) {
            input.parse::<Token![*]>()?;
            Cardinality::Many
        } else {
            Cardinality::Required
        };

        if matches!(rule, Rule::Delimited { .. }) && !matches!(cardinality, Cardinality::Required) {
            return Err(input.error(
                "Delimited(..) already represents a collection; '?' and '*' are not allowed on it",
            ));
        }

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        Ok(Child {
            name,
            rule,
            cardinality,
        })
    }
}

impl Parse for AstNode {
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![struct]>()?;

        let name = input.parse::<Ident>()?;
        let kind = if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);

            Some(content.parse::<Ident>()?)
        } else {
            None
        };

        let group = if input.peek(Token![in]) {
            input.parse::<Token![in]>()?;
            Some(input.parse::<Ident>()?)
        } else {
            None
        };

        let content;
        syn::braced!(content in input);

        let mut children = Vec::new();

        while !content.is_empty() {
            children.push(content.parse()?);
        }

        Ok(AstNode {
            name,
            kind,
            group,
            children,
        })
    }
}

impl Parse for Config {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();

        while !input.is_empty() {
            nodes.push(input.parse()?);
        }

        Ok(Config { nodes })
    }
}

fn to_screaming_snake(s: &str) -> String {
    let mut snake = String::new();

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            snake.push('_');
        }
        snake.extend(ch.to_uppercase());
    }

    snake
}

fn delimited_getter(
    ch_name: &Ident,
    open: &Ident,
    close: &Ident,
    target: &Rule,
) -> proc_macro2::TokenStream {
    match target {
        Rule::Leaf(leaf_kind) => quote! {
            pub fn #ch_name(&self) -> impl Iterator<Item = SyntaxToken> {
                let start_sk = SyntaxKind::#open;
                let end_sk = SyntaxKind::#close;

                self.0
                    .children_with_tokens()
                    .skip_while(move |element| element.kind() != start_sk)
                    .skip(1)
                    .take_while(move |element| element.kind() != end_sk)
                    .filter_map(|element| element.into_token())
                    .filter(|token| token.kind() == SyntaxKind::#leaf_kind)
            }
        },
        Rule::Branch(branch_type) => quote! {
            pub fn #ch_name(&self) -> impl Iterator<Item =  #branch_type> {
                let start_sk = SyntaxKind::#open;
                let end_sk = SyntaxKind::#close;

                self.0
                    .children_with_tokens()
                    .skip_while(move |element| element.kind() != start_sk)
                    .skip(1)
                    .take_while(move |element| element.kind() != end_sk)
                    .filter_map(|element| element.into_node().and_then(#branch_type::cast))
            }
        },
        Rule::Group(check_fn) => quote! {
            pub fn #ch_name(&self) -> impl Iterator<Item =  SyntaxToken> {
                let start_sk = SyntaxKind::#open;
                let end_sk = SyntaxKind::#close;

                self.0
                    .children_with_tokens()
                    .skip_while(move |element| element.kind() != start_sk)
                    .skip(1)
                    .take_while(move |element| element.kind() != end_sk)
                    .filter_map(|element| element.into_token())
                    .filter(|token| token.kind().#check_fn())
            }
        },
        Rule::Delimited { .. } => {
            unreachable!("nested Delimited(..) is not supported")
        }
    }
}

pub fn expand(input: TokenStream) -> TokenStream {
    let config = syn::parse_macro_input!(input as Config);

    let mut expanded = quote! {};

    let mut enum_map: BTreeMap<String, Vec<(Ident, Ident)>> = BTreeMap::new();

    for node in &config.nodes {
        let node_name = &node.name;
        let group_str = node.group.as_ref().map(|g| g.to_string());
        let screaming_name = to_screaming_snake(&node_name.to_string());

        let node_kind = match &node.kind {
            Some(explicit_kind) => explicit_kind.clone(),
            None => match &group_str {
                Some(group_str) => {
                    let screaming_group = to_screaming_snake(group_str);
                    format_ident!("{}_{}", screaming_group, screaming_name)
                }
                None => format_ident!("{}", screaming_name),
            },
        };

        let struct_name = match &group_str {
            Some(group_str) => format_ident!("{}{}", group_str, node_name),
            None => node_name.clone(),
        };

        if let Some(group_str) = group_str {
            enum_map
                .entry(group_str)
                .or_default()
                .push((node_name.clone(), struct_name.clone()));
        }

        // Track seen identifiers to calculate correct index offsets per node definition context
        let mut leaf_counts: HashMap<String, usize> = HashMap::new();
        let mut branch_counts: HashMap<String, usize> = HashMap::new();
        let mut group_counts: HashMap<String, usize> = HashMap::new();

        let child_getters = node.children.iter().map(|child| {
            let ch_name = &child.name;
            let panic_msg = format!("Child '{}' is required", ch_name);

            match (&child.rule, &child.cardinality) {
                (Rule::Leaf(leaf_kind), Cardinality::Required) => {
                    let key = leaf_kind.to_string();
                    let idx = *leaf_counts
                        .entry(key.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(0);
                    quote! {
                        pub fn #ch_name(&self) -> SyntaxToken {
                            self.0
                                .children_with_tokens()
                                .filter_map(|c| c.into_token())
                                .filter(|t| t.kind() == SyntaxKind::#leaf_kind)
                                .nth(#idx)
                                .expect(#panic_msg)
                        }
                    }
                }
                (Rule::Branch(branch_type), Cardinality::Required) => {
                    let key = branch_type.to_string();
                    let idx = *branch_counts
                        .entry(key.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(0);
                    quote! {
                        pub fn #ch_name(&self) -> #branch_type {
                            self.0
                                .children()
                                .filter_map(#branch_type::cast)
                                .nth(#idx)
                                .expect(#panic_msg)
                        }
                    }
                }
                (Rule::Leaf(leaf_kind), Cardinality::Optional) => {
                    let key = leaf_kind.to_string();
                    let idx = *leaf_counts
                        .entry(key.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(0);
                    quote! {
                        pub fn #ch_name(&self) -> Option<SyntaxToken> {
                            self.0
                                .children_with_tokens()
                                .filter_map(|c| c.into_token())
                                .filter(|t| t.kind() == SyntaxKind::#leaf_kind)
                                .nth(#idx)
                        }
                    }
                }
                (Rule::Leaf(leaf_kind), Cardinality::Many) => quote! {
                    pub fn #ch_name(&self) -> impl Iterator<Item =  SyntaxToken> {
                        self.0
                            .children_with_tokens()
                            .filter_map(|c| c.into_token())
                            .filter(|t| t.kind() == SyntaxKind::#leaf_kind)
                    }
                },
                (Rule::Branch(branch_type), Cardinality::Optional) => {
                    let key = branch_type.to_string();
                    let idx = *branch_counts
                        .entry(key.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(0);
                    quote! {
                        pub fn #ch_name(&self) -> Option<#branch_type> {
                            self.0
                                .children()
                                .filter_map(#branch_type::cast)
                                .nth(#idx)
                        }
                    }
                }
                (Rule::Branch(branch_type), Cardinality::Many) => quote! {
                    pub fn #ch_name(&self) -> impl Iterator<Item =  #branch_type> {
                        self.0.children().filter_map(#branch_type::cast)
                    }
                },
                (Rule::Group(check_fn), Cardinality::Required) => {
                    let key = check_fn.to_string();
                    let idx = *group_counts
                        .entry(key.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(0);
                    quote! {
                        pub fn #ch_name(&self) -> SyntaxToken {
                            self.0
                                .children_with_tokens()
                                .filter_map(|c| c.into_token())
                                .filter(|t| t.kind().#check_fn())
                                .nth(#idx)
                                .expect(#panic_msg)
                        }
                    }
                }
                (Rule::Group(check_fn), Cardinality::Optional) => {
                    let key = check_fn.to_string();
                    let idx = *group_counts
                        .entry(key.clone())
                        .and_modify(|c| *c += 1)
                        .or_insert(0);
                    quote! {
                        pub fn #ch_name(&self) -> Option<SyntaxToken> {
                            self.0
                                .children_with_tokens()
                                .filter_map(|c| c.into_token())
                                .filter(|t| t.kind().#check_fn())
                                .nth(#idx)
                        }
                    }
                }
                (Rule::Group(check_fn), Cardinality::Many) => quote! {
                    pub fn #ch_name(&self) -> impl Iterator<Item =  SyntaxToken> {
                        self.0
                            .children_with_tokens()
                            .filter_map(|c| c.into_token())
                            .filter(|t| t.kind().#check_fn())
                    }
                },
                (
                    Rule::Delimited {
                        open,
                        close,
                        target,
                    },
                    _,
                ) => delimited_getter(ch_name, open, close, target),
            }
        });

        expanded.extend(quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update)]
            #[repr(transparent)]
            pub struct #struct_name(SyntaxNode);

            impl #struct_name {
                pub fn cast(node: SyntaxNode) -> Option<Self> {
                    (node.kind() == SyntaxKind::#node_kind).then(|| Self(node))
                }

                pub fn syntax(&self) -> &SyntaxNode {
                    &self.0
                }

                #(#child_getters)*
            }
        });
    }

    for (group_name, variants_data) in enum_map {
        let enum_ident = Ident::new(&group_name, proc_macro2::Span::call_site());

        let (variant_names, struct_names): (Vec<Ident>, Vec<Ident>) =
            variants_data.iter().cloned().unzip();

        let expected_kinds = config
            .nodes
            .iter()
            .filter(|node| matches!(&node.group, Some(g) if *g == group_name))
            .map(|node| match &node.kind {
                Some(explicit_kind) => explicit_kind.clone(),
                None => {
                    let screaming_group = to_screaming_snake(&group_name);
                    let screaming_name = to_screaming_snake(&node.name.to_string());
                    format_ident!("{}_{}", screaming_group, screaming_name)
                }
            });

        expanded.extend(quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash, salsa::Update)]
            pub enum #enum_ident {
                #( #variant_names(#struct_names) ),*
            }

            impl #enum_ident {
                pub fn cast(node: SyntaxNode) -> Option<Self> {
                    #(
                        if let Some(v) = #struct_names::cast(node.clone()) {
                            return Some(Self::#variant_names(v));
                        }
                    )*
                    None
                }

                pub fn is(kind: SyntaxKind) -> bool {
                    matches!(kind, #( SyntaxKind::#expected_kinds )|*)
                }
            }
        });
    }

    TokenStream::from(expanded)
}
