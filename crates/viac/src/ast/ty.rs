/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    expr::Expr,
    macros::ast,
    node::{NodeRef, Nodes},
};
use crate::lexer::token::Token;

ast! {
    Ty {
        Builtin { token: Token },
        Optional { ty: NodeRef<Ty> },
        Array { ty: NodeRef<Ty> },
        Map {
            key: NodeRef<Ty>,
            value: NodeRef<Ty>,
        },
        Function {
            params: Nodes<Ty>,
            result: NodeRef<Ty>,
        },
        Union {
            lhs: NodeRef<Ty>,
            rhs: NodeRef<Ty>
        },
        Effect {
            lhs: NodeRef<Ty>,
            rhs: NodeRef<Ty>
        },
        TypeOf { expr: NodeRef<Expr> },
    }
}
