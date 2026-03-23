use super::ty::Ty;
use crate::{module::symbol::SymbolId, node::NodeId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncSig {
    pub sym: SymbolId,
    // TODO: Represent optional self parameter
    pub parms: Vec<NodeId<Ty>>,
    pub ret: NodeId<Ty>,
}

#[derive(Debug)]
pub enum Intrinsic {
    IAdd,
    FAdd,
}

#[derive(Debug)]
pub enum FuncImpl {
    Intrin(Intrinsic),
}
