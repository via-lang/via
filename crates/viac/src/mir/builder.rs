/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Mir, block::Block};
use crate::{
    clinic::Clinic,
    hir::Hir,
    mir::{block::BlockId, instr::Instr, term::Term},
    module::symbol::SymbolTable,
    source::SourceBuf,
};

#[derive(Debug)]
pub struct MirBuilder<'a> {
    pub(super) source: &'a SourceBuf,
    pub(super) symbols: &'a mut SymbolTable,
    pub(super) clinic: &'a mut Clinic,
    pub(super) hir: &'a Hir,
}

impl<'a> MirBuilder<'a> {
    pub fn new(
        source: &'a SourceBuf,
        symbols: &'a mut SymbolTable,
        clinic: &'a mut Clinic,
        hir: &'a Hir,
    ) -> Self {
        Self {
            source,
            symbols,
            clinic,
            hir,
        }
    }

    pub fn block(&mut self, mir: &mut Mir) -> BlockId {
        let len = mir.blocks.len();
        let id = BlockId::from(len as u32);
        mir.blocks.push(Block::new(id));
        id
    }

    pub fn terminate(&mut self, mir: &mut Mir, block: BlockId, term: Term) {
        mir.get_mut(block).term = term;
    }

    pub fn push(&mut self, mir: &mut Mir, block: BlockId, instr: Instr) {
        mir.get_mut(block).instrs.push(instr);
    }

    pub fn is_terminated(&self, mir: &mut Mir, block: BlockId) -> bool {
        !matches!(mir.get(block).term, Term::Halt)
    }

    pub(crate) fn lower(&mut self) -> Option<Mir> {
        let /* mut */ mir = Mir::default();
        // let mut env = Env::new();

        // let mut current = self.block(&mut mir);
        // for stmt in &self.ast.stmts {
        //    current = self.lower_stmt(&mut mir, &mut env, current, self.ast.get(*stmt));
        // }

        self.clinic.healthy().then_some(mir)
    }
}
