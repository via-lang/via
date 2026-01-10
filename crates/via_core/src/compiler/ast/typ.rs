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
    pub enum Type {
        Builtin {
            span: Span,
            token: Token,
        },
        Optional {
            span: Span,
            typ: Box<Type>,
        },
        Union {
            span: Span,
            lhs: Box<Type>,
            rhs: Box<Type>
        },
        Array {
            span: Span,
            typ: Box<Type>,
        },
        Map {
            span: Span,
            key: Box<Type>,
            value: Box<Type>,
        },
        Function {
            span: Span,
            params: Vec<Type>,
            result: Box<Type>,
        },
        TypeOf {
            span: Span,
            expr: Box<Expr>,
        },
    }
}

impl Node for Type {
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
