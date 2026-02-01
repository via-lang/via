/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::{ast::node::Marker, lexer::Lexer, source::SourceBuf};

pub mod attr;
pub mod expr;
pub mod ty;

pub fn parse<T: Marker>(src: &str, f: impl FnOnce(&mut Parser) -> Result<T>) -> Result<T> {
    let src = SourceBuf::new("<test>", src);
    let tt = Lexer::new(&src).tokenize();
    let mut parser = Parser::new(&src, &tt);
    f(&mut parser)
}
