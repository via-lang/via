use super::{
    Hir, HirBuilder,
    error::{Error, Result},
};
use crate::{
    ast::{Expr as AstExpr, ExprKind as AstExprKind},
    def::FnDef,
    macros::ice_unimplemented,
    node::NodeId,
    sema::{BinaryOp, Ty},
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
                let fn_def = &self.def[*callee];
                let sig = &self.def[fn_def.sig];
                return Ok(sig.ret);
            }
        };

        Ok(self.sem.intern_ty(ty))
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
                let ty = self.unify(lty, rty)?;

                let trait_name = match op {
                    BinaryOp::Add => "Add",
                    BinaryOp::Sub => "Sub",
                    BinaryOp::Mul => "Mul",
                    BinaryOp::Div => "Div",
                    _ => ice_unimplemented!(),
                };

                // TODO: This is a hack that will probably break when we have more complex trait resolution.
                // We should have a better way to resolve trait methods.
                let method_name = trait_name.to_string().to_lowercase();

                let trait_sym = self.st.intern(trait_name);
                let method_sym = self.st.intern(method_name);

                let class = self
                    .def
                    .get_trait(trait_sym)
                    .ok_or(Error::InvalidBinaryOp)?;

                let imp = self
                    .def
                    .get_trait_impl(class, ty)
                    .ok_or(Error::InvalidBinaryOp)?;

                let fn_def_id = imp.impls.get(&method_sym).ok_or(Error::InvalidBinaryOp)?;
                let fn_def = &self.def[*fn_def_id];

                let sig = &self.def[fn_def.sig];

                (sig.parms.len() == 2 && sig.parms[0] == ty && sig.parms[1] == ty)
                    .then_some(())
                    .ok_or(Error::InvalidBinaryOp)?;

                Expr::Call {
                    callee: *fn_def_id,
                    args: vec![lhs, rhs],
                }
            }
            _ => ice_unimplemented!(),
        };

        Ok(expr)
    }
}
