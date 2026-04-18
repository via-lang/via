use crate::{
    clinic::{Diagnostic, Severity},
    def::{FnSig, traits::TraitDef},
    node::NodeId,
    sema::Ty,
    symbol::SymbolId,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    DuplicateDef(SymbolId),
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
