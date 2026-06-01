pub mod error;
mod expr;
mod pass;
mod stmt;
mod ty;

use via_macros::Arena;

use crate::node::NodeId;

use pass::zonk::ZonkPass;

pub use {expr::*, pass::Pass, stmt::*};

#[derive(Arena, Debug, Default)]
pub struct Hir {
    #[allocator]
    expr: Vec<Expr>,
    #[allocator]
    stmt: Vec<Stmt>,
    pub roots: Vec<NodeId<Stmt>>,
}

use crate::{ast::Tree, clinic::Clinic, def::DefContext, sema::SemContext, symbol::StringInterner};

pub struct HirBuilder<'cx, 'tree> {
    pub(super) clinic: &'cx mut Clinic,
    pub(super) interner: &'cx mut StringInterner,
    pub(super) sem_ctxt: &'cx mut SemContext,
    pub(super) def_ctxt: &'cx mut DefContext,
    pub(super) ast: &'tree Tree,
}

impl<'cx, 'tree> HirBuilder<'cx, 'tree> {
    pub fn new(
        clinic: &'cx mut Clinic,
        interner: &'cx mut StringInterner,
        sem_ctxt: &'cx mut SemContext,
        def_ctxt: &'cx mut DefContext,
        ast: &'tree Tree,
    ) -> Self {
        Self {
            clinic,
            interner,
            sem_ctxt,
            def_ctxt,
            ast,
        }
    }

    fn run_pass(&mut self, hir: &mut Hir, pass: &mut impl Pass) -> Option<()> {
        pass.run(self.sem_ctxt, hir)
            .map_err(|e| self.clinic.report(e))
            .ok()
    }

    pub fn lower(&mut self) -> Option<Hir> {
        let mut hir = Hir::default();

        for root in &self.ast.roots {
            let stmt = self
                .lower_stmt(&mut hir, *root)
                .inspect_err(|e| self.clinic.report(*e))
                .map(|stmt| hir.alloc_stmt(stmt))
                .ok()?;

            hir.roots.push(stmt);
        }

        self.run_pass(&mut hir, &mut ZonkPass)?;

        Some(hir)
    }
}
