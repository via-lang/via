use super::{
    Hir, HirBuilder,
    error::{Error, Result},
    ty::unify,
};
use crate::{
    ast::expr::{Expr as AstExpr, ExprKind as AstExprKind},
    node::NodeId,
    sema::{context::SemContext, func::FuncSig, ops::BinaryOp, traits::TraitDef, ty::Ty},
};

#[derive(Debug, Clone)]
pub enum Expr {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    TraitCall {
        proto: NodeId<TraitDef>,
        method: NodeId<FuncSig>,
        args: Vec<NodeId<Expr>>,
    },
}

pub fn infer(sem: &mut SemContext, hir: &Hir, expr: NodeId<Expr>) -> Result<NodeId<Ty>> {
    let ty = match &hir[expr] {
        Expr::None => Ty::None,
        Expr::Bool(_) => Ty::Bool,
        Expr::Int(_) => Ty::Int,
        Expr::Float(_) => Ty::Float,
        Expr::String(_) => Ty::String,
        Expr::TraitCall { method, .. } => return Ok(sem[*method].ret),
    };

    Ok(sem.intern_ty(ty))
}

impl HirBuilder<'_, '_> {
    pub(super) fn lower_expr(&mut self, hir: &mut Hir, expr: NodeId<AstExpr>) -> Result<Expr> {
        use AstExprKind::*;

        let expr = &self.ast[expr];
        let expr = match &expr.kind {
            None => Expr::None,
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

                let lty = infer(self.sema, hir, lhs)?;
                let rty = infer(self.sema, hir, rhs)?;

                let ty = unify(self.sema, lty, rty)?;

                let trait_name = match op {
                    BinaryOp::Add => "Add",
                    BinaryOp::Sub => "Sub",
                    BinaryOp::Mul => "Mul",
                    BinaryOp::Div => "Div",
                    _ => todo!(),
                };

                // TODO: This is a hack that will probably break when we have more complex trait resolution.
                // We should have a better way to resolve trait methods.
                let method_name = trait_name.to_string().to_lowercase();

                let trait_sym = self.symbols.intern(trait_name);
                let method_sym = self.symbols.intern(method_name);

                let proto = self
                    .sema
                    .get_trait(trait_sym)
                    .ok_or(Error::InvalidBinaryOp)?;

                let imp = self
                    .sema
                    .get_trait_impl(proto, ty)
                    .ok_or(Error::InvalidBinaryOp)?;

                let (sig_id, _) = imp.impls.get(&method_sym).ok_or(Error::InvalidBinaryOp)?;
                let sig = &self.sema[*sig_id];

                (sig.parms.len() == 2 && sig.parms[0] == ty && sig.parms[1] == ty)
                    .then_some(())
                    .ok_or(Error::InvalidBinaryOp)?;

                Expr::TraitCall {
                    proto,
                    method: *sig_id,
                    args: vec![lhs, rhs],
                }
            }
            _ => todo!(),
        };

        Ok(expr)
    }
}
