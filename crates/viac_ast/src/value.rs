/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::attr;
use crate::expr::Expr;
use crate::extra::{Body, NodeList, Param};
use crate::macros::ast;
use crate::node::{Node, NodeRef};
use crate::place::Place;
use crate::ty::Ty;
use viac_lexer::token::Token;

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
        Reference {
            strong: bool,
            mutable: bool,
            expr: NodeRef<Expr>,
        },
        Ternary {
            cond: NodeRef<Expr>,
            iftrue: NodeRef<Expr>,
            iffalse: NodeRef<Expr>,
        },
        Cast {
            expr: NodeRef<Expr>,
            ty: NodeRef<Ty>,
        },
        Type { ty: NodeRef<Ty> },
        Attr { attr: NodeRef<attr::Attr> },
        Read { place: NodeRef<Place> },
    }
}
