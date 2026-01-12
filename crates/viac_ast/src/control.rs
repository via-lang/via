/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::body::Body;
use crate::expr::Expr;
use crate::macros::ast;
use crate::node::Node;
use crate::ty::Ty;
use viac_lexer::token::Token;
use viac_source::span;
use viac_source::span::Span;

ast! {
    pub enum Control {
        Assign {
            op: Token,
            lhs: Box<Expr>,
            rhs: Box<Expr>,
        },
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
            body: Body,
            elifs: Vec<(Expr, Body)>,
            els: Option<Body>,
        },
        While {
            span: Span,
            cond: Box<Expr>,
            body: Body,
        },
        For {
            span: Span,
            param: (Token, Option<Box<Ty>>),
            expr: Box<Expr>,
            body: Body,
        },
    }
}

impl Node for Control {
    fn span(&self) -> Span {
        match self {
            Self::Assign(c) => span![c.lhs.span().begin, c.rhs.span().end],
            Self::Break(c) => c.span,
            Self::Continue(c) => c.span,
            Self::Return(c) => c.span,
            Self::Raise(c) => c.span,
            Self::If(c) => c.span,
            Self::While(c) => c.span,
            Self::For(c) => c.span,
        }
    }
}
