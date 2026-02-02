/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Id, ty::TyId};
use crate::{ast::macros::ast, lexer::token::Token, source::SourceSpan};

#[derive(Debug, Clone)]
pub struct Nodes<I: Id> {
    pub inner: Vec<I>,
    pub span: SourceSpan,
}

ast! {
    Param {
        name: Token,
        ty: TyId,
    }
}

impl PartialEq for Param {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty
    }
}
