use std::collections::{HashMap, HashSet, hash_map::Entry};

use itertools::Itertools;
use string_interner::{DefaultStringInterner, DefaultSymbol};

use crate::{
    IntoSymbol, StringInterner, Symbol,
    def::{
        DefContext, DefId, FnDef, FnImpl, FnSig, Intrin, TraitDef, TraitImpl,
        error::{Error, Result},
        traits::{MethodDef, MethodImpl, TraitBound, TraitImplKey},
    },
    node::NodeId,
    sema::{SemContext, Ty, TyParam, TySubst},
};

pub trait IntoCanonTy: Clone {
    fn into_canon_ty(self, sem_ctxt: &mut SemContext) -> NodeId<Ty>;
}

pub struct FnStage0 {
    symbol: DefaultSymbol,
}

pub struct FnStage1 {
    symbol: DefaultSymbol,
    result: NodeId<Ty>,
    params: Vec<NodeId<Ty>>,
}

pub struct FnStage2 {
    symbol: DefaultSymbol,
    result: NodeId<Ty>,
    params: Vec<NodeId<Ty>>,
    parent: Option<DefId>,
    impl_: FnImpl,
}

#[must_use]
pub struct FnBuilder<'a, S> {
    interner: &'a mut DefaultStringInterner,
    sem_ctxt: &'a mut SemContext,
    def_ctxt: &'a mut DefContext,
    data: S,
}

#[must_use]
pub struct TraitBuilder<'a> {
    interner: &'a mut DefaultStringInterner,
    sem_ctxt: &'a mut SemContext,
    def_ctxt: &'a mut DefContext,
    symbol: DefaultSymbol,
    generics: Vec<TyParam>,
    assoc_types: Vec<TyParam>,
    methods: HashMap<DefaultSymbol, MethodDef>,
}

#[must_use]
pub struct TraitImplBuilder<'a> {
    interner: &'a mut DefaultStringInterner,
    sem_ctxt: &'a mut SemContext,
    def_ctxt: &'a mut DefContext,
    class: NodeId<TraitDef>,
    this: NodeId<Ty>,
    generics: Vec<NodeId<Ty>>,
    assoc_types: HashMap<DefaultSymbol, NodeId<Ty>>,
    methods: HashMap<Symbol, NodeId<FnDef>>,
}

macro_rules! assoc {
    ($name:expr) => {
        crate::sema::Ty::Subst(crate::sema::TySubst::Assoc($name))
    };
}

macro_rules! generic {
    ($name:expr) => {
        crate::sema::Ty::Subst(crate::sema::TySubst::Generic($name))
    };
}

pub(crate) use {assoc, generic};

impl IntoCanonTy for Ty {
    fn into_canon_ty(self, sem_ctxt: &mut SemContext) -> NodeId<Ty> {
        sem_ctxt.intern_ty(self)
    }
}

impl IntoCanonTy for NodeId<Ty> {
    fn into_canon_ty(self, _: &mut SemContext) -> NodeId<Ty> {
        self
    }
}

impl<'a> FnBuilder<'a, FnStage0> {
    pub fn new(
        interner: &'a mut StringInterner,
        sem_ctxt: &'a mut SemContext,
        def_ctxt: &'a mut DefContext,
        symbol: impl IntoSymbol,
    ) -> Self {
        let symbol = symbol.into_symbol(interner);
        Self {
            interner,
            sem_ctxt,
            def_ctxt,
            data: FnStage0 { symbol },
        }
    }

    pub fn returns(self, ty: impl IntoCanonTy) -> FnBuilder<'a, FnStage1> {
        let result = ty.into_canon_ty(self.sem_ctxt);
        FnBuilder {
            interner: self.interner,
            sem_ctxt: self.sem_ctxt,
            def_ctxt: self.def_ctxt,
            data: FnStage1 {
                symbol: self.data.symbol,
                result,
                params: Vec::new(),
            },
        }
    }
}

impl<'a> FnBuilder<'a, FnStage1> {
    pub fn parameter(mut self, ty: impl IntoCanonTy) -> Self {
        let ty = ty.into_canon_ty(self.sem_ctxt);
        self.data.params.push(ty);
        self
    }

    pub fn with_body(self, parent: Option<DefId>, impl_: FnImpl) -> FnBuilder<'a, FnStage2> {
        FnBuilder {
            interner: self.interner,
            sem_ctxt: self.sem_ctxt,
            def_ctxt: self.def_ctxt,
            data: FnStage2 {
                symbol: self.data.symbol,
                result: self.data.result,
                params: self.data.params,
                parent,
                impl_,
            },
        }
    }
}

impl<'a> FnBuilder<'a, FnStage2> {
    #[allow(unused)]
    pub fn build(self) -> FnDef {
        let sig = self.def_ctxt.intern_fnsig(FnSig {
            params: self.data.params,
            result: self.data.result,
        });

        FnDef {
            symbol: self.data.symbol,
            parent: self.data.parent,
            sig,
            impl_: self.data.impl_,
        }
    }

    pub fn register(self) -> Result<NodeId<FnDef>> {
        let sig = self.def_ctxt.intern_fnsig(FnSig {
            params: self.data.params,
            result: self.data.result,
        });

        self.def_ctxt.register_fn(FnDef {
            symbol: self.data.symbol,
            parent: self.data.parent,
            sig,
            impl_: self.data.impl_,
        })
    }
}

impl<'a> TraitBuilder<'a> {
    pub fn new(
        interner: &'a mut DefaultStringInterner,
        sem_ctxt: &'a mut SemContext,
        def_ctxt: &'a mut DefContext,
        symbol: impl IntoSymbol,
    ) -> Self {
        let symbol = symbol.into_symbol(interner);
        Self {
            interner,
            sem_ctxt,
            def_ctxt,
            symbol,
            generics: vec![],
            methods: HashMap::new(),
            assoc_types: vec![],
        }
    }

    pub fn generic(
        mut self,
        name: impl IntoSymbol,
        bounds: impl Into<HashSet<TraitBound>>,
        default: Option<impl IntoCanonTy>,
    ) -> Self {
        let symbol = name.into_symbol(self.interner);
        self.generics.push(TyParam {
            symbol,
            bounds: bounds.into(),
            default: default.map(|d| d.into_canon_ty(self.sem_ctxt)),
        });
        self
    }

    pub fn assoc(
        mut self,
        name: impl IntoSymbol,
        bounds: impl Into<HashSet<TraitBound>>,
        default: Option<impl IntoCanonTy>,
    ) -> Self {
        let symbol = name.into_symbol(self.interner);
        self.assoc_types.push(TyParam {
            symbol,
            bounds: bounds.into(),
            default: default.map(|d| d.into_canon_ty(self.sem_ctxt)),
        });
        self
    }

    pub fn method(
        mut self,
        name: impl IntoSymbol,
        params: &[impl IntoCanonTy],
        result: impl IntoCanonTy,
    ) -> Result<Self> {
        let symbol = name.into_symbol(self.interner);
        let sig = self.def_ctxt.intern_fnsig(FnSig {
            params: params
                .iter()
                .cloned()
                .map(|p| p.into_canon_ty(self.sem_ctxt))
                .collect_vec(),
            result: result.into_canon_ty(self.sem_ctxt),
        });
        match self.methods.entry(symbol) {
            Entry::Vacant(e) => {
                e.insert(MethodDef { sig });
                Ok(self)
            }
            Entry::Occupied(_) => Err(Error::DuplicateTraitMethod(sig)),
        }
    }

    pub fn register(self, parent: Option<impl Into<DefId>>) -> Result<NodeId<TraitDef>> {
        self.def_ctxt.register_trait(TraitDef {
            symbol: self.symbol,
            parent: parent.map(Into::into),
            generics: self.generics,
            methods: self.methods,
            assoc_types: self.assoc_types,
        })
    }
}

impl<'a> TraitImplBuilder<'a> {
    pub fn new(
        interner: &'a mut DefaultStringInterner,
        sem_ctxt: &'a mut SemContext,
        def_ctxt: &'a mut DefContext,
        class: NodeId<TraitDef>,
        this: impl IntoCanonTy,
    ) -> Self {
        let this = this.into_canon_ty(sem_ctxt);
        Self {
            interner,
            sem_ctxt,
            def_ctxt,
            class,
            this,
            generics: vec![],
            assoc_types: HashMap::new(),
            methods: HashMap::new(),
        }
    }

    pub fn generic(mut self, ty: impl IntoCanonTy) -> Self {
        let ty = ty.into_canon_ty(self.sem_ctxt);
        self.generics.push(ty);
        self
    }

    pub fn assoc(mut self, name: impl IntoSymbol, ty: impl IntoCanonTy) -> Self {
        let symbol = name.into_symbol(self.interner);
        let ty = ty.into_canon_ty(self.sem_ctxt);
        self.assoc_types.insert(symbol, ty);
        self
    }

    pub fn method_intrin(mut self, name: impl IntoSymbol, intrin: Intrin) -> Self {
        let symbol = name.into_symbol(self.interner);
        let class = &self.def_ctxt[self.class];
        let method_sig_id = class.methods[&symbol].sig;
        let method_sig = self.def_ctxt[method_sig_id].clone();

        let resolve = |ty: NodeId<Ty>| -> NodeId<Ty> {
            match &self.sem_ctxt[ty] {
                Ty::Subst(TySubst::This) => self.this,
                Ty::Subst(TySubst::Assoc(s)) => self.assoc_types.get(s).copied().unwrap_or(ty),
                _ => ty,
            }
        };

        let params = method_sig.params.iter().map(|&p| resolve(p)).collect();
        let result = resolve(method_sig.result);

        let sig = self.def_ctxt.intern_fnsig(FnSig { params, result });

        let fn_def = self.def_ctxt.alloc_fn_def(FnDef {
            symbol,
            parent: Some(DefId::TraitId(self.class)),
            sig,
            impl_: FnImpl::Intrin(intrin),
        });

        self.methods.insert(symbol, fn_def);
        self
    }

    pub fn finish(self) -> Result<()> {
        let key = TraitImplKey::new(self.this, self.generics.clone());
        let imp = TraitImpl {
            class: self.class,
            generics: self.generics,
            methods: self
                .methods
                .into_iter()
                .map(|(symbol, def)| (symbol, MethodImpl { def }))
                .collect(),
            assoc_types: self.assoc_types,
        };

        self.def_ctxt.impl_trait(self.sem_ctxt, key, imp)
    }
}
