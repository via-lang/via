use super::{Hir, HirBuilder, expr::Expr};
use crate::{
    ast::stmt::{Stmt as AstStmt, StmtKind as AstStmtKind},
    module::symbol::SymbolId,
    node::NodeId,
    sema::ty::Ty,
};

#[derive(Debug, Clone)]
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
                        self.sema.intern_ty(Ty::Meta(meta))
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
