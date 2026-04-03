use std::collections::{HashMap, hash_map::Entry};

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
    methods: HashMap<SymbolId, NodeId<FuncSig>>,
}

impl<'a> TraitBuilder<'a> {
    pub fn new(st: &'a mut SymbolTable, sem: &'a mut SemContext, sym: &str) -> Self {
        let sym = st.intern(sym);
        Self {
            st,
            sem,
            sym,
            methods: HashMap::new(),
        }
    }

    pub fn method(
        &mut self,
        sym: &str,
        parms: &[NodeId<Ty>],
        ret: NodeId<Ty>,
    ) -> Result<&mut Self> {
        let sym = self.st.intern(sym);
        let parms = Vec::from(parms);

        let sig = self.sem.intern_fnsig(FuncSig { sym, parms, ret });

        match self.methods.entry(sym) {
            Entry::Vacant(e) => {
                e.insert(sig);
                Ok(self)
            }
            Entry::Occupied(_) => Err(Error::DuplicateTraitMethod(sig)),
        }
    }

    pub fn finish(&self) -> Result<TraitDef> {
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

    pub fn impl_intr(
        &mut self,
        proto: NodeId<TraitDef>,
        method_name: &str,
        ty: NodeId<Ty>,
        intrin: Intrinsic,
    ) -> Result<&mut Self> {
        let sym = self.st.intern(method_name);
        let sig = self.sem.intern_fnsig(FuncSig {
            sym,
            parms: vec![ty, ty], // concrete type, not This
            ret: ty,
        });

        self.sem.impl_trait(
            ty,
            TraitImpl {
                proto,
                impls: HashMap::from([(sym, (sig, FuncImpl::Intrin(intrin)))]),
            },
        )?;

        Ok(self)
    }
}
