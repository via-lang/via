/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Body, Node, Parameter, expr::Expr, macros::ast, ty};
use crate::compiler::{lexer::token::Token, source::Span};

ast! {
    pub enum Decl {
        Variable {
            span: Span,
            symbol: Token,
            ty: Option<Box<ty::Ty>>,
            expr: Box<Expr>,
        },
        Function {
            span: Span,
            symbol: Token,
            params: Vec<Parameter>,
            result: Option<Box<ty::Ty>>,
            body: Body,
        },
        Use {
            span: Span,
            symbol: Token,
        },
        Ty {
            span: Span,
            symbol: Token,
            ty: Box<ty::Ty>,
        },
        Const {
            span: Span,
            symbol: Token,
            expr: Box<Expr>,
        },
        Struct {
            span: Span,
            symbol: Token,
            body: Body<Decl>,
        },
        Import {
            span: Span,
            path: Vec<Token>,
            alias: Option<Token>,
        },
    }
}

impl Node for Decl {
    fn span(&self) -> Span {
        match self {
            Self::Variable(d) => d.span,
            Self::Function(d) => d.span,
            Self::Use(d) => d.span,
            Self::Ty(d) => d.span,
            Self::Const(d) => d.span,
            Self::Struct(d) => d.span,
            Self::Import(d) => d.span,
        }
    }
}
