/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

#[derive(Debug)]
pub enum Payload {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct Value {
    pub(super) rc: usize,
    pub(super) inner: Payload,
}

impl Value {
    pub fn new(payload: Payload) -> Self {
        Self {
            rc: 0,
            inner: payload,
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {}
}

#[derive(Debug)]
pub struct ValueRef<'a>(&'a mut Value);

impl<'a> ValueRef<'a> {
    pub fn new(value: &'a mut Value) -> Self {
        value.rc += 1;
        Self(value)
    }
}

impl<'a> Drop for ValueRef<'a> {
    fn drop(&mut self) {
        debug_assert!(self.0.rc > 0, "double free on value");
        self.0.rc -= 1;

        if self.0.rc == 0 {
            drop(*self.0);
        }
    }
}

pub trait IntoValue {
    fn into_value(self) -> Value;
}
