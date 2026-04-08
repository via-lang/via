pub mod error;
mod func;
mod module;
pub mod traits;

use std::collections::{HashMap, hash_map::Entry};

use via_macros::Arena;

use super::SymbolId;
use crate::{
    node::NodeId,
    sema::{SemContext, Ty, TySubst},
};

use error::*;

pub use {
    func::*,
    module::*,
    traits::{TraitDef, TraitImpl},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefId {
    FnId(NodeId<FnDef>),
    ModId(NodeId<ModDef>),
    TraitId(NodeId<TraitDef>),
}

#[derive(Arena, Debug, Default)]
pub struct DefContext {
    #[interner(map = "fnsig_map")]
    fnsig: Vec<FnSig>,
    fnsig_map: HashMap<FnSig, NodeId<FnSig>>,
    #[allocator]
    fn_def: Vec<FnDef>,
    #[allocator]
    mod_def: Vec<ModDef>,
    #[allocator]
    trait_def: Vec<TraitDef>,
    trait_map: HashMap<SymbolId, NodeId<TraitDef>>,
    trait_impls: HashMap<NodeId<TraitDef>, HashMap<NodeId<Ty>, TraitImpl>>,
}

impl DefContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_trait(&mut self, def: TraitDef) -> Result<NodeId<TraitDef>> {
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

    pub fn get_trait_impl(&self, class: NodeId<TraitDef>, ty: NodeId<Ty>) -> Option<&TraitImpl> {
        self.trait_impls.get(&class)?.get(&ty)
    }

    fn match_sig(
        &self,
        sem: &SemContext,
        class_sig: NodeId<FnSig>,
        impl_sig: NodeId<FnSig>,
    ) -> bool {
        let resolve = |ty: NodeId<Ty>| match &sem[ty] {
            Ty::Subst(subst) => sem.get_subst(*subst).unwrap_or(ty),
            _ => ty,
        };

        let class = &self[class_sig];
        let impl_ = &self[impl_sig];

        class.parms.len() == impl_.parms.len()
            && class
                .parms
                .iter()
                .zip(&impl_.parms)
                .all(|(&p, &i)| sem[resolve(p)] == sem[resolve(i)])
            && sem[resolve(class.ret)] == sem[resolve(impl_.ret)]
    }

    pub fn impl_trait(
        &mut self,
        sem: &mut SemContext,
        ty: NodeId<Ty>,
        mut imp: TraitImpl,
    ) -> Result<()> {
        if self.get_trait_impl(imp.class, ty).is_some() {
            return Err(Error::DuplicateTraitImpl(ty, imp.class));
        }

        let class = &self[imp.class];

        sem.define_subst(TySubst::This, ty);

        for (sym, sig) in &class.methods {
            match imp.impls.entry(*sym) {
                Entry::Occupied(e) if self.match_sig(sem, self[*e.get()].sig, *sig) => {}
                Entry::Occupied(_) | Entry::Vacant(_) => return Err(Error::BadTraitImpl),
            }
        }

        sem.remove_subst(TySubst::This);

        self.trait_impls
            .entry(imp.class)
            .or_default()
            .insert(ty, imp);

        Ok(())
    }
}
