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
            symbol: Token,
            ty: Option<Box<Ty>>,
            expr: Box<Expr>,
        },
        Function {
            symbol: Token,
            params: Vec<Param>,
            result: Option<Box<Ty>>,
            body: Body,
        },
        Use { symbol: Token },
        Type {
            symbol: Token,
            ty: Box<Ty>,
        },
        Const {
            symbol: Token,
            expr: Box<Expr>,
        },
        Struct {
            symbol: Token,
            body: Body<Decl>,
        },
        Import {
            path: Vec<Token>,
            alias: Option<Token>,
        },
    }
}
