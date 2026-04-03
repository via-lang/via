use super::{
    Hir, HirBuilder,
    error::Result,
    expr::{Expr, infer},
    ty::unify,
};
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
    pub(super) fn lower_stmt(&mut self, hir: &mut Hir, stmt: NodeId<AstStmt>) -> Result<Stmt> {
        let stmt = &self.ast[stmt];
        let stmt = match &stmt.kind {
            AstStmtKind::Let { ident, ty, expr } => {
                let expr = self.lower_expr(hir, *expr)?;
                let expr = hir.alloc_expr(expr);

                let rty = infer(self.sema, hir, expr)?;
                let lty = ty.and_then(|ty| self.lower_ty(hir, ty));

                if let Some(lty) = lty {
                    unify(self.sema, lty, rty)?;
                }

                Stmt::Let {
                    ident: self.symbols.intern(ident),
                    ty: lty.unwrap_or(rty),
                    expr,
                }
            }
            _ => todo!(),
        };

        Ok(stmt)
    }
}
