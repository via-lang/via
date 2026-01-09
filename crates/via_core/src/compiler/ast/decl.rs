/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{expr::Expr, stmt::Stmt, typ::Type};
use crate::compiler::{lexer::token::Token, source::Span};

#[derive(Debug)]
pub enum Decl {
    Variable {
        span: Span,
        symbol: Token,
        typ: Option<Box<Type>>,
        expr: Box<Expr>,
    },
    Function {
        span: Span,
        symbol: Token,
        params: Vec<(Token, Type)>,
        result: Option<Box<Type>>,
        body: Vec<Stmt>,
    },
    Use {
        span: Span,
        symbol: Token,
    },
    Type {
        span: Span,
        symbol: Token,
        typ: Box<Type>,
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

impl Decl {
    pub fn span(&self) -> &Span {
        match self {
            Self::Variable { span, .. }
            | Self::Function { span, .. }
            | Self::Use { span, .. }
            | Self::Type { span, .. }
            | Self::Const { span, .. }
            | Self::Struct { span, .. }
            | Self::Import { span, .. } => span,
        }
    }
}
