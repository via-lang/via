/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{aux::Nodes, expr::ExprId, macros::ast, stmt::StmtId, ty::TyId};
use crate::lexer::token::Token;

ast! {
    Control {
        Break {},
        Continue {},
        Return {
            expr: Option<ExprId>,
        },
        Raise {
            expr: ExprId,
        },
        If {
            cond: ExprId,
            body: Nodes<StmtId>,
            elseif: Vec<(ExprId, Nodes<StmtId>)>,
            alt: Option<Nodes<StmtId>>,
        },
        While {
            cond: ExprId,
            body: Nodes<StmtId>,
        },
        For {
            param: (Token, Option<TyId>),
            expr: ExprId,
            body: Nodes<StmtId>,
        },
        Assign {
            op: Token,
            lhs: ExprId,
            rhs: ExprId,
        },
    }
}
