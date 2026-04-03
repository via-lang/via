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
    trait_impls: HashMap<NodeId<TraitDef>, HashMap<NodeId<Ty>, TraitImpl>>,
    metas: Counter<MetaId>,
    meta_solutions: HashMap<MetaId, NodeId<Ty>>,
}

impl SemContext {
    pub fn new() -> Self {
        Self::default()
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

    pub fn get_trait_impl(&self, proto: NodeId<TraitDef>, ty: NodeId<Ty>) -> Option<&TraitImpl> {
        self.trait_impls.get(&proto)?.get(&ty)
    }

    pub fn impl_trait(&mut self, ty: NodeId<Ty>, mut imp: TraitImpl) -> Result<()> {
        if self.get_trait_impl(imp.proto, ty).is_some() {
            return Err(Error::DuplicateTraitImpl(ty, imp.proto));
        }

        let proto = &self[imp.proto];
        let cmp_sig =
            |lhs: NodeId<FuncSig>, rhs: NodeId<FuncSig>, this_sub: Option<NodeId<Ty>>| -> bool {
                let resolve = |ty: NodeId<Ty>| match self[ty] {
                    Ty::This => this_sub.unwrap_or(ty),
                    _ => ty,
                };

                let lsig = &self[lhs];
                let rsig = &self[rhs];

                lsig.parms.len() == rsig.parms.len()
                    && lsig
                        .parms
                        .iter()
                        .zip(&rsig.parms)
                        .all(|(&l, &r)| self[resolve(l)] == self[resolve(r)])
                    && self[resolve(lsig.ret)] == self[resolve(rsig.ret)]
            };

        for (sym, sig) in &proto.funcs {
            match imp.impls.entry(*sym) {
                Entry::Occupied(e) if cmp_sig(e.get().0, *sig, Some(ty)) => {}
                Entry::Occupied(_) | Entry::Vacant(_) => return Err(Error::BadTraitImpl),
            }
        }

        self.trait_impls
            .entry(imp.proto)
            .or_default()
            .insert(ty, imp);

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
