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
use viac_source::span::Span;

ast! {
    pub enum Control {
        Break {},
        Continue {},
        Return {
            expr: Option<Box<Expr>>,
        },
        Raise {
            expr: Box<Expr>,
        },
        If {
            cond: Box<Expr>,
            body: Body,
            elifs: Vec<(Expr, Body)>,
            els: Option<Body>,
        },
        While {
            cond: Box<Expr>,
            body: Body,
        },
        For {
            param: (Token, Option<Box<Ty>>),
            expr: Box<Expr>,
            body: Body,
        },
        Assign {
            op: Token,
            lhs: Box<Expr>,
            rhs: Box<Expr>,
        },
    }
}
