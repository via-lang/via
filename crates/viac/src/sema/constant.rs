/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::ty::Ty;
use crate::intern::Interned;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Constant<'cx> {
    pub ty: Interned<'cx, Ty<'cx>>,
}
