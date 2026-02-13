/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{expr::ExprId, macros::ast};
use crate::lexer::token::Token;

ast! {
    enum Place {
        This {},
        Symbol { symbol: String },
        Dynamic {
            expr: ExprId,
            field: Token,
        },
        Static {
            expr: ExprId,
            field: Token,
        },
        Subscript {
            expr: ExprId,
            index: ExprId,
        },
    }
}
