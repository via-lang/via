use std::collections::{HashMap, HashSet};

use super::{
    super::{
        context::SemContext,
        error::*,
        func::{FuncImpl, FuncSig, Intrinsic},
        ty::Ty,
    },
    TraitDef, TraitImpl,
};
use crate::{
    module::symbol::{SymbolId, SymbolTable},
    node::NodeId,
};

pub struct TraitBuilder<'a> {
    st: &'a mut SymbolTable,
    sem: &'a mut SemContext,
    sym: SymbolId,
    methods: Vec<NodeId<FuncSig>>,
}

impl<'a> TraitBuilder<'a> {
    pub fn new(st: &'a mut SymbolTable, sem: &'a mut SemContext, sym: &str) -> Self {
        let sym = st.intern(sym);
        Self {
            st,
            sem,
            sym,
            methods: Vec::new(),
        }
    }

    pub fn method(&mut self, sym: &str, parms: &[NodeId<Ty>], ret: NodeId<Ty>) -> &mut Self {
        let sig = self.sem.intern_fnsig(FuncSig {
            sym: self.st.intern(sym),
            parms: Vec::from(parms),
            ret,
        });

        self.methods.push(sig);
        self
    }

    pub fn finish(&self) -> Result<TraitDef> {
        let unique: HashSet<_> = self.methods.iter().map(|id| self.sem[*id].sym).collect();

        (unique.len() == self.methods.len())
            .then_some(self)
            .ok_or(Error::DuplicateTraitMethod)?;

        Ok(TraitDef {
            sym: self.sym,
            funcs: self.methods.clone(),
        })
    }
}

pub struct ImplBuilder<'a> {
    st: &'a mut SymbolTable,
    sem: &'a mut SemContext,
}

impl<'a> ImplBuilder<'a> {
    pub fn new(st: &'a mut SymbolTable, sem: &'a mut SemContext) -> Self {
        Self { st, sem }
    }

    pub fn register(
        &mut self,
        trait_name: &str,
        method_name: &str,
        parms: Vec<NodeId<Ty>>,
        ret: NodeId<Ty>,
        intrin: Intrinsic,
    ) -> Result<&mut Self> {
        let this = self.sem.intern_ty(Ty::This);
        let proto = TraitBuilder::new(self.st, self.sem, trait_name)
            .method(method_name, &[this, this], this)
            .finish()?;

        let proto = self.sem.register_trait(proto)?;
        let sig = self.sem.intern_fnsig(FuncSig {
            sym: self.st.intern(method_name),
            parms,
            ret,
        });

        self.sem.impl_trait(
            ret,
            TraitImpl {
                proto,
                impls: HashMap::from([(sig, FuncImpl::Intrin(intrin))]),
            },
        )?;

        Ok(self)
    }
}
