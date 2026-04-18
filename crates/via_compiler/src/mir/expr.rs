use itertools::Itertools;

use super::{Block, Instr, Mir, MirBuilder, Operand, env::Env};
use crate::{
    def::{FnImpl, Intrin},
    hir::Expr,
    macros::ice_unimplemented,
    node::NodeId,
    sema::ConstValue,
};

impl MirBuilder<'_> {
    pub(super) fn lower_expr(
        &mut self,
        mir: &mut Mir,
        env: &mut Env,
        block_id: NodeId<Block>,
        expr: NodeId<Expr>,
    ) -> Operand {
        let value = match &self.hir[expr] {
            Expr::Unit => ConstValue::Unit,
            Expr::Bool(b) => ConstValue::Bool(*b),
            Expr::Int(i) => ConstValue::Int(*i),
            Expr::Float(fp) => ConstValue::Float(*fp),
            Expr::Call { callee, args } => {
                let args = args
                    .iter()
                    .cloned()
                    .map(|expr| self.lower_expr(mir, env, block_id, expr))
                    .collect_vec();

                let out = Operand::Temp(env.temp_id.bump());

                match &self.def[*callee].impl_ {
                    FnImpl::Intrin(intrin) => {
                        let (lhs, rhs) = (args[0], args[1]);
                        let instr = match intrin {
                            Intrin::IAdd => Instr::IAdd { lhs, rhs, out },
                            _ => ice_unimplemented!(),
                        };

                        self.push(mir, block_id, instr);
                    }
                    FnImpl::Native(native) => {}
                }

                return out;
            }
            _ => ice_unimplemented!(),
        };

        let out = Operand::Temp(env.temp_id.bump());
        self.push(mir, block_id, Instr::Const { value, out });
        out
    }
}
