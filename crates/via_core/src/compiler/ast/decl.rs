/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::macros::ast;
use super::{expr::Expr, stmt::Stmt, typ};
use crate::compiler::{lexer::token::Token, source::Span};

ast! {
    pub enum Decl {
        Variable {
            span: Span,
            symbol: Token,
            typ: Option<Box<typ::Type>>,
            expr: Box<Expr>,
        },
        Function {
            span: Span,
            symbol: Token,
            params: Vec<(Token, typ::Type)>,
            result: Option<Box<typ::Type>>,
            body: Vec<Stmt>,
        },
        Use {
            span: Span,
            symbol: Token,
        },
        Type {
            span: Span,
            symbol: Token,
            typ: Box<typ::Type>,
        },
        Const {
            span: Span,
            symbol: Token,
            expr: Box<Expr>,
        },
        Struct {
            span: Span,
            symbol: Token,
            fields: Vec<Decl>,
        },
        Import {
            span: Span,
            path: Vec<Token>,
            alias: Option<Token>,
        },
    }
}

impl Decl {
    pub fn span(&self) -> &Span {
        match self {
            Self::Variable(d) => &d.span,
            Self::Function(d) => &d.span,
            Self::Use(d) => &d.span,
            Self::Type(d) => &d.span,
            Self::Const(d) => &d.span,
            Self::Struct(d) => &d.span,
            Self::Import(d) => &d.span,
        }
    }
}
