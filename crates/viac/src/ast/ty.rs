/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{expr::ExprId, macros::ast, param::Params};
use crate::lexer::token::Token;

ast! {
    enum Ty {
        Builtin {
            token: Token
        },
        Optional {
            ty: TyId
        },
        Tuple {
            tys: Vec<TyId>
        },
        Array {
            ty: TyId
        },
        Map {
            key: TyId,
            value: TyId,
        },
        Function {
            params: Params,
            result: TyId,
        },
        Union {
            lhs: TyId,
            rhs: TyId
        },
        Raise {
            lhs: TyId,
            rhs: TyId
        },
        Type {
            expr: ExprId
        },
    }
}
