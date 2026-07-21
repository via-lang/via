mod arena;
mod def_tree;
mod id;
mod opcode;
mod operation;
mod syntax;
mod syntax_tree;

use proc_macro::TokenStream;

#[proc_macro_derive(Id, attributes(id))]
pub fn id(input: TokenStream) -> TokenStream {
    id::expand(input)
}

#[proc_macro_derive(Arena, attributes(allocator, interner))]
pub fn arena(input: TokenStream) -> TokenStream {
    arena::expand(input)
}

#[proc_macro_derive(Syntax, attributes(keyword, operator))]
pub fn syntax(input: TokenStream) -> TokenStream {
    syntax::expand(input)
}

#[proc_macro]
pub fn syntax_tree(input: TokenStream) -> TokenStream {
    syntax_tree::expand(input)
}

#[proc_macro]
pub fn def_tree(input: TokenStream) -> TokenStream {
    def_tree::expand(input)
}

#[proc_macro_derive(Opcode, attributes(layout))]
pub fn opcode(input: TokenStream) -> TokenStream {
    opcode::expand(input)
}

#[proc_macro_derive(Operation, attributes(from, trait_info))]
pub fn operation(input: TokenStream) -> TokenStream {
    operation::expand(input)
}
