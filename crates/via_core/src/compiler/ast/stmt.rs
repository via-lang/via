/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{control::Control, decl::Decl, expr::Expr};
use crate::compiler::source::Span;

#[derive(Debug)]
pub enum Stmt {
    Empty(Span),
    Decl(Decl),
    Control(Control),
    Expr(Expr),
}

impl Stmt {
    pub fn span(&self) -> &Span {
        match self {
            Self::Empty(span) => span,
            Self::Decl(decl) => decl.span(),
            Self::Control(ctrl) => ctrl.span(),
            Self::Expr(expr) => expr.span(),
        }
    }
}
