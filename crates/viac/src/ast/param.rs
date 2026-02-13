/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use bitflags::bitflags;

use crate::{ast::ty::TyId, lexer::token::Token, source::SourceSpan};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ParamQuals: u8 {
        const None = 0;
        const Borrow = 1 << 1;
        const Mutable = 1 << 2;
    }
}

#[derive(Debug, Clone)]
pub struct ThisParam {
    pub quals: ParamQuals,
    pub span: SourceSpan,
}

#[derive(Debug, Clone)]
pub enum ParamKind {
    Named {
        quals: ParamQuals,
        name: Token,
        ty: TyId,
    },
    Anonymous {
        ty: TyId,
    },
}

#[derive(Debug, Clone)]
pub struct Param {
    pub kind: ParamKind,
    pub span: SourceSpan,
}

#[derive(Debug, Clone)]
pub struct Params {
    pub this: Option<ThisParam>,
    pub inner: Vec<Param>,
    pub span: SourceSpan,
}
