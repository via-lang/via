/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{body::Body, expr::ExprId, macros::ast, param::Params, place::PlaceId, ty::TyId};
use crate::lexer::token::Token;

ast! {
    enum Value {
        None {},
        True {},
        False {},
        Integer {
            value: i64
        },
        Float {
            value: f64
        },
        String {
            value: std::string::String
        },
        Range {
            lhs: ExprId,
            rhs: ExprId,
            inclusive: bool,
        },
        Tuple {
            exprs: Vec<ExprId>
        },
        Array {
            exprs: Vec<ExprId>
        },
        Map {
            pairs: Vec<(ExprId, ExprId)>
        },
        Lambda {
            params: Params,
            result: Option<TyId>,
            body: Body,
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
        Borrow {
            expr: ExprId
        },
        Call {
            callee: ExprId,
            args: Vec<ExprId>
        },
        Try {
            expr: ExprId
        },
        Await {
            expr: ExprId
        },
        Type {
            ty: TyId
        },
        Cast {
            expr: ExprId,
            ty: TyId,
        },
        Break {
            expr: Option<ExprId>
        },
        Continue {},
        Raise {
            expr: ExprId
        },
        Return {
            expr: Option<ExprId>
        },
        If {
            cond: ExprId,
            body: Body,
            elseif: Vec<(ExprId, Body)>,
            alt: Option<Body>,
        },
        While {
            cond: ExprId,
            body: Body,
        },
        For {
            param: (Token, Option<TyId>),
            expr: ExprId,
            body: Body,
        },
        Import {
            path: Vec<Token>
        },
        Variable {
            name: Token, // TODO: This should be a pattern, not a DISGUSTING token
            ty: Option<TyId>,
            expr: Option<ExprId>,
        },
        Read {
            place: PlaceId
        }
    }
}
