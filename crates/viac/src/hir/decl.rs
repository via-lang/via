/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Hir, block::BlockId, builder::IrBuilder, env::Env, instr::Instr, place::ReadKind};
use crate::ast::decl::Decl;

impl<'a> IrBuilder<'a> {
    pub(super) fn lower_decl(
        &mut self,
        hir: &mut Hir,
        env: &mut Env,
        block: BlockId,
        decl: &'a Decl,
    ) -> BlockId {
        match decl {
            Decl::Variable(var) => {
                let name = self.source.get_span(&var.symbol.span);
                let id = self.symbols.intern(name);
                let local = env.push(id);

                self.lower_expr(
                    hir,
                    env,
                    block,
                    self.ast.get(var.expr),
                    local,
                    ReadKind::Move,
                );

                block
            }
            Decl::Function(fun) => {
                let name = self.source.get_span(&fun.symbol.span);
                let id = self.symbols.intern(name);
                let out = env.push(id).into();

                let mut inner_env = Env::new();
                let mut current = self.block(hir);

                self.push(
                    hir,
                    block,
                    Instr::Closure {
                        block: current,
                        upvals: vec![],
                        out,
                    },
                );

                for stmt in &fun.body.inner {
                    let stmt = self.ast.get(*stmt);
                    current = self.lower_stmt(hir, &mut inner_env, current, stmt);
                }

                block
            }
            _ => todo!(),
        }
    }
}
