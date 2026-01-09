/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::expr::Expr;
use super::macros::ast;
use crate::compiler::{lexer::token::Token, source::Span};

ast! {
    pub enum Place {
        This { span: Span },
        Symbol {
            span: Span,
            token: Token,
        },
        Dynamic {
            span: Span,
            expr: Box<Expr>,
            field: Token,
        },
        Static {
            span: Span,
            expr: Box<Expr>,
            field: Token,
        },
        Subscript {
            span: Span,
            expr: Box<Expr>,
            index: Box<Expr>,
        },
    }
}

impl Place {
    pub fn span(&self) -> &Span {
        match self {
            Self::This(p) => &p.span,
            Self::Symbol(p) => &p.span,
            Self::Dynamic(p) => &p.span,
            Self::Static(p) => &p.span,
            Self::Subscript(p) => &p.span,
        }
    }
}
