use super::{Hir, HirBuilder, error::Result, expr::Expr};
use crate::{
    ast::{Stmt as AstStmt, StmtKind as AstStmtKind},
    macros::ice_unimplemented,
    module::SymbolId,
    node::NodeId,
    sema::Ty,
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

                let rty = self.infer(hir, expr)?;
                let lty = ty.and_then(|ty| self.lower_ty(hir, ty));

                if let Some(lty) = lty {
                    self.unify(lty, rty)?;
                }

                Stmt::Let {
                    ident: self.st.intern(ident),
                    ty: lty.unwrap_or(rty),
                    expr,
                }
            }
            _ => ice_unimplemented!(),
        };

        Ok(stmt)
    }
}
