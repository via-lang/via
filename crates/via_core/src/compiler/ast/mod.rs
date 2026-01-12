/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod attr;
pub mod control;
pub mod decl;
pub mod expr;
pub mod macros;
pub mod place;
pub mod stmt;
pub mod ty;
pub mod value;

use crate::compiler::{
    lexer::token::Token,
    source::{Span, span},
};
use stmt::Stmt;
use ty::Ty;

pub trait Node {
    fn span(&self) -> Span;
}

#[derive(Debug)]
pub struct Body<T: Node = Stmt>(pub Span, pub Vec<T>);

#[derive(Debug)]
pub struct Parameter(pub Token, pub Box<Ty>);

impl Node for Body {
    fn span(&self) -> Span {
        self.0
    }
}

impl Node for Parameter {
    fn span(&self) -> Span {
        span![self.0.span.begin, self.1.span().end]
    }
}
