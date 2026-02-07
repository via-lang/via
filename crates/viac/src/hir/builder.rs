/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Hir, block::Block};
use crate::{
    ast::Tree,
    clinic::Clinic,
    hir::{block::BlockId, env::Env, instr::Instr, term::Term},
    module::symbol::SymbolTable,
    source::SourceBuf,
};

#[derive(Debug)]
pub struct IrBuilder<'a> {
    pub(super) source: SourceBuf,
    pub(super) symbols: &'a mut SymbolTable,
    pub(super) ast: &'a Tree,
    pub(super) clinic: &'a mut Clinic,
}

impl<'a> IrBuilder<'a> {
    pub fn new(
        source: &SourceBuf,
        symbols: &'a mut SymbolTable,
        ast: &'a Tree,
        clinic: &'a mut Clinic,
    ) -> Self {
        Self {
            source: source.clone(),
            symbols,
            ast,
            clinic,
        }
    }

    pub fn block(&mut self, hir: &mut Hir) -> BlockId {
        let len = hir.blocks.len();
        let id = BlockId::from(len as u32);
        hir.blocks.push(Block::new(id));
        id
    }

    pub fn terminate(&mut self, hir: &mut Hir, block: BlockId, term: Term) {
        hir.get_mut(block).term = term;
    }

    pub fn push(&mut self, hir: &mut Hir, block: BlockId, instr: Instr) {
        hir.get_mut(block).instrs.push(instr);
    }

    pub fn is_terminated(&self, hir: &mut Hir, block: BlockId) -> bool {
        !matches!(hir.get(block).term, Term::Halt)
    }

    pub(crate) fn lower(&mut self) -> Option<Hir> {
        let mut hir = Hir::default();
        let mut env = Env::new();

        let mut current = self.block(&mut hir);
        for stmt in &self.ast.stmts {
            current = self.lower_stmt(&mut hir, &mut env, current, self.ast.get(*stmt));
        }

        self.clinic.healthy().then_some(hir)
    }
}
