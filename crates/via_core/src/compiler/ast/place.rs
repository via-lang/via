/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::expr::Expr;
use crate::compiler::{lexer::token::Token, source::Span};

#[derive(Debug)]
pub enum Place {
    This(Span),
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

impl Place {
    pub fn span(&self) -> &Span {
        match self {
            Self::This(span) => span,
            Self::Symbol { span, .. }
            | Self::Dynamic { span, .. }
            | Self::Static { span, .. }
            | Self::Subscript { span, .. } => span,
        }
    }
}
