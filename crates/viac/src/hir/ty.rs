use super::{
    Hir, HirBuilder,
    error::{Error, Result},
};
use crate::{
    ast::ty::{Ty as AstTy, TyKind as AstTyKind},
    node::NodeId,
    sema::{context::SemContext, ty::Ty},
};

pub fn unify(sem: &mut SemContext, lty: NodeId<Ty>, rty: NodeId<Ty>) -> Result<NodeId<Ty>> {
    match (&sem[lty], &sem[rty]) {
        (Ty::Meta(m), _) => sem.solve_meta(*m, rty),
        (_, Ty::Meta(m)) => sem.solve_meta(*m, lty),
        (_, _) if lty == rty => {}
        (_, _) => return Err(Error::TypeMismatch(lty, rty)),
    }
    Ok(lty)
}

impl HirBuilder<'_, '_> {
    pub(super) fn lower_ty(&mut self, _hir: &mut Hir, ty: NodeId<AstTy>) -> Option<NodeId<Ty>> {
        let ty = &self.ast[ty];
        let ty = match ty.kind {
            AstTyKind::None => Ty::None,
            AstTyKind::Bool => Ty::Bool,
            AstTyKind::Int => Ty::Int,
            AstTyKind::Float => Ty::Float,
        };

        Some(self.sema.intern_ty(ty))
    }
}
