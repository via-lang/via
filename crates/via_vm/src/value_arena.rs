/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::marker::PhantomData;

use crate::value::{Tag, Value, ValueRef};

#[derive(Debug)]
pub struct Singletons {
    none: Value,
    true_: Value,
    false_: Value,
}

#[derive(Debug)]
pub struct ValueArena<'a, const S: usize> {
    sing: Singletons,
    inner: Box<[Value; S]>,
    _marker: PhantomData<&'a ()>,
}

impl<'a, const S: usize> ValueArena<'a, S> {
    pub fn new() -> Self {
        Self {
            sing: Singletons {
                none: Value::owned(Value::none()),
                true_: Value::owned(Value::bool(true)),
                false_: Value::owned(Value::bool(false)),
            },
            inner: Box::new(std::array::from_fn(|_| Value::dead())),
            _marker: PhantomData::default(),
        }
    }

    pub fn clone(&'a mut self, mut value: ValueRef<'a>) -> ValueRef<'a> {
        match value.tag() {
            Tag::None | Tag::Bool | Tag::Int | Tag::Float => value.clone(),
            Tag::String => self.get_string(value.as_string().as_str()),
            Tag::Dead => panic!("attempt to clone dead value"),
        }
    }

    pub fn get_none(&'a mut self) -> ValueRef<'a> {
        ValueRef::new(&mut self.sing.none)
    }

    pub fn get_true(&'a mut self) -> ValueRef<'a> {
        ValueRef::new(&mut self.sing.true_)
    }

    pub fn get_false(&'a mut self) -> ValueRef<'a> {
        ValueRef::new(&mut self.sing.false_)
    }

    fn find_empty_slot(&mut self) -> &mut Value {
        for slot in self.inner.iter_mut() {
            if slot.tag() == Tag::Dead {
                return slot;
            }
        }
        panic!("out of arena space")
    }

    pub fn get_int(&'a mut self, value: i64) -> ValueRef<'a> {
        let slot = self.find_empty_slot();
        *slot = Value::int(value);
        ValueRef::new(slot)
    }

    pub fn get_float(&'a mut self, value: f64) -> ValueRef<'a> {
        let slot = self.find_empty_slot();
        *slot = Value::float(value);
        ValueRef::new(slot)
    }

    pub fn get_string(&'a mut self, value: &str) -> ValueRef<'a> {
        let slot = self.find_empty_slot();
        *slot = Value::string(value);
        ValueRef::new(slot)
    }
}
