/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    attr::AttrId,
    aux::{Nodes, ParamId},
    expr::ExprId,
    macros::ast,
    place::PlaceId,
    stmt::StmtId,
    ty::TyId,
};
use crate::lexer::token::Token;

ast! {
    Value {
        None {  },
        True {  },
        False {  },
        Integer { value: i64 },
        Float { value: f64 },
        String { string: std::string::String },
        Range {
            lhs: ExprId,
            rhs: ExprId,
            inclusive: bool,
        },
        Tuple { exprs: Vec<ExprId> },
        Array { exprs: Vec<ExprId> },
        Map { pairs: Vec<(ExprId, ExprId)> },
        Lambda {
            params: Vec<ParamId>,
            result: Option<TyId>,
            body: Nodes<StmtId>,
        },
        Unary {
            op: Token,
            expr: ExprId,
        },
        Binary {
            op: Token,
            lhs: ExprId,
            rhs: ExprId,
        },
        Reference { expr: ExprId },
        Ternary {
            cond: ExprId,
            iftrue: ExprId,
            iffalse: ExprId,
        },
        Call {
            callee: ExprId,
            args: Vec<ExprId>
        },
        Cast {
            expr: ExprId,
            ty: TyId,
        },
        Try { expr: ExprId },
        Await { expr: ExprId },
        Type { ty: TyId },
        Attr { attr: AttrId },
        Read { place: PlaceId },
    }
}
