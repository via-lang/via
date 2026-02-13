/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Ty;
use crate::intern::Interned;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Map<'cx> {
    pub key: Interned<'cx, Ty<'cx>>,
    pub value: Interned<'cx, Ty<'cx>>,
}
