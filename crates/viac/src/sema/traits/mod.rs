/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{canonical_map::CanonicalMap, function::Function, ty::Ty};
use crate::{intern::Interned, module::symbol::SymbolId};

pub mod context;
pub mod imp;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trait<'cx> {
    pub tys: CanonicalMap<SymbolId, Interned<'cx, Ty<'cx>>>,
    pub fns: CanonicalMap<SymbolId, Interned<'cx, Function<'cx>>>,
}
