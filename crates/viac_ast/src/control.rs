/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::expr::Expr;
use crate::extra::Body;
use crate::macros::ast;
use crate::node::{Node, NodeRef};
use crate::ty::Ty;
use viac_lexer::token::Token;

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
