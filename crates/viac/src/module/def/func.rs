use super::{super::symbol::SymbolId, DefId};
use crate::{node::NodeId, sema::Ty};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnSig {
    pub parms: Vec<NodeId<Ty>>,
    pub ret: NodeId<Ty>,
}

#[derive(Debug)]
pub struct FnDef {
    pub sym: SymbolId,
    pub parent: Option<DefId>,
    pub sig: NodeId<FnSig>,
    pub impl_: FnImpl,
}

#[derive(Debug)]
pub enum Intrin {
    IAdd,
    FAdd,
}

#[derive(Debug)]
pub enum FnImpl {
    Intrin(Intrin),
}
