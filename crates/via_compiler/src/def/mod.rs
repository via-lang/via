pub mod error;
mod func;
mod ns;
pub mod traits;

use std::collections::HashMap;

use via_macros::Arena;

use crate::{
    def::traits::TraitImplKey,
    node::NodeId,
    sema::{SemContext, Ty, TySubst},
    symbol::Symbol,
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
    NsId(NodeId<NsDef>),
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
    trait_impls: HashMap<NodeId<TraitDef>, HashMap<TraitImplKey, TraitImpl>>,
    def_map: HashMap<Symbol, DefId>,
}

impl DefContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_fn(&mut self, fun: FnDef) -> Result<NodeId<FnDef>> {
        let symbol = fun.symbol;
        if self.def_map.contains_key(&symbol) {
            return Err(Error::DuplicateDef(symbol));
        }

        let id = self.alloc_fn_def(fun);
        self.def_map.insert(symbol, DefId::FnId(id));
        Ok(id)
    }

    pub fn register_trait(&mut self, def: TraitDef) -> Result<NodeId<TraitDef>> {
        let symbol = def.symbol;
        if self.def_map.contains_key(&symbol) {
            return Err(Error::DuplicateDef(symbol));
        }

        let id = self.alloc_trait_def(def);
        self.def_map.insert(symbol, DefId::TraitId(id));
        Ok(id)
    }

    pub fn get(&self, symbol: Symbol) -> Option<DefId> {
        self.def_map.get(&symbol).cloned()
    }

    pub fn get_trait(&self, symbol: Symbol) -> Option<NodeId<TraitDef>> {
        match self.def_map.get(&symbol).cloned()? {
            DefId::TraitId(trait_id) => Some(trait_id),
            _ => None,
        }
    }

    pub fn get_trait_impl(
        &self,
        class: NodeId<TraitDef>,
        key: &TraitImplKey,
    ) -> Option<&TraitImpl> {
        self.trait_impls.get(&class)?.get(&key)
    }

    pub fn impl_trait(
        &mut self,
        sem_ctxt: &mut SemContext,
        key: impl Into<TraitImplKey>,
        imp: TraitImpl,
    ) -> Result<()> {
        let key = key.into();
        if self.get_trait_impl(imp.class, &key).is_some() {
            return Err(Error::DuplicateTraitImpl(key, imp.class));
        }

        let class = &self[imp.class];

        sem_ctxt.define_subst(TySubst::This, key.this);

        for (symbol, method_def) in &class.methods {
            let impl_fn = match imp.methods.get(symbol) {
                Some(m) => m,
                None => return Err(Error::BadTraitImpl),
            };

            let class_sig = &self[method_def.sig];
            let impl_sig = &self[self[impl_fn.def].sig];

            let normalize = |ty: NodeId<Ty>| -> Result<NodeId<Ty>> {
                match sem_ctxt[ty] {
                    Ty::Subst(TySubst::This) => Ok(key.this),
                    Ty::Subst(TySubst::Generic(s)) => {
                        let idx = class.generics.iter().position(|g| g.symbol == s).unwrap();
                        let generic = &class.generics[idx];
                        imp.generics
                            .get(idx)
                            .cloned()
                            .or(generic.default)
                            .ok_or(Error::MissingGenericParam(s))
                    }
                    Ty::Subst(TySubst::Assoc(s)) => Ok(imp.assoc_types[&s]),
                    _ => Ok(ty),
                }
            };

            let class_parms = class_sig
                .params
                .iter()
                .map(|&p| normalize(p))
                .collect::<Result<Vec<_>>>()?;

            let impl_parms = impl_sig
                .params
                .iter()
                .map(|&p| normalize(p))
                .collect::<Result<Vec<_>>>()?;

            let parms_match = class_parms
                .iter()
                .zip(impl_parms.iter())
                .all(|(c, i)| sem_ctxt[*c] == sem_ctxt[*i]);

            let ret_match =
                sem_ctxt[normalize(class_sig.result)?] == sem_ctxt[normalize(impl_sig.result)?];

            if class_sig.params.len() != impl_sig.params.len() || !parms_match || !ret_match {
                return Err(Error::BadTraitImpl);
            }
        }

        sem_ctxt.remove_subst(TySubst::This);

        self.trait_impls
            .entry(imp.class)
            .or_default()
            .insert(key, imp);

        Ok(())
    }
}
