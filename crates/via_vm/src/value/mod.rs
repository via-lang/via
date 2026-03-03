/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

#[cfg(debug_assertions)]
use std::any::TypeId;

pub trait Marker {}

impl Marker for () {}
impl Marker for bool {}
impl Marker for i64 {}
impl Marker for f64 {}

#[derive(Debug)]
pub struct Value {
    rc: usize,
    inner: u64,
    #[cfg(debug_assertions)]
    tag: TypeId,
}

#[derive(Debug)]
pub struct ValueRef<'a>(&'a mut Value);

impl<'a> ValueRef<'a> {
    pub fn new(value: &'a mut Value) -> Self {
        Self(value)
    }

    pub fn coerce<T: Value>(&self) -> &mut T {
        debug_assert_eq!(self.0.tag, TypeId::of::<T>(), "erronous value coercion");
        unsafe { &mut *(self.0 as *mut dyn Value as *mut T) }
    }
}
