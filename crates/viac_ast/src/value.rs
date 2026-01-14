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
use crate::extra::{Body, Param};
use crate::macros::ast;
use crate::node::{Node, NodeRef};
use crate::place::Place;
use crate::ty::Ty;
use bitflags::bitflags;
use viac_lexer::token::Token;
use viac_source::span::Span;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ReferenceFlags(u8);

bitflags! {
    impl ReferenceFlags: u8 {
        const None = 0b00;
        const Mutable = 0b10;
        const Strong = 0b01;
    }
}

ast! {
    pub enum Value {
        Group { expr: NodeRef<Expr> },
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
        Tuple { exprs: Vec<Node<Expr>> },
        Array { exprs: Vec<Node<Expr>> },
        Map { pairs: Vec<(Node<Expr>, Node<Expr>)> },
        Lambda {
            params: Vec<Node<Param>>,
            result: Option<NodeRef<Ty>>,
            body: Node<Body>,
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
            flags: ReferenceFlags,
            expr: NodeRef<Expr>,
        },
        Ternary {
            cond: NodeRef<Expr>,
            iftrue: NodeRef<Expr>,
            iffalse: NodeRef<Expr>,
        },
        Cast {
            expr: NodeRef<Expr>,
            typ: NodeRef<Ty>,
        },
        Type { ty: NodeRef<Ty> },
        Attr { attr: NodeRef<attr::Attr> },
        Read { place: NodeRef<Place> },
    }
}
