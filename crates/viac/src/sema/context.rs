/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use delegate::delegate;

use super::Def;
use crate::intern::{Interned, Interner};

pub struct SemContext<'cx> {
    defs: Interner<Def<'cx>>,
}

impl<'cx> SemContext<'cx> {
    delegate! {
        to self.defs {
            fn intern(&'cx mut self, def: Def<'cx>)
                -> Interned<'cx, Def<'cx>>;
        }
    }
}
