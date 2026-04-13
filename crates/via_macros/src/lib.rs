mod access;
mod arena;
mod id;
mod opcode;
mod token;

use proc_macro::TokenStream;

#[proc_macro_derive(Id, attributes(id))]
pub fn id(input: TokenStream) -> TokenStream {
    id::expand(input)
}

#[proc_macro_derive(Arena, attributes(allocator, interner))]
pub fn arena(input: TokenStream) -> TokenStream {
    arena::expand(input)
}

#[proc_macro_derive(Access, attributes(getter))]
pub fn access(input: TokenStream) -> TokenStream {
    access::expand(input)
}

#[proc_macro_derive(Token, attributes(token_kind, prec, keyword, operator))]
pub fn token(input: TokenStream) -> TokenStream {
    token::expand(input)
}

#[proc_macro_derive(Opcode, attributes(layout))]
pub fn opcode(input: TokenStream) -> TokenStream {
    opcode::expand(input)
}
