use super::{func::FuncSig, traits::TraitDef, ty::Ty};
use crate::{module::symbol::SymbolId, node::NodeId};

#[derive(Debug)]
pub enum Error {
    DuplicateTrait(SymbolId),
    DuplicateTraitImpl(NodeId<Ty>, NodeId<TraitDef>),
    DuplicateTraitMethod(NodeId<FuncSig>),
    // TODO: This is way too generic
    BadTraitImpl,
}

pub type Result<T> = std::result::Result<T, Error>;
