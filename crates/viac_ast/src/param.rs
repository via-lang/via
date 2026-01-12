/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::node::Node;
use crate::ty::Ty;
use viac_lexer::token::Token;
use viac_source::span;
use viac_source::span::Span;

#[derive(Debug, Eq, PartialEq)]
pub struct Param(pub Token, pub Box<Ty>);

impl Node for Param {
    fn span(&self) -> Span {
        span![self.0.span.begin, self.1.span().end]
    }
}
