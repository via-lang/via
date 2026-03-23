use super::{
    Hir,
    pass::{Pass, typeck::TypeckPass},
};
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

    fn run_pass(&mut self, hir: &mut Hir, pass: &mut impl Pass) -> Option<()> {
        pass.run(self.sema, hir)
            .map_err(|e| self.clinic.report(e))
            .ok()
    }

    pub fn lower(&mut self) -> Option<Hir> {
        let mut hir = Hir::default();

        for root in &self.ast.roots {
            let stmt = self.lower_stmt(&mut hir, *root)?;
            let stmt = hir.alloc_stmt(stmt);
            hir.roots.push(stmt);
        }

        self.run_pass(&mut hir, &mut TypeckPass)?;

        Some(hir)
    }
}
