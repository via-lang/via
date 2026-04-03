use super::{func::FuncSig, traits::TraitDef, ty::Ty};
use crate::{
    clinic::{Diagnostic, Severity},
    module::symbol::SymbolId,
    node::NodeId,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    DuplicateTrait(SymbolId),
    DuplicateTraitImpl(NodeId<Ty>, NodeId<TraitDef>),
    DuplicateTraitMethod(NodeId<FuncSig>),
    // TODO: This is way too generic
    BadTraitImpl,
}

impl Diagnostic for Error {
    fn severity(&self) -> Severity {
        Severity::Error
    }
}
