/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Hir, block::BlockId, builder::IrBuilder, error::Result, instr::Instr, term::Term};
use crate::ast::{decl::Decl, stmt::Stmt};

impl<'a> IrBuilder<'a> {
    pub(super) fn lower_stmt(
        &mut self,
        hir: &mut Hir,
        block: BlockId,
        stmt: &'a Stmt,
    ) -> Result<BlockId> {
        match stmt {
            Stmt::Decl(d) => match d {
                Decl::Variable(var) => {
                    let [id] = hir.local_id.bump::<1>().map(Into::into);
                    self.lower_expr(hir, block, self.ast.get(var.expr), Some(id))
                }
                Decl::Function(fun) => {
                    let mut current = self.block(hir);
                    let [out] = hir.local_id.bump::<1>().map(Into::into);

                    self.push(
                        hir,
                        block,
                        Instr::Closure {
                            block: current,
                            upvals: vec![],
                            out,
                        },
                    );

                    let ctr = hir.local_id.reset();

                    for stmt in &fun.body.inner {
                        let stmt = self.ast.get(*stmt);
                        current = self.lower_stmt(hir, current, stmt)?;
                    }

                    hir.local_id.restore(ctr);
                    Ok(block)
                }
                _ => todo!(),
            },
            Stmt::Control(_) => todo!(),
            Stmt::Expr(expr) => self.lower_expr(hir, block, expr, None),
        }
    }
}
