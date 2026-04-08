pub mod arith;

use std::collections::{HashMap, hash_map::Entry};

use super::{DefId, FnImpl, Intrin, error::*, func::FnSig};
use crate::{
    module::{
        def::{DefContext, FnDef},
        symbol::{SymbolId, SymbolTable},
    },
    node::NodeId,
    sema::{SemContext, Ty},
};

use arith::register_builtin_arith;

#[derive(Debug)]
pub struct TraitDef {
    pub sym: SymbolId,
    pub parent: Option<DefId>,
    pub methods: HashMap<SymbolId, NodeId<FnSig>>,
}

#[derive(Debug)]
pub struct TraitImpl {
    pub class: NodeId<TraitDef>,
    pub impls: HashMap<SymbolId, NodeId<FnDef>>,
}

pub fn register_builtin(
    st: &mut SymbolTable,
    sem: &mut SemContext,
    def: &mut DefContext,
) -> Result<()> {
    register_builtin_arith(st, sem, def)?;
    Ok(())
}

pub struct TraitBuilder<'a> {
    st: &'a mut SymbolTable,
    def: &'a mut DefContext,
    sym: SymbolId,
    methods: HashMap<SymbolId, NodeId<FnSig>>,
}

impl<'a> TraitBuilder<'a> {
    pub fn new(st: &'a mut SymbolTable, def: &'a mut DefContext, sym: &str) -> Self {
        let sym = st.intern(sym);
        Self {
            st,
            def,
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

        let sig = self.def.intern_fnsig(FnSig { parms, ret });

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
            parent: None,
            methods: self.methods.clone(),
        })
    }
}

pub struct ImplBuilder<'a> {
    st: &'a mut SymbolTable,
    sem: &'a mut SemContext,
    def: &'a mut DefContext,
}

impl<'a> ImplBuilder<'a> {
    pub fn new(st: &'a mut SymbolTable, sem: &'a mut SemContext, def: &'a mut DefContext) -> Self {
        Self { st, sem, def }
    }

    pub fn impl_intr(
        &mut self,
        class: NodeId<TraitDef>,
        method_name: &str,
        ty: NodeId<Ty>,
        intrin: Intrin,
    ) -> Result<&mut Self> {
        let sym = self.st.intern(method_name);
        let sig = self.def.intern_fnsig(FnSig {
            parms: vec![ty, ty], // concrete type, not This
            ret: ty,
        });

        let fn_def = self.def.alloc_fn_def(FnDef {
            sym,
            parent: Some(DefId::TraitId(class)),
            sig,
            impl_: FnImpl::Intrin(intrin),
        });

        self.def.impl_trait(
            self.sem,
            ty,
            TraitImpl {
                class,
                impls: HashMap::from([(sym, fn_def)]),
            },
        )?;

        Ok(self)
    }
}
