/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::collections::{HashMap, hash_map::Entry};

use via_macros::Arena;

use super::{
    func::FuncSig,
    traits::{TraitDef, TraitImpl},
    ty::{MetaId, Ty},
};
use crate::{
    counter::Counter,
    node::{NodeId, NodeStore},
};

#[derive(Arena)]
pub struct SemContext {
    #[arena]
    ty: Vec<Ty>,
    #[arena]
    func_sig: Vec<FuncSig>,
    #[arena]
    trait_def: Vec<TraitDef>,
    trait_impls: HashMap<NodeId<Ty>, TraitImpl>,
    metas: Counter<MetaId>,
    meta_solutions: HashMap<MetaId, NodeId<Ty>>,
}

impl SemContext {
    pub fn new() -> Self {
        Self {
            ty: Vec::new(),
            func_sig: Vec::new(),
            trait_def: Vec::new(),
            trait_impls: HashMap::new(),
            metas: Counter::new(),
            meta_solutions: HashMap::new(),
        }
    }

    pub fn get_trait_impl(&self, ty: NodeId<Ty>) -> Option<&TraitImpl> {
        self.trait_impls.get(&ty)
    }

    pub fn impl_trait(&mut self, ty: NodeId<Ty>, imp: TraitImpl) -> bool {
        !self.trait_impls.contains_key(&ty) && self.trait_impls.insert(ty, imp).is_none()
    }

    pub fn next_meta(&mut self) -> MetaId {
        self.metas.next()
    }

    pub fn get_meta(&mut self, id: MetaId) -> Option<NodeId<Ty>> {
        self.meta_solutions.get(&id).cloned()
    }

    pub fn solve_meta(&mut self, id: MetaId, ty: NodeId<Ty>) {
        match self.meta_solutions.entry(id) {
            Entry::Occupied(_) => panic!("placeholder '{id:#?}' solved twice"),
            Entry::Vacant(e) => e.insert(ty),
        };
    }
}
