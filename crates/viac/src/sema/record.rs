/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{ty::Ty, visibility::Visibility};
use crate::{intern::Interned, module::symbol::SymbolId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field<'cx> {
    pub vis: Visibility,
    pub name: SymbolId,
    pub ty: Interned<'cx, Ty<'cx>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Record<'cx> {
    pub fields: Vec<Field<'cx>>,
}
