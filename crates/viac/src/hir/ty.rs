/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

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
            _ => todo!(),
        };

        Some(self.sema.alloc_ty(ty))
    }
}
