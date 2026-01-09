/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::macros::ast;
use super::{expr::Expr, stmt::Stmt, typ::Type};
use crate::compiler::{lexer::token::Token, source::Span};

ast! {
    pub enum Control {
        Break { span: Span },
        Continue { span: Span },
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
}

impl Control {
    pub fn span(&self) -> &Span {
        match self {
            Self::Break(c) => &c.span,
            Self::Continue(c) => &c.span,
            Self::Return(c) => &c.span,
            Self::Raise(c) => &c.span,
            Self::If(c) => &c.span,
            Self::While(c) => &c.span,
            Self::WhileNot(c) => &c.span,
            Self::For(c) => &c.span,
            Self::ForEach(c) => &c.span,
        }
    }
}
