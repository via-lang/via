mod arena;
mod opcode;
mod token;

use proc_macro::TokenStream;

#[proc_macro_derive(Arena, attributes(allocator, interner))]
pub fn arena(input: TokenStream) -> TokenStream {
    arena::expand(input)
}

#[proc_macro_derive(Token, attributes(token_kind, prec))]
pub fn token(input: TokenStream) -> TokenStream {
    token::expand(input)
}

#[proc_macro_derive(Opcode, attributes(layout))]
pub fn opcode(input: TokenStream) -> TokenStream {
    opcode::expand(input)
}
