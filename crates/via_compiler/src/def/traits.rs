use std::collections::HashMap;

use crate::{def::FnDef, node::NodeId, sema::TyParam, symbol::Symbol};

use super::{DefId, FnSig, Ty};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraitBound {
    Impl(TraitImplKey),
    NotImpl(TraitImplKey),
}

#[derive(Debug)]
pub struct MethodDef {
    pub sig: NodeId<FnSig>,
}

#[derive(Debug)]
pub struct TraitDef {
    pub symbol: Symbol,
    pub parent: Option<DefId>,
    pub generics: Vec<TyParam>,
    pub assoc_types: Vec<TyParam>,
    pub methods: HashMap<Symbol, MethodDef>,
}

#[derive(Debug)]
pub struct MethodImpl {
    pub def: NodeId<FnDef>,
}

#[derive(Debug)]
pub struct TraitImpl {
    pub class: NodeId<TraitDef>,
    pub generics: Vec<NodeId<Ty>>,
    pub methods: HashMap<Symbol, MethodImpl>,
    pub assoc_types: HashMap<Symbol, NodeId<Ty>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitImplKey {
    pub this: NodeId<Ty>,
    pub generics: Vec<NodeId<Ty>>,
}

impl TraitImplKey {
    pub fn new<I, E>(this: NodeId<Ty>, params: I) -> Self
    where
        I: IntoIterator<Item = E>,
        E: Into<NodeId<Ty>>,
    {
        Self {
            this,
            generics: params.into_iter().map(Into::into).collect(),
        }
    }
}
