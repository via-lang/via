use std::fmt;

use via_vm::NativeCallback;

use crate::{def::DefId, node::NodeId, sema::Ty, symbol::Symbol};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnSig {
    pub params: Vec<NodeId<Ty>>,
    pub result: NodeId<Ty>,
}

#[derive(Debug)]
pub struct FnDef {
    pub symbol: Symbol,
    pub parent: Option<DefId>,
    pub sig: NodeId<FnSig>,
    pub impl_: FnImpl,
}

#[derive(Debug, Clone, Copy)]
pub enum Intrin {
    IAdd,
    IAddF,
    FAdd,
    FAddI,
    ISub,
    ISubF,
    FSub,
    FSubI,
    IMul,
    IMulF,
    FMul,
    FMulI,
    IDiv,
    IDivF,
    FDiv,
    FDivI,
    IPow,
    IPowF,
    FPow,
    FPowI,
    IRem,
    FRem,
}

pub enum FnImpl {
    Intrin(Intrin),
    Native(Box<dyn NativeCallback>),
}

impl fmt::Debug for FnImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Intrin(intrin) => write!(f, "{intrin:?}"),
            Self::Native(native) => write!(f, "<native@{:p}>", native.as_ref() as *const _),
        }
    }
}
