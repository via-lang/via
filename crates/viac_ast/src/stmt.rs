/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::control::Control;
use crate::decl::Decl;
use crate::expr::Expr;
use crate::node::Node;
use viac_source::span::Span;

#[derive(Debug, Eq, PartialEq)]
pub enum Stmt {
    Empty(Span),
    Decl(Decl),
    Control(Control),
    Expr(Expr),
}

impl Node for Stmt {
    fn span(&self) -> Span {
        match self {
            Self::Empty(span) => *span,
            Self::Decl(decl) => decl.span(),
            Self::Control(ctrl) => ctrl.span(),
            Self::Expr(expr) => expr.span(),
        }
    }
}
