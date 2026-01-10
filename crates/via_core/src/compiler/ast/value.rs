/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::attr;
use super::macros::ast;
use super::{expr::Expr, place::Place, stmt::Stmt, typ::Type};
use crate::compiler::{lexer::token::Token, source::Span};
use bitflags::bitflags;

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
        Group {
            span: Span,
            expr: Box<Expr>,
        },
        None { span: Span },
        True { span: Span },
        False { span: Span },
        Integer { span: Span, token: Token },
        Float { span: Span, token: Token },
        String { span: Span, token: Token },
        Tuple {
            span: Span,
            exprs: Vec<Expr>,
        },
        Array {
            span: Span,
            exprs: Vec<Expr>,
        },
        Map {
            span: Span,
            pairs: Vec<(Expr, Expr)>,
        },
        Lambda {
            span: Span,
            params: Vec<(Token, Type)>,
            result: Option<Box<Expr>>,
            body: Vec<Stmt>,
        },
        Unary {
            span: Span,
            op: Token,
            expr: Box<Expr>,
        },
        Binary {
            span: Span,
            op: Token,
            lhs: Box<Expr>,
            rhs: Box<Expr>,
        },
        Reference {
            span: Span,
            flags: ReferenceFlags,
            expr: Box<Expr>,
        },
        Ternary {
            span: Span,
            cond: Box<Expr>,
            iftrue: Box<Expr>,
            iffalse: Box<Expr>,
        },
        Cast {
            span: Span,
            expr: Box<Expr>,
            typ: Box<Type>,
        },
        Attr {
            span: Span,
            attr: attr::Attr,
        },
        Read { place: Place },
    }
}

impl Value {
    pub fn span(&self) -> &Span {
        match self {
            Value::Group(v) => &v.span,
            Value::None(v) => &v.span,
            Value::True(v) => &v.span,
            Value::False(v) => &v.span,
            Value::Integer(v) => &v.span,
            Value::Float(v) => &v.span,
            Value::String(v) => &v.span,
            Value::Tuple(v) => &v.span,
            Value::Array(v) => &v.span,
            Value::Map(v) => &v.span,
            Value::Lambda(v) => &v.span,
            Value::Unary(v) => &v.span,
            Value::Binary(v) => &v.span,
            Value::Reference(v) => &v.span,
            Value::Ternary(v) => &v.span,
            Value::Cast(v) => &v.span,
            Value::Attr(v) => &v.span,
            Value::Read(v) => v.place.span(),
        }
    }
}
