use super::{Mir, block::Block, env::Env};
use crate::{
    clinic::Clinic,
    hir::Hir,
    mir::{block::BlockId, instr::Instr, term::Term},
    module::symbol::SymbolTable,
};

#[derive(Debug)]
pub struct MirBuilder<'cx> {
    pub(super) symbols: &'cx mut SymbolTable,
    pub(super) clinic: &'cx mut Clinic,
    pub(super) hir: &'cx Hir,
}

impl<'cx> MirBuilder<'cx> {
    pub fn new(symbols: &'cx mut SymbolTable, clinic: &'cx mut Clinic, hir: &'cx Hir) -> Self {
        Self {
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

    pub fn terminate(&mut self, mir: &'cx mut Mir, block: BlockId, term: Term) {
        mir.get_mut(block).term = term;
    }

    pub fn push(&mut self, mir: &'cx mut Mir, block: BlockId, instr: Instr) {
        mir.get_mut(block).instrs.push(instr);
    }

    pub fn is_terminated(&self, mir: &mut Mir, block: BlockId) -> bool {
        !matches!(mir.get(block).term, Term::Halt)
    }

    pub fn lower(&mut self) -> Option<Mir> {
        let mut mir = Mir::default();

        let mut env = Env::new();
        let mut current = self.block(&mut mir);

        // for stmt in &self.hir.inner {
        //     current = self.lower_stmt(&mut mir, &mut env, current);
        // }

        self.clinic.healthy().then_some(mir)
    }
}
