use std::collections::{HashMap, hash_map::Entry};

use crate::{
    def::{
        DefContext, DefId, FnDef, FnImpl, FnSig, Intrin, TraitDef, TraitImpl,
        error::{Error, Result},
    },
    node::NodeId,
    sema::{SemContext, Ty},
    symbol::{SymbolId, SymbolTable},
};

pub trait IntoCanonTy: Clone {
    fn into_canon_ty(self, sem: &mut SemContext) -> NodeId<Ty>;
}

impl IntoCanonTy for Ty {
    fn into_canon_ty(self, sem: &mut SemContext) -> NodeId<Ty> {
        sem.intern_ty(self)
    }
}

impl IntoCanonTy for NodeId<Ty> {
    fn into_canon_ty(self, _: &mut SemContext) -> NodeId<Ty> {
        self
    }
}

pub struct FnStage0 {
    sym: SymbolId,
}

pub struct FnStage1 {
    sym: SymbolId,
    ret: NodeId<Ty>,
    parms: Vec<NodeId<Ty>>,
}

pub struct FnStage2 {
    sym: SymbolId,
    ret: NodeId<Ty>,
    parms: Vec<NodeId<Ty>>,
    parent: Option<DefId>,
    impl_: FnImpl,
}

#[must_use]
pub struct FnBuilder<'a, S> {
    st: &'a mut SymbolTable,
    sem: &'a mut SemContext,
    def: &'a mut DefContext,
    data: S,
}

impl<'a> FnBuilder<'a, FnStage0> {
    pub fn new(
        st: &'a mut SymbolTable,
        sem: &'a mut SemContext,
        def: &'a mut DefContext,
        sym: &str,
    ) -> Self {
        let sym = st.intern(sym);
        Self {
            st,
            sem,
            def,
            data: FnStage0 { sym },
        }
    }

    pub fn returns(self, ty: impl IntoCanonTy) -> FnBuilder<'a, FnStage1> {
        let ret = ty.into_canon_ty(self.sem);
        FnBuilder {
            st: self.st,
            sem: self.sem,
            def: self.def,
            data: FnStage1 {
                sym: self.data.sym,
                ret,
                parms: Vec::new(),
            },
        }
    }
}

impl<'a> FnBuilder<'a, FnStage1> {
    pub fn parameter(mut self, ty: impl IntoCanonTy) -> Self {
        let ty = ty.into_canon_ty(self.sem);
        self.data.parms.push(ty);
        self
    }

    pub fn with_body(self, parent: Option<DefId>, impl_: FnImpl) -> FnBuilder<'a, FnStage2> {
        FnBuilder {
            st: self.st,
            sem: self.sem,
            def: self.def,
            data: FnStage2 {
                sym: self.data.sym,
                ret: self.data.ret,
                parms: self.data.parms,
                parent,
                impl_,
            },
        }
    }
}

impl<'a> FnBuilder<'a, FnStage2> {
    pub fn build(self) -> FnDef {
        let sig = self.def.intern_fnsig(FnSig {
            parms: self.data.parms,
            ret: self.data.ret,
        });

        FnDef {
            sym: self.data.sym,
            parent: self.data.parent,
            sig,
            impl_: self.data.impl_,
        }
    }
}

#[must_use]
pub struct TraitBuilder<'a> {
    st: &'a mut SymbolTable,
    sem: &'a mut SemContext,
    def: &'a mut DefContext,
    sym: SymbolId,
    methods: HashMap<SymbolId, NodeId<FnSig>>,
}

impl<'a> TraitBuilder<'a> {
    pub fn new(
        st: &'a mut SymbolTable,
        sem: &'a mut SemContext,
        def: &'a mut DefContext,
        sym: &str,
    ) -> Self {
        let sym = st.intern(sym);
        Self {
            st,
            sem,
            def,
            sym,
            methods: HashMap::new(),
        }
    }

    pub fn method(
        &mut self,
        sym: &str,
        parms: &[impl IntoCanonTy],
        ret: impl IntoCanonTy,
    ) -> Result<&mut Self> {
        let sym = self.st.intern(sym);

        let ret = ret.into_canon_ty(self.sem);
        let parms = parms
            .iter()
            .cloned()
            .map(|p| p.into_canon_ty(self.sem))
            .collect::<Vec<_>>();

        let sig = self.def.intern_fnsig(FnSig { parms, ret });

        match self.methods.entry(sym) {
            Entry::Vacant(e) => {
                e.insert(sig);
                Ok(self)
            }
            Entry::Occupied(_) => Err(Error::DuplicateTraitMethod(sig)),
        }
    }

    pub fn finish(&self, parent: Option<DefId>) -> Result<TraitDef> {
        Ok(TraitDef {
            sym: self.sym,
            parent,
            methods: self.methods.clone(),
        })
    }
}

#[must_use]
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
        ty: impl IntoCanonTy,
        intrin: Intrin,
    ) -> Result<&mut Self> {
        let sym = self.st.intern(method_name);

        let ty = ty.into_canon_ty(self.sem);
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
