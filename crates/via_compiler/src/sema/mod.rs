mod ops;
mod ty;
mod value;

use std::collections::{HashMap, hash_map::Entry};

use via_macros::Arena;

use crate::{counter::Counter, macros::ice_panic, node::NodeId};

pub use {ops::*, ty::*, value::*};

#[derive(Arena, Debug, Default)]
pub struct SemContext {
    #[interner(map = "ty_map")]
    ty: Vec<Ty>,
    ty_map: HashMap<Ty, NodeId<Ty>>,
    metas: Counter<MetaId>,
    meta_solutions: HashMap<MetaId, NodeId<Ty>>,
    subst_env: HashMap<TySubst, NodeId<Ty>>,
}

impl SemContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_meta(&mut self) -> MetaId {
        self.metas.bump()
    }

    pub fn get_meta(&self, id: MetaId) -> Option<NodeId<Ty>> {
        self.meta_solutions
            .get(&id)
            .copied()
            .and_then(|ty| match self[ty] {
                Ty::Meta(other) => self.get_meta(other),
                _ => Some(ty),
            })
    }

    pub fn solve_meta(&mut self, id: MetaId, ty: NodeId<Ty>) {
        match self.meta_solutions.entry(id) {
            Entry::Occupied(_) => ice_panic!("placeholder '{id:#?}' solved twice"),
            Entry::Vacant(e) => e.insert(ty),
        };
    }

    pub fn get_subst(&self, parm: TySubst) -> Option<NodeId<Ty>> {
        self.subst_env.get(&parm).copied()
    }

    pub fn define_subst(&mut self, parm: TySubst, ty: NodeId<Ty>) {
        self.subst_env.insert(parm, ty);
    }

    pub fn remove_subst(&mut self, parm: TySubst) {
        self.subst_env.remove(&parm);
    }
}
