use crate::{hir::stmt::Stmt, node::NodeId};

use super::{block::BlockId, builder::MirBuilder};

impl MirBuilder<'_> {
    pub(super) fn lower_stmt(
        &mut self,
        mir: &mut super::Mir,
        env: &mut super::env::Env,
        block: BlockId,
        stmt: NodeId<Stmt>,
    ) -> BlockId {
        match &self.hir[stmt] {
            Stmt::Let { ident, ty, expr } => {
                let _ = ();
                block
            }
            _ => todo!(),
        }
    }
}
