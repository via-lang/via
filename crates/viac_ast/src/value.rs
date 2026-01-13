/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::attr;
use crate::body::Body;
use crate::expr::Expr;
use crate::macros::ast;
use crate::node::Node;
use crate::param::Param;
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
        Group { expr: Box<Expr> },
        None {  },
        True {  },
        False {  },
        Integer { token: Token },
        Float { token: Token },
        String { token: Token },
        Range {
            lhs: Box<Expr>,
            rhs: Box<Expr>,
            inclusive: bool,
        },
        Tuple { exprs: Vec<Expr> },
        Array { exprs: Vec<Expr> },
        Map { pairs: Vec<(Expr, Expr)> },
        Lambda {
            params: Vec<Param>,
            result: Option<Box<Ty>>,
            body: Body,
        },
        Unary {
            op: Token,
            expr: Box<Expr>,
        },
        Binary {
            op: Token,
            lhs: Box<Expr>,
            rhs: Box<Expr>,
        },
        Reference {
            flags: ReferenceFlags,
            expr: Box<Expr>,
        },
        Ternary {
            cond: Box<Expr>,
            iftrue: Box<Expr>,
            iffalse: Box<Expr>,
        },
        Cast {
            expr: Box<Expr>,
            typ: Box<Ty>,
        },
        Attr { attr: attr::Attr },
        Read { place: Place },
    }
}
