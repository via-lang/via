/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::expr::Expr;
use super::extra::Body;
use super::macros::ast;
use super::node::{Node, NodeRef};
use super::ty::Ty;
use crate::lexer::token::Token;

ast! {
    pub enum Control {
        Break {},
        Continue {},
        Return {
            expr: Option<NodeRef<Expr>>,
        },
        Raise {
            expr: NodeRef<Expr>,
        },
        If {
            cond: NodeRef<Expr>,
            body: Body,
            elseif: Vec<(Node<Expr>, Body)>,
            else_body: Option<Body>,
        },
        While {
            cond: NodeRef<Expr>,
            body: Body,
        },
        For {
            param: (Token, Option<NodeRef<Ty>>),
            expr: NodeRef<Expr>,
            body: Body,
        },
        Assign {
            op: Token,
            lhs: NodeRef<Expr>,
            rhs: NodeRef<Expr>,
        },
    }
}
