/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Trait;
use crate::intern::Interned;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitImpl<'cx> {
    pub class: Interned<'cx, Trait<'cx>>,
}
