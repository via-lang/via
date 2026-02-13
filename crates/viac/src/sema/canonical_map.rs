/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::{
    collections::BTreeMap,
    hash::{Hash, Hasher},
};

use delegate::delegate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalMap<K, V>(BTreeMap<K, V>);

impl<K: Hash + Ord, V: Hash> CanonicalMap<K, V> {
    delegate! {
        to self.0 {
            pub fn insert(&mut self, key: K, value: V) -> Option<V>;
            pub fn get(&self, key: &K) -> Option<&V>;
        }
    }
}

impl<K: Hash, V: Hash> Hash for CanonicalMap<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (k, v) in &self.0 {
            k.hash(state);
            v.hash(state);
        }
    }
}
