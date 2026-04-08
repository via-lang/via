use super::{
    Hir, HirBuilder,
    error::{Error, Result},
};
use crate::{
    ast::{Ty as AstTy, TyKind as AstTyKind},
    node::NodeId,
    sema::Ty,
};

impl HirBuilder<'_, '_> {
    pub(super) fn unify(&mut self, lty: NodeId<Ty>, rty: NodeId<Ty>) -> Result<NodeId<Ty>> {
        match (&self.sem[lty], &self.sem[rty]) {
            (Ty::Meta(m), _) => self.sem.solve_meta(*m, rty),
            (_, Ty::Meta(m)) => self.sem.solve_meta(*m, lty),
            (_, _) if lty == rty => {}
            (_, _) => return Err(Error::TypeMismatch(lty, rty)),
        }
        Ok(lty)
    }

    pub(super) fn lower_ty(&mut self, _hir: &mut Hir, ty: NodeId<AstTy>) -> Option<NodeId<Ty>> {
        let ty = &self.ast[ty];
        let ty = match ty.kind {
            AstTyKind::Unit => Ty::Unit,
            AstTyKind::Bool => Ty::Bool,
            AstTyKind::Int => Ty::Int,
            AstTyKind::Float => Ty::Float,
        };

        Some(self.sem.intern_ty(ty))
    }
}
