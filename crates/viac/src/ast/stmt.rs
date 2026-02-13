/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{body::Body, expr::ExprId, macros::ast, param::Params, ty::TyId};
use crate::{lexer::token::Token, source::SourceSpan};

#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Module { span: SourceSpan },
    Private { span: SourceSpan },
}

ast! {
    enum Stmt {
        Discard {
            expr: ExprId
        },
        Consume {
            expr: ExprId
        },
        DefineConst {
            name: Token,
            ty: TyId,
            expr: ExprId,
        },
        DefineFn {
            name: Token,
            params: Params,
            result: Option<TyId>,
            body: Body,
        },
        DefineType {
            name: Token,
            ty: TyId,
        },
    }
}
