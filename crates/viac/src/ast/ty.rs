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
    Ty {
        Builtin { token: Token },
        Optional { ty: TyId },
        Array { ty: TyId },
        Map {
            key: TyId,
            value: TyId,
        },
        Function {
            params: Vec<TyId>,
            result: TyId,
        },
        Union {
            lhs: TyId,
            rhs: TyId
        },
        Effect {
            lhs: TyId,
            rhs: TyId
        },
        TypeOf { expr: ExprId },
    }
}
