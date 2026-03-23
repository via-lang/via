use super::{Hir, HirBuilder, error::Error};
use crate::{
    ast::expr::{Expr as AstExpr, ExprKind as AstExprKind},
    node::NodeId,
    sema::ops::*,
};

#[derive(Debug, Clone)]
pub enum Expr {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Binary {
        op: BinaryOp,
        lhs: NodeId<Expr>,
        rhs: NodeId<Expr>,
    },
}

impl HirBuilder<'_, '_> {
    pub(super) fn lower_expr(&mut self, hir: &mut Hir, expr: NodeId<AstExpr>) -> Option<Expr> {
        use AstExprKind::*;

        let expr = &self.ast[expr];
        let expr = match &expr.kind {
            None => Expr::None,
            True => Expr::Bool(true),
            False => Expr::Bool(false),
            Integer(int) => {
                let int = i64::try_from(*int)
                    .inspect_err(|_| self.clinic.report(Error::IntOutOfRange))
                    .ok()?;
                Expr::Int(int)
            }
            Float(float) => Expr::Float(*float),
            Binary { op, lhs, rhs } => {
                let lhs = self.lower_expr(hir, *lhs)?;
                let rhs = self.lower_expr(hir, *rhs)?;

                Expr::Binary {
                    op: *op,
                    lhs: hir.alloc_expr(lhs),
                    rhs: hir.alloc_expr(rhs),
                }
            }
            _ => todo!(),
        };

        Some(expr)
    }
}
