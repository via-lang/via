/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::{ast::Tree, source::SourceBuf};

#[derive(Debug)]
pub struct IrBuilder<'a> {
    pub(super) source: SourceBuf,
    pub(super) ast: &'a Tree,
}

impl<'a> IrBuilder<'a> {
    pub fn new(source: SourceBuf, ast: &'a Tree) -> Self {
        Self { source, ast }
    }
}
