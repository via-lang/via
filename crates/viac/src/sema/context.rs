use std::collections::{HashMap, hash_map::Entry};

use via_macros::Arena;

use super::{
    error::*,
    func::FuncSig,
    traits::{TraitDef, TraitImpl},
    ty::{MetaId, Ty},
};
use crate::{counter::Counter, module::symbol::SymbolId, node::NodeId};

#[derive(Arena)]
pub struct SemContext {
    #[interner(map = "ty_map")]
    ty: Vec<Ty>,
    ty_map: HashMap<Ty, NodeId<Ty>>,
    #[interner(map = "fnsig_map")]
    fnsig: Vec<FuncSig>,
    fnsig_map: HashMap<FuncSig, NodeId<FuncSig>>,
    #[allocator]
    trait_def: Vec<TraitDef>,
    trait_map: HashMap<SymbolId, NodeId<TraitDef>>,
    trait_impls: HashMap<NodeId<Ty>, TraitImpl>,
    metas: Counter<MetaId>,
    meta_solutions: HashMap<MetaId, NodeId<Ty>>,
}

impl SemContext {
    pub fn new() -> Self {
        Self {
            ty: Vec::new(),
            ty_map: HashMap::new(),
            fnsig: Vec::new(),
            fnsig_map: HashMap::new(),
            trait_def: Vec::new(),
            trait_map: HashMap::new(),
            trait_impls: HashMap::new(),
            metas: Counter::new(),
            meta_solutions: HashMap::new(),
        }
    }

    pub(super) fn register_trait(&mut self, def: TraitDef) -> Result<NodeId<TraitDef>> {
        let sym = def.sym;
        let id = self.alloc_trait_def(def);
        self.trait_map.insert(sym, id);
        Ok(id)
    }

    pub fn get_trait(&self, sym: SymbolId) -> Option<NodeId<TraitDef>> {
        self.trait_map.get(&sym).copied()
    }

    pub fn get_trait_impl(&self, ty: NodeId<Ty>) -> Option<&TraitImpl> {
        self.trait_impls.get(&ty)
    }

    pub fn impl_trait(&mut self, ty: NodeId<Ty>, imp: TraitImpl) -> Result<()> {
        match self.trait_impls.entry(ty) {
            Entry::Occupied(_) => Err(Error::DuplicateTrait),
            Entry::Vacant(e) => {
                e.insert(imp);
                Ok(())
            }
        }
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
            Entry::Occupied(_) => panic!("placeholder '{id:#?}' solved twice"),
            Entry::Vacant(e) => e.insert(ty),
        };
    }
}

impl Default for SemContext {
    fn default() -> Self {
        Self::new()
    }
}
