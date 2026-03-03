/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

mod diagnostic;
mod opcode;
mod token;

use proc_macro::TokenStream;

#[proc_macro_derive(Token, attributes(token_kind, prec))]
pub fn token(input: TokenStream) -> TokenStream {
    token::expand(input)
}

#[proc_macro_derive(Opcode, attributes(layout))]
pub fn opcode(input: TokenStream) -> TokenStream {
    opcode::expand(input)
}

#[proc_macro_derive(Diagnostic, attributes(severity, message))]
pub fn diagnostic(input: TokenStream) -> TokenStream {
    diagnostic::expand(input)
}
