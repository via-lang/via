/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::expr::Expr;
use crate::macros::ast;
use crate::node::Node;
use viac_lexer::token::Token;
use viac_source::span::Span;

ast! {
    pub enum Ty {
        Builtin { token: Token },
        Optional { ty: Box<Ty> },
        Array { ty: Box<Ty> },
        Map {
            key: Box<Ty>,
            value: Box<Ty>,
        },
        Function {
            params: Vec<Ty>,
            result: Box<Ty>,
        },
        Union {
            lhs: Box<Ty>,
            rhs: Box<Ty>
        },
        TypeOf { expr: Box<Expr> },
    }
}
