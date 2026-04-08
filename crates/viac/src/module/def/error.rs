use crate::{
    clinic::{Diagnostic, Severity},
    module::{
        def::{FnSig, traits::TraitDef},
        symbol::SymbolId,
    },
    node::NodeId,
    sema::Ty,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    DuplicateTrait(SymbolId),
    DuplicateTraitImpl(NodeId<Ty>, NodeId<TraitDef>),
    DuplicateTraitMethod(NodeId<FnSig>),
    // TODO: This is way too generic
    BadTraitImpl,
}

impl Diagnostic for Error {
    fn severity(&self) -> Severity {
        Severity::Error
    }
}
