/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::attr;
use super::expr::Expr;
use super::extra::{Body, NodeList, Param};
use super::macros::ast;
use super::node::{Node, NodeRef};
use super::place::Place;
use super::ty::Ty;
use crate::lexer::token::Token;

ast! {
    pub enum Value {
        None {  },
        True {  },
        False {  },
        Integer { token: Token },
        Float { token: Token },
        String { token: Token },
        Range {
            lhs: NodeRef<Expr>,
            rhs: NodeRef<Expr>,
            inclusive: bool,
        },
        Tuple { exprs: NodeList<Expr> },
        Array { exprs: NodeList<Expr> },
        Map { pairs: Vec<(Node<Expr>, Node<Expr>)> },
        Lambda {
            params: NodeList<Param>,
            result: Option<NodeRef<Ty>>,
            body: Body,
        },
        Unary {
            op: Token,
            expr: NodeRef<Expr>,
        },
        Binary {
            op: Token,
            lhs: NodeRef<Expr>,
            rhs: NodeRef<Expr>,
        },
        Reference { expr: NodeRef<Expr> },
        Ternary {
            cond: NodeRef<Expr>,
            iftrue: NodeRef<Expr>,
            iffalse: NodeRef<Expr>,
        },
        Call {
            callee: NodeRef<Expr>,
            args: NodeList<Expr>
        },
        Cast {
            expr: NodeRef<Expr>,
            ty: NodeRef<Ty>,
        },
        Try { expr: NodeRef<Expr> },
        Await { expr: NodeRef<Expr> },
        Type { ty: NodeRef<Ty> },
        Attr { attr: NodeRef<attr::Attr> },
        Read { place: NodeRef<Place> },
    }
}
