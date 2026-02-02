/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    aux::{Nodes, ParamId},
    expr::ExprId,
    macros::ast,
    stmt::StmtId,
    ty::TyId,
};
use crate::lexer::token::Token;

ast! {
    Decl {
        Variable {
            symbol: Token,
            ty: Option<TyId>,
            expr: ExprId,
        },
        Function {
            symbol: Token,
            params: Vec<ParamId>,
            result: Option<TyId>,
            body: Nodes<StmtId>,
        },
        Use { symbol: Token },
        Type {
            symbol: Token,
            ty: TyId,
        },
        Const {
            symbol: Token,
            expr: ExprId,
        },
        Struct {
            symbol: Token,
            body: Nodes<DeclId>,
        },
        Import {
            path: Vec<Token>,
            alias: Option<Token>,
        },
    }
}
