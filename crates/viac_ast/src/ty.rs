/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::expr::Expr;
use crate::extra::NodeList;
use crate::macros::ast;
use crate::node::NodeRef;
use viac_lexer::token::Token;

ast! {
    pub enum Ty {
        Builtin { token: Token },
        Optional { ty: NodeRef<Ty> },
        Array { ty: NodeRef<Ty> },
        Map {
            key: NodeRef<Ty>,
            value: NodeRef<Ty>,
        },
        Function {
            params: NodeList<Ty>,
            result: NodeRef<Ty>,
        },
        Union {
            lhs: NodeRef<Ty>,
            rhs: NodeRef<Ty>
        },
        TypeOf { expr: NodeRef<Expr> },
    }
}
