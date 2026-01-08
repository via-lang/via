/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{attr::Attr, expr::Expr, place::Place, stmt::Stmt, typ::Type};
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

#[derive(Debug)]
pub enum Value {
    Group {
        span: Span,
        expr: Box<Expr>,
    },
    None(Span),
    True(Span),
    False(Span),
    Constant {
        span: Span,
        token: Token,
    },
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
        attr: Attr,
    },
    Read(Place),
}

impl Value {
    pub fn span(&self) -> &Span {
        match self {
            Value::Group { span, .. }
            | Value::None(span)
            | Value::True(span)
            | Value::False(span)
            | Value::Constant { span, .. }
            | Value::Tuple { span, .. }
            | Value::Array { span, .. }
            | Value::Map { span, .. }
            | Value::Lambda { span, .. }
            | Value::Unary { span, .. }
            | Value::Binary { span, .. }
            | Value::Reference { span, .. }
            | Value::Ternary { span, .. }
            | Value::Cast { span, .. }
            | Value::Attr { span, .. } => span,
            Value::Read(place) => place.span(),
        }
    }
}
