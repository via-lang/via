/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::body::Body;
use crate::expr::Expr;
use crate::macros::ast;
use crate::node::Node;
use crate::param::Param;
use crate::ty::Ty;
use viac_lexer::token::Token;
use viac_source::span::Span;

ast! {
    pub enum Decl {
        Variable {
            span: Span,
            symbol: Token,
            ty: Option<Box<Ty>>,
            expr: Box<Expr>,
        },
        Function {
            span: Span,
            symbol: Token,
            params: Vec<Param>,
            result: Option<Box<Ty>>,
            body: Body,
        },
        Use {
            span: Span,
            symbol: Token,
        },
        Type {
            span: Span,
            symbol: Token,
            ty: Box<Ty>,
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
            Self::Type(d) => d.span,
            Self::Const(d) => d.span,
            Self::Struct(d) => d.span,
            Self::Import(d) => d.span,
        }
    }
}
