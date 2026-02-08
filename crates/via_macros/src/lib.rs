/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

mod prec_data;

use proc_macro::TokenStream;

#[proc_macro_derive(PrecData, attributes(prec_data, prec))]
pub fn prec_data(input: TokenStream) -> TokenStream {
    prec_data::expand(input)
}
