/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use delegate::delegate;

use super::ty::Ty;
use crate::intern::Interner;

pub struct SemContext<'sem> {
    tys: Interner<Ty<'sem>>,
}

impl<'sem> SemContext<'sem> {
    pub fn new() -> Self {
        Self {
            tys: Interner::new(),
        }
    }

    delegate! {
        to self.tys {
            #[call(intern)]
            pub fn intern_ty(&mut self, ty: Ty<'sem>);
        }
    }
}
