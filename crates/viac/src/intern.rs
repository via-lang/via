/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::{collections::HashMap, hash::Hash};

use typed_arena::Arena;

#[derive(Debug, Clone, Eq, Hash)]
pub struct Interned<'t, T: ?Sized> {
    ptr: &'t T,
}

impl<'t, T: ?Sized> Interned<'t, T> {
    pub(super) fn new(ptr: &'t T) -> Self {
        Self { ptr }
    }
}

impl<'t, T: ?Sized> PartialEq for Interned<'t, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.ptr, other.ptr)
    }
}

impl<'t, T: ?Sized> AsRef<T> for Interned<'t, T> {
    fn as_ref(&self) -> &T {
        self.ptr
    }
}

impl<T: ?Sized + Clone> Copy for Interned<'_, T> {}

pub struct Interner<T>
where
    T: Clone + Eq + Hash,
{
    arena: Arena<T>,
    map: HashMap<T, *const T>,
}

impl<'t, T> Interner<T>
where
    T: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            arena: Arena::new(),
            map: HashMap::new(),
        }
    }

    pub fn intern(&'t mut self, value: T) -> Interned<'t, T> {
        if let Some(&ptr) = self.map.get(&value) {
            return unsafe { Interned::new(&*ptr) };
        }

        let ptr = self.arena.alloc(value) as *const T;
        let key = unsafe { (*ptr).clone() };

        self.map.insert(key, ptr);

        unsafe { Interned::new(&*ptr) }
    }
}

impl<T> Default for Interner<T>
where
    T: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
