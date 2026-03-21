/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Hir, HirBuilder, error::*, expr::Expr};
use crate::{
    ast::stmt::{Stmt as AstStmt, StmtKind as AstStmtKind},
    module::symbol::SymbolId,
    node::NodeId,
    sema::ty::Ty,
};

#[derive(Debug)]
pub enum Stmt {
    Let {
        ident: SymbolId,
        ty: NodeId<Ty>,
        expr: NodeId<Expr>,
    },
    Discard(NodeId<Expr>),
    Consume(NodeId<Expr>),
}

impl HirBuilder<'_, '_> {
    pub(super) fn lower_stmt(&mut self, hir: &mut Hir, stmt: NodeId<AstStmt>) -> Option<Stmt> {
        let stmt = &self.ast[stmt];
        let stmt = match &stmt.kind {
            AstStmtKind::Let { ident, ty, expr } => {
                let expr = self.lower_expr(hir, *expr)?;
                let ty = ty
                    .map(|ty| self.lower_ty(hir, ty))
                    .flatten()
                    .unwrap_or_else(|| {
                        let meta = self.sema.next_meta();
                        let ty = self.sema.alloc_ty(Ty::Meta(meta));
                        ty
                    });

                Stmt::Let {
                    ident: self.symbols.intern(ident),
                    ty,
                    expr: hir.alloc_expr(expr),
                }
            }
            _ => todo!(),
        };

        Some(stmt)
    }
}
