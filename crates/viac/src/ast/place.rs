/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{expr::Expr, macros::ast, node::NodeRef};
use crate::lexer::token::Token;

ast! {
    Place {
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
