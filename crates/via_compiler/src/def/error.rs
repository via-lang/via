use crate::{
    clinic::{Diagnostic, Severity},
    def::{
        FnSig,
        traits::{TraitDef, TraitImplKey},
    },
    node::NodeId,
    symbol::Symbol,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    DuplicateDef(Symbol),
    DuplicateTraitImpl(TraitImplKey, NodeId<TraitDef>),
    DuplicateTraitMethod(NodeId<FnSig>),
    MissingGenericParam(Symbol),
    // TODO: This is way too generic
    BadTraitImpl,
}

impl Diagnostic for Error {
    fn severity(&self) -> Severity {
        Severity::Error
    }
}
