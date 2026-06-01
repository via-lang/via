use std::collections::HashSet;

use via_macros::Id;

use crate::{def::traits::TraitBound, node::NodeId, symbol::Symbol};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ty {
    Unit,
    Bool,
    Int,
    Float,
    String,
    Vector(NodeId<Ty>),
    Array { ty: NodeId<Ty>, size: ConstSubst },
    Map { key: NodeId<Ty>, value: NodeId<Ty> },
    Meta(MetaId),
    Subst(TySubst),
}

#[derive(Debug, Clone)]
pub struct TyParam {
    pub symbol: Symbol,
    pub bounds: HashSet<TraitBound>,
    pub default: Option<NodeId<Ty>>,
}

impl TyParam {
    pub fn new<I>(symbol: Symbol, bounds: I, default: Option<NodeId<Ty>>) -> Self
    where
        I: IntoIterator<Item = TraitBound>,
    {
        Self {
            symbol,
            bounds: bounds.into_iter().collect(),
            default,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TySubst {
    This,
    Generic(Symbol),
    Assoc(Symbol),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstSubst {
    Bool(bool),
    Int(i64),
}

#[derive(Id)]
#[id(inner = u32)]
pub struct MetaId(u32);
