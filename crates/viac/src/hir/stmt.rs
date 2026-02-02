/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    block::Block,
    builder::IrBuilder,
    env::Env,
    error::{Error, Result},
};
use crate::ast::{
    Tree,
    stmt::{Stmt, StmtId},
};

impl IrBuilder<'_> {
    pub(super) fn lower_stmt(
        &mut self,
        env: &mut Env,
        block: &mut Block,
        stmt: StmtId,
    ) -> Result<()> {
        let ast = self.ast;
        let stmt = ast.get(stmt);

        match stmt {
            Stmt::Control(ctrl) => match ctrl {
                _ => todo!(),
            },
            Stmt::Decl(decl) => match decl {
                _ => todo!(),
            },
            Stmt::Expr(expr) => {
                // self.lower_expr(env, block, ast.insert(expr.clone()), None)?;
            }
        };
        Ok(())
    }
}
