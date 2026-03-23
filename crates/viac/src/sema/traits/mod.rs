pub mod arith;
pub mod builder;

use std::collections::HashMap;

use super::func::{FuncImpl, FuncSig};
use crate::{module::symbol::SymbolId, node::NodeId};

#[derive(Debug)]
pub struct TraitDef {
    pub sym: SymbolId,
    pub funcs: Vec<NodeId<FuncSig>>,
}

#[derive(Debug)]
pub struct TraitImpl {
    pub proto: NodeId<TraitDef>,
    pub impls: HashMap<NodeId<FuncSig>, FuncImpl>,
}
