/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Parser;
use super::error::Result;
use crate::ast::node::Ast;
use crate::source::Source;

pub mod attr;
pub mod expr;
pub mod ty;

pub fn parse<T: Ast>(src: &str, f: impl FnOnce(&mut Parser) -> Result<T>) -> Result<T> {
    let source = Source::new(src.to_string());
    let tokens = crate::lexer::tokenize(&source);
    let mut parser = Parser::new(&source, &tokens);
    f(&mut parser)
}
