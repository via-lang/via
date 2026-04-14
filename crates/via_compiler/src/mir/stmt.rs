use super::{Block, Instr, Mir, MirBuilder, env::Env};
use crate::{hir::Stmt, macros::ice_unimplemented, node::NodeId};

impl MirBuilder<'_> {
    pub(super) fn lower_stmt(
        &mut self,
        mir: &mut Mir,
        env: &mut Env,
        block_id: NodeId<Block>,
        stmt: NodeId<Stmt>,
    ) -> NodeId<Block> {
        match &self.hir[stmt] {
            Stmt::Let { ident, expr, .. } => {
                let local = env.insert(*ident);
                let value = self.lower_expr(mir, env, block_id, *expr);

                self.push(
                    mir,
                    block_id,
                    Instr::Local {
                        id: value,
                        out: local,
                    },
                );

                block_id
            }
            _ => ice_unimplemented!(),
        }
    }
}
