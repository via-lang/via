/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Hir;
use crate::{ast::Tree, clinic::Clinic, module::symbol::SymbolTable};

#[derive(Debug)]
pub struct HirBuilder<'a> {
    pub(super) symbols: &'a mut SymbolTable,
    pub(super) clinic: &'a mut Clinic,
    pub(super) ast: &'a Tree,
}

impl<'a> HirBuilder<'a> {
    pub fn new(symbols: &'a mut SymbolTable, clinic: &'a mut Clinic, ast: &'a Tree) -> Self {
        Self {
            symbols,
            clinic,
            ast,
        }
    }

    pub fn lower(&mut self) -> Option<Hir> {
        todo!()
    }
}
