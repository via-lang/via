use std::collections::{HashMap, hash_map::Entry};

use via_macros::Arena;

use super::{
    error::*,
    func::FuncSig,
    traits::{TraitDef, TraitImpl},
    ty::{MetaId, Ty},
};
use crate::{counter::Counter, module::symbol::SymbolId, node::NodeId};

#[derive(Arena, Default)]
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
        Self::default()
    }

    fn subst_this(&self, this: NodeId<Ty>, ty: NodeId<Ty>) -> NodeId<Ty> {
        match &self[ty] {
            Ty::This => this,
            _ => ty,
        }
    }

    fn subst_this_in_fnsig(&mut self, this: NodeId<Ty>, sig: NodeId<FuncSig>) {
        todo!()
    }

    fn subst_this_in_impl(&mut self, this: NodeId<Ty>, imp: &mut TraitImpl) {
        todo!()
    }

    pub(super) fn register_trait(&mut self, def: TraitDef) -> Result<NodeId<TraitDef>> {
        let sym = def.sym;

        if self.trait_map.contains_key(&sym) {
            return Err(Error::DuplicateTrait(sym));
        }

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

    pub fn impl_trait(&mut self, ty: NodeId<Ty>, mut imp: TraitImpl) -> Result<()> {
        if self.trait_impls.contains_key(&ty) {
            return Err(Error::DuplicateTraitImpl(ty, imp.proto));
        }

        self.subst_this_in_impl(ty, &mut imp);

        let proto = &self[imp.proto];

        for (sym, sig) in &proto.funcs {
            match imp.impls.entry(*sym) {
                Entry::Occupied(e) if e.get().0 == *sig => {}
                Entry::Occupied(_) | Entry::Vacant(_) => return Err(Error::BadTraitImpl),
            }
        }

        self.trait_impls.insert(ty, imp);
        Ok(())
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
