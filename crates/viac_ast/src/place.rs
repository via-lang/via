/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::expr::Expr;
use crate::macros::ast;
use crate::node::Node;
use viac_lexer::token::Token;
use viac_source::span::Span;

ast! {
    pub enum Place {
        This { span: Span },
        Symbol { token: Token },
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

impl Node for Place {
    fn span(&self) -> Span {
        match self {
            Self::This(p) => p.span,
            Self::Symbol(p) => p.token.span,
            Self::Dynamic(p) => p.span,
            Self::Static(p) => p.span,
            Self::Subscript(p) => p.span,
        }
    }
}
