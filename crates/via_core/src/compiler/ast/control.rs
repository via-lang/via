/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{expr::Expr, stmt::Stmt, typ::Type};
use crate::compiler::{lexer::token::Token, source::Span};

#[derive(Debug)]
pub enum Control {
    Break(Span),
    Continue(Span),
    Return {
        span: Span,
        expr: Option<Box<Expr>>,
    },
    Raise {
        span: Span,
        expr: Box<Expr>,
    },
    If {
        span: Span,
        cond: Box<Expr>,
        body: Vec<Stmt>,
        elifs: Vec<(Expr, Vec<Stmt>)>,
        els: Option<Vec<Stmt>>,
    },
    While {
        span: Span,
        cond: Box<Expr>,
        body: Vec<Stmt>,
    },
    WhileNot {
        span: Span,
        cond: Box<Expr>,
        body: Vec<Stmt>,
    },
    For {
        span: Span,
        cond: Box<Expr>,
        action: Box<Expr>,
        body: Vec<Stmt>,
    },
    ForEach {
        span: Span,
        param: (Token, Option<Box<Type>>),
        expr: Box<Expr>,
        body: Vec<Stmt>,
    },
}

impl Control {
    pub fn span(&self) -> &Span {
        match self {
            Self::Break(span) | Control::Continue(span) => span,
            Self::Return { span, .. }
            | Self::Raise { span, .. }
            | Self::If { span, .. }
            | Self::While { span, .. }
            | Self::WhileNot { span, .. }
            | Self::For { span, .. }
            | Self::ForEach { span, .. } => span,
        }
    }
}
