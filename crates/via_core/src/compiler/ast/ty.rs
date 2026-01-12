/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Node, expr::Expr, macros::ast};
use crate::compiler::{lexer::token::Token, source::Span};

ast! {
    pub enum Ty {
        Builtin {
            span: Span,
            token: Token,
        },
        Optional {
            span: Span,
            ty: Box<Ty>,
        },
        Union {
            span: Span,
            lhs: Box<Ty>,
            rhs: Box<Ty>
        },
        Array {
            span: Span,
            ty: Box<Ty>,
        },
        Map {
            span: Span,
            key: Box<Ty>,
            value: Box<Ty>,
        },
        Function {
            span: Span,
            params: Vec<Ty>,
            result: Box<Ty>,
        },
        TypeOf {
            span: Span,
            expr: Box<Expr>,
        },
    }
}

impl Node for Ty {
    fn span(&self) -> Span {
        match self {
            Self::Builtin(t) => t.span,
            Self::Optional(t) => t.span,
            Self::Union(t) => t.span,
            Self::Array(t) => t.span,
            Self::Map(t) => t.span,
            Self::Function(t) => t.span,
            Self::TypeOf(t) => t.span,
        }
    }
}
