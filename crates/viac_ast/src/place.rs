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
use crate::node::{Node, NodeRef};
use viac_lexer::token::Token;
use viac_source::span::Span;

ast! {
    pub enum Place {
        This { },
        Symbol { token: Token },
        Dynamic {
            expr: NodeRef<Expr>,
            field: Token,
        },
        Static {
            expr: NodeRef<Expr>,
            field: Token,
        },
        Subscript {
            expr: NodeRef<Expr>,
            index: NodeRef<Expr>,
        },
    }
}
