mod block;
mod env;
pub mod error;
mod expr;
mod instr;
mod stmt;
mod term;

use pretty::RcDoc;
use via_macros::Arena;

use crate::{
    clinic::Clinic, def::DefContext, hir::Hir, node::NodeId, sema::SemContext, symbol::SymbolTable,
};

use env::Env;

pub use {block::*, instr::*, term::*};

#[derive(Arena, Default)]
pub struct Mir {
    #[allocator]
    pub blocks: Vec<Block>,
}

impl Mir {
    pub fn to_doc(&self) -> RcDoc<'_> {
        let mut doc = RcDoc::nil();
        for block in &self.blocks {
            doc = doc.append(block.to_doc());
        }
        doc
    }

    pub fn print(&self) {
        self.to_doc()
            .render(usize::MAX, &mut std::io::stdout())
            .unwrap();
        println!();
    }
}

#[derive(Debug)]
pub struct MirBuilder<'cx> {
    pub(super) st: &'cx mut SymbolTable,
    pub(super) sem: &'cx mut SemContext,
    pub(super) def: &'cx mut DefContext,
    pub(super) clinic: &'cx mut Clinic,
    pub(super) hir: &'cx Hir,
}

impl<'cx> MirBuilder<'cx> {
    pub fn new(
        st: &'cx mut SymbolTable,
        sem: &'cx mut SemContext,
        def: &'cx mut DefContext,
        clinic: &'cx mut Clinic,
        hir: &'cx Hir,
    ) -> Self {
        Self {
            st,
            sem,
            def,
            clinic,
            hir,
        }
    }

    pub(super) fn terminate(&mut self, mir: &mut Mir, block: NodeId<Block>, term: Term) {
        mir[block].term = term;
    }

    pub(super) fn push(&mut self, mir: &mut Mir, block: NodeId<Block>, instr: Instr) {
        mir[block].instrs.push(instr);
    }

    pub(super) fn is_terminated(&self, mir: &mut Mir, block: NodeId<Block>) -> bool {
        !matches!(mir[block].term, Term::Halt)
    }

    pub fn lower(&mut self) -> Option<Mir> {
        let mut mir = Mir::default();

        let local_id = Default::default();

        let mut env = Env::new(local_id, None);
        let mut current = mir.alloc_blocks(Block::new());

        for stmt in &self.hir.roots {
            current = self.lower_stmt(&mut mir, &mut env, current, *stmt);
        }

        self.clinic.healthy().then_some(mir)
    }
}
