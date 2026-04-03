use crate::{
    clinic::{Diagnostic, Severity},
    node::NodeId,
    sema::ty::{MetaId, Ty},
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    IntOutOfRange,
    TypeMismatch(NodeId<Ty>, NodeId<Ty>),
    UnsolvedMetavar(MetaId),
    InvalidBinaryOp,
}

impl Diagnostic for Error {
    fn severity(&self) -> Severity {
        Severity::Error
    }
}
