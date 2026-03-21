/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::{
    node::{NodeId, NodeStore},
    sema::ty::Ty,
};

pub struct Infer<'a> {
    hir: &'a Hir,
    builder: &'a mut HirBuilder<'a, 'a>,
}

impl Infer<'_> {
    fn infer_expr(&mut self, expr: NodeId<Expr>) -> NodeId<Ty> {
        let ty = match self.hir.get(expr) {
            Expr::None => Ty::None,
            Expr::Bool(_) => Ty::Bool,
            Expr::Int(_) => Ty::Int,
            Expr::Float(_) => Ty::Float,
            Expr::String(_) => Ty::String,
            Expr::Binary { op, lhs, rhs } => {}
            _ => unimplemented!(),
        };

        self.builder.sema.alloc_ty(ty)
    }
}

impl<'a> Pass<'a> for Infer<'a> {
    fn new(builder: &'a mut HirBuilder<'a, 'a>, hir: &'a Hir) -> Self {
        Self { hir, builder }
    }

    fn run(&mut self) -> Result<()> {}
}
