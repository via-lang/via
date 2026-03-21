/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Hir;
use crate::{ast::Tree, clinic::Clinic, module::symbol::SymbolTable, sema::context::SemContext};

pub struct HirBuilder<'cx, 'tree> {
    pub(super) clinic: &'cx mut Clinic,
    pub(super) symbols: &'cx mut SymbolTable,
    pub(super) sema: &'cx mut SemContext,
    pub(super) ast: &'tree Tree,
}

impl<'cx, 'tree> HirBuilder<'cx, 'tree> {
    pub fn new(
        clinic: &'cx mut Clinic,
        symbols: &'cx mut SymbolTable,
        sema: &'cx mut SemContext,
        ast: &'tree Tree,
    ) -> Self {
        Self {
            clinic,
            symbols,
            sema,
            ast,
        }
    }

    pub fn lower(&mut self) -> Option<Hir> {
        let mut hir = Hir::default();

        for root in &self.ast.roots {
            let stmt = self.lower_stmt(&mut hir, *root)?;
            let stmt = hir.alloc_stmt(stmt);
            hir.roots.push(stmt);
        }

        Some(hir)
    }
}
