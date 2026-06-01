use super::{
    Hir, HirBuilder,
    error::{Error, Result},
};
use crate::{
    ast::{Ty as AstTy, TyKind as AstTyKind},
    hir::Expr,
    node::NodeId,
    sema::{ConstSubst, Ty},
};

impl HirBuilder<'_, '_> {
    pub(super) fn unify(&mut self, lty: NodeId<Ty>, rty: NodeId<Ty>) -> Result<NodeId<Ty>> {
        match (&self.sem_ctxt[lty], &self.sem_ctxt[rty]) {
            (Ty::Meta(m), _) => self.sem_ctxt.solve_meta(*m, rty),
            (_, Ty::Meta(m)) => self.sem_ctxt.solve_meta(*m, lty),
            (_, _) if lty == rty => {}
            (_, _) => return Err(Error::TypeMismatch(lty, rty)),
        }
        Ok(lty)
    }

    pub(super) fn lower_ty(&mut self, hir: &mut Hir, ty: NodeId<AstTy>) -> Result<NodeId<Ty>> {
        let ty = &self.ast[ty];
        let ty = match ty.kind {
            AstTyKind::Unit => Ty::Unit,
            AstTyKind::Bool => Ty::Bool,
            AstTyKind::Int => Ty::Int,
            AstTyKind::Float => Ty::Float,
            AstTyKind::Array { ty, size } => {
                let expr = self.lower_expr(hir, size)?;
                let subst = match expr {
                    Expr::Bool(bool) => ConstSubst::Bool(bool),
                    Expr::Int(int) => ConstSubst::Int(int),
                    _ => return Err(Error::InvalidConstGeneric),
                };

                Ty::Array {
                    ty: self.lower_ty(hir, ty)?,
                    size: subst,
                }
            }
            AstTyKind::Vector(ty) => Ty::Vector(self.lower_ty(hir, ty)?),
        };

        Ok(self.sem_ctxt.intern_ty(ty))
    }
}
