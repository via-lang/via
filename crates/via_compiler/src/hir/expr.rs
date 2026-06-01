use super::{
    Hir, HirBuilder,
    error::{Error, Result},
};
use crate::{
    ast::{Expr as AstExpr, ExprKind as AstExprKind},
    def::{FnDef, traits::TraitImplKey},
    macros::ice_unimplemented,
    node::NodeId,
    sema::{BinaryOp, Ty},
    symbol::IntoSymbol,
};

#[derive(Debug, Clone)]
pub enum Expr {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Call {
        callee: NodeId<FnDef>,
        args: Vec<NodeId<Expr>>,
    },
}

impl HirBuilder<'_, '_> {
    pub(super) fn infer(&mut self, hir: &Hir, expr: NodeId<Expr>) -> Result<NodeId<Ty>> {
        let ty = match &hir[expr] {
            Expr::Unit => Ty::Unit,
            Expr::Bool(_) => Ty::Bool,
            Expr::Int(_) => Ty::Int,
            Expr::Float(_) => Ty::Float,
            Expr::String(_) => Ty::String,
            Expr::Call { callee, .. } => {
                let fn_def = &self.def_ctxt[*callee];
                let sig = &self.def_ctxt[fn_def.sig];
                return Ok(sig.result);
            }
        };

        Ok(self.sem_ctxt.intern_ty(ty))
    }

    pub(super) fn lower_expr(&mut self, hir: &mut Hir, expr: NodeId<AstExpr>) -> Result<Expr> {
        use AstExprKind::*;

        let expr = &self.ast[expr];
        let expr = match &expr.kind {
            Unit => Expr::Unit,
            True => Expr::Bool(true),
            False => Expr::Bool(false),
            Integer(int) => {
                let int = i64::try_from(*int).map_err(|_| Error::IntOutOfRange)?;
                Expr::Int(int)
            }
            Float(float) => Expr::Float(*float),
            Binary { op, lhs, rhs } => {
                let lhs = self.lower_expr(hir, *lhs)?;
                let rhs = self.lower_expr(hir, *rhs)?;
                let lhs = hir.alloc_expr(lhs);
                let rhs = hir.alloc_expr(rhs);

                let lty = self.infer(hir, lhs)?;
                let rty = self.infer(hir, rhs)?;

                let trait_name = match op {
                    BinaryOp::Add => "Add",
                    BinaryOp::Sub => "Sub",
                    BinaryOp::Mul => "Mul",
                    BinaryOp::Div => "Div",
                    BinaryOp::Pow => "Pow",
                    BinaryOp::Mod => "Rem",
                    _ => ice_unimplemented!(),
                };

                let trait_sym = trait_name.into_symbol(self.interner);
                let method_sym = trait_name.to_lowercase().into_symbol(self.interner);

                let class = self
                    .def_ctxt
                    .get_trait(trait_sym)
                    .ok_or(Error::InvalidBinaryOp)?;

                let imp = self
                    .def_ctxt
                    .get_trait_impl(class, &TraitImplKey::new(lty, [rty]))
                    .ok_or(Error::InvalidBinaryOp)?;

                let fn_def_id = imp.methods.get(&method_sym).ok_or(Error::InvalidBinaryOp)?;

                Expr::Call {
                    callee: fn_def_id.def,
                    args: vec![lhs, rhs],
                }
            }
            _ => ice_unimplemented!(),
        };

        Ok(expr)
    }
}
