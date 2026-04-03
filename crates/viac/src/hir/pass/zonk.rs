use super::prelude::*;
use crate::{
    node::NodeId,
    sema::{context::SemContext, ty::Ty},
};

pub struct ZonkPass;

fn zonk_ty(sem: &mut SemContext, ty: NodeId<Ty>) -> Result<NodeId<Ty>> {
    match &sem[ty] {
        Ty::Meta(meta) => sem.get_meta(*meta).ok_or(Error::UnsolvedMetavar(*meta)),
        _ => Ok(ty),
    }
}

#[allow(clippy::only_used_in_recursion)]
fn zonk_expr(sem: &mut SemContext, hir: &mut Hir, expr: NodeId<Expr>) -> Result<NodeId<Expr>> {
    #[allow(clippy::single_match)]
    match &hir[expr] {
        Expr::TraitCall { args, .. } => {
            for arg in args.clone() {
                zonk_expr(sem, hir, arg)?;
            }
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

impl Pass for ZonkPass {
    fn run(&mut self, sem: &mut SemContext, hir: &mut Hir) -> Result<()> {
        let roots = hir.roots.clone();
        roots.iter().try_for_each(|&stmt| zonk(sem, hir, stmt))?;

        Ok(())
    }
}
