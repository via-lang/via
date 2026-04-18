use std::collections::HashMap;

use crate::{node::NodeId, symbol::SymbolId};

use super::{DefId, FnDef, FnSig};

#[derive(Debug)]
pub struct TraitDef {
    pub sym: SymbolId,
    pub parent: Option<DefId>,
    pub methods: HashMap<SymbolId, NodeId<FnSig>>,
}

#[derive(Debug)]
pub struct TraitImpl {
    pub class: NodeId<TraitDef>,
    pub impls: HashMap<SymbolId, NodeId<FnDef>>,
}
