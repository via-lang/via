/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Hir, block::BlockId, builder::IrBuilder, env::Env, instr::ValueId, place::ReadKind};
use crate::ast::stmt::Stmt;

impl<'a> IrBuilder<'a> {
    pub(super) fn lower_stmt(
        &mut self,
        hir: &mut Hir,
        env: &mut Env,
        block: BlockId,
        stmt: &'a Stmt,
    ) -> BlockId {
        match stmt {
            Stmt::Decl(decl) => self.lower_decl(hir, env, block, decl),
            Stmt::Control(control) => self.lower_control(hir, env, block, control),
            Stmt::Expr(expr) => {
                self.lower_expr(hir, env, block, expr, ValueId::Discard, ReadKind::Move);
                block
            }
        }
    }
}
