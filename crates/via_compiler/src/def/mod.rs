pub mod error;
mod func;
mod ns;
pub mod traits;

use std::collections::{HashMap, hash_map::Entry};

use via_macros::Arena;

use crate::{
    node::NodeId,
    sema::{SemContext, Ty, TySubst},
    symbol::SymbolId,
};

use error::*;

pub use {
    func::*,
    ns::*,
    traits::{TraitDef, TraitImpl},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefId {
    FnId(NodeId<FnDef>),
    ModId(NodeId<NsDef>),
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
    ns_def: Vec<NsDef>,
    #[allocator]
    trait_def: Vec<TraitDef>,
    trait_impls: HashMap<NodeId<TraitDef>, HashMap<NodeId<Ty>, TraitImpl>>,
    def_map: HashMap<SymbolId, DefId>,
}

impl DefContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_fn(&mut self, fun: FnDef) -> Result<NodeId<FnDef>> {
        let sym = fun.sym;
        if self.def_map.contains_key(&sym) {
            return Err(Error::DuplicateDef(sym));
        }

        let id = self.alloc_fn_def(fun);
        self.def_map.insert(sym, DefId::FnId(id));
        Ok(id)
    }

    pub fn register_trait(&mut self, def: TraitDef) -> Result<NodeId<TraitDef>> {
        let sym = def.sym;
        if self.def_map.contains_key(&sym) {
            return Err(Error::DuplicateDef(sym));
        }

        let id = self.alloc_trait_def(def);
        self.def_map.insert(sym, DefId::TraitId(id));
        Ok(id)
    }

    pub fn get(&self, sym: SymbolId) -> Option<DefId> {
        self.def_map.get(&sym).cloned()
    }

    pub fn get_trait(&self, sym: SymbolId) -> Option<NodeId<TraitDef>> {
        match self.def_map.get(&sym).cloned()? {
            DefId::TraitId(trait_id) => Some(trait_id),
            _ => None,
        }
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
