pub mod arith;
pub mod builder;

use std::collections::HashMap;

use super::{
    context::SemContext,
    error::Result,
    func::{FuncImpl, FuncSig},
};
use crate::{
    module::symbol::{SymbolId, SymbolTable},
    node::NodeId,
};

use arith::register_builtin_arith;

#[derive(Debug)]
pub struct TraitDef {
    pub sym: SymbolId,
    pub funcs: HashMap<SymbolId, NodeId<FuncSig>>,
}

#[derive(Debug)]
pub struct TraitImpl {
    pub proto: NodeId<TraitDef>,
    pub impls: HashMap<SymbolId, (NodeId<FuncSig>, FuncImpl)>,
}

pub fn register_builtin(sem: &mut SemContext, st: &mut SymbolTable) -> Result<()> {
    register_builtin_arith(st, sem)?;
    Ok(())
}
