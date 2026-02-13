/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Ty;
use crate::{
    intern::{Interned, Interner},
    source::SourceSpan,
};

#[derive(Debug)]
pub enum TyOrigin {
    Source { span: SourceSpan },
    Infered { from: SourceSpan },
    Builtin,
}

pub struct TyContext<'cx> {
    tys: Interner<Ty<'cx>>,
}

impl<'cx> TyContext<'cx> {
    pub fn new() -> Self {
        Self {
            tys: Interner::default(),
        }
    }

    pub fn intern(&'cx mut self, ty: Ty<'cx>) -> Interned<'cx, Ty<'cx>> {
        self.tys.intern(ty)
    }
}
