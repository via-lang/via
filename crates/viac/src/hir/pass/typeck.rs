use super::prelude::*;
use crate::{
    node::NodeId,
    sema::{context::SemContext, ty::Ty},
};

pub struct TypeckPass;

fn unify(sem: &mut SemContext, lty: NodeId<Ty>, rty: NodeId<Ty>) -> Result<()> {
    match (&sem[lty], &sem[rty]) {
        (Ty::Meta(m), _) => sem.solve_meta(*m, rty),
        (_, Ty::Meta(m)) => sem.solve_meta(*m, lty),
        (_, _) if lty == rty => {}
        (_, _) => return Err(Error::TypeMismatch(lty, rty)),
    }
    Ok(())
}

fn infer(sem: &mut SemContext, hir: &Hir, expr: NodeId<Expr>) -> Result<NodeId<Ty>> {
    let ty = match &hir[expr] {
        Expr::None => Ty::None,
        Expr::Bool(_) => Ty::Bool,
        Expr::Int(_) => Ty::Int,
        Expr::Float(_) => Ty::Float,
        Expr::String(_) => Ty::String,
        Expr::Binary { op, lhs, rhs } => {
            let lty = infer(sem, hir, *lhs)?;
            let rty = infer(sem, hir, *rhs)?;

            unify(sem, lty, rty)?;

            return Ok(lty);
        }
    };

    Ok(sem.intern_ty(ty))
}

fn solve(sem: &mut SemContext, hir: &Hir, stmt: NodeId<Stmt>) -> Result<()> {
    match hir[stmt] {
        Stmt::Let { ty: lty, expr, .. } => {
            let rty = infer(sem, hir, expr)?;
            unify(sem, lty, rty)?;
        }
        Stmt::Consume(_) | Stmt::Discard(_) => {}
    }
    Ok(())
}

fn zonk_ty(sem: &mut SemContext, ty: NodeId<Ty>) -> Result<NodeId<Ty>> {
    match &sem[ty] {
        Ty::Meta(meta) => sem.get_meta(*meta).ok_or(Error::UnsolvedMetavar(*meta)),
        _ => Ok(ty),
    }
}

#[allow(clippy::only_used_in_recursion)]
fn zonk_expr(sem: &mut SemContext, hir: &mut Hir, expr: NodeId<Expr>) -> Result<NodeId<Expr>> {
    #[allow(clippy::single_match)]
    match hir[expr] {
        Expr::Binary { lhs, rhs, .. } => {
            zonk_expr(sem, hir, lhs)?;
            zonk_expr(sem, hir, rhs)?;
        }
        _ => {}
    }
    Ok(expr)
}

fn zonk(sem: &mut SemContext, hir: &mut Hir, stmt: NodeId<Stmt>) -> Result<()> {
    match hir[stmt] {
        Stmt::Let { ident, ty, expr } => {
            let ty = zonk_ty(sem, ty)?;
            let expr = zonk_expr(sem, hir, expr)?;
            hir[stmt] = Stmt::Let { ident, ty, expr }
        }
        Stmt::Discard(_) | Stmt::Consume(_) => {}
    }
    Ok(())
}

impl Pass for TypeckPass {
    fn run(&mut self, sem: &mut SemContext, hir: &mut Hir) -> Result<()> {
        let roots = hir.roots.clone();
        roots.iter().try_for_each(|&stmt| solve(sem, hir, stmt))?;
        roots.iter().try_for_each(|&stmt| zonk(sem, hir, stmt))?;

        Ok(())
    }
}
