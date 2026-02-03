/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::symbol::SymbolId;
use crate::sema::ty::Ty;

#[derive(Debug)]
pub enum Binding {
    Type {
        id: SymbolId,
        ty: Ty,
    },
    Constant {
        id: SymbolId,
        ty: Ty,
    },
    Function {
        id: SymbolId,
        ret: Ty,
        params: Vec<Ty>,
    },
}
