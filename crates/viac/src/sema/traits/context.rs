/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::collections::HashMap;

use delegate::delegate;

use super::{super::ty::Ty, Trait, imp::TraitImpl};
use crate::intern::{Interned, Interner};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplKey<'cx> {
    pub this: Interned<'cx, Ty<'cx>>,
    pub class: Interned<'cx, Trait<'cx>>,
}

pub struct TraitContext<'cx> {
    traits: Interner<Trait<'cx>>,
    impls: Interner<TraitImpl<'cx>>,
    impl_index: HashMap<ImplKey<'cx>, Interned<'cx, TraitImpl<'cx>>>,
}

impl<'cx> TraitContext<'cx> {
    pub fn probe_impl(&'cx self, key: ImplKey<'cx>) -> bool {
        self.get_impl(&key).is_some()
    }

    pub fn intern_impl(
        &'cx mut self,
        this: Interned<'cx, Ty<'cx>>,
        imp: TraitImpl<'cx>,
    ) -> Interned<'cx, TraitImpl<'cx>> {
        let imp = self.impls.intern(imp);
        self.impl_index.insert(
            ImplKey {
                this,
                class: imp.as_ref().class,
            },
            imp,
        );
        imp
    }

    delegate! {
        to self.traits {
            fn intern(&'cx mut self, value: Trait<'cx>)
                -> Interned<'cx, Trait<'cx>>;
        }
        to self.impl_index {
            #[call(get)]
            fn get_impl(&'cx self, key: &ImplKey<'cx>)
                -> Option<&Interned<'cx, TraitImpl<'cx>>>;
        }
    }
}
