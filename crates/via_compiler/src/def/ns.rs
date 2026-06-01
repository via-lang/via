use super::DefId;
use crate::{node::NodeId, symbol::Symbol};

#[derive(Debug)]
pub struct NsDef {
    pub symbol: Symbol,
    pub parent: Option<DefId>,
}

impl From<NodeId<NsDef>> for DefId {
    fn from(value: NodeId<NsDef>) -> Self {
        DefId::NsId(value)
    }
}
