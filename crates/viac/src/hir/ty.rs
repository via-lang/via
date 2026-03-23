use super::{Hir, HirBuilder};
use crate::{
    ast::ty::{Ty as AstTy, TyKind as AstTyKind},
    node::NodeId,
    sema::ty::Ty,
};

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
