/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::{
    ops::{Deref, DerefMut},
    ptr::drop_in_place,
};

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tag {
    Dead = 0,
    None,
    Bool,
    Int,
    Float,
    String,
}

#[derive(Debug)]
pub struct Value {
    control: u64,
    payload: u64,
}

impl Value {
    fn make_control_block(rc: u64, tag: Tag) -> u64 {
        (rc & !(0xFFu64 << 56)) | ((tag as u64) << 56)
    }

    pub(crate) fn dead() -> Self {
        Self {
            control: Self::make_control_block(1, Tag::Dead),
            payload: 0,
        }
    }

    pub(crate) fn none() -> Self {
        Self {
            control: Self::make_control_block(1, Tag::None),
            payload: 0,
        }
    }

    pub(crate) fn bool(value: bool) -> Self {
        Self {
            control: Self::make_control_block(1, Tag::Bool),
            payload: value as u64,
        }
    }

    pub(crate) fn int(value: i64) -> Self {
        Self {
            control: Self::make_control_block(0, Tag::Int),
            payload: value.cast_unsigned(),
        }
    }

    pub(crate) fn float(value: f64) -> Self {
        Self {
            control: Self::make_control_block(0, Tag::Float),
            payload: value.to_bits(),
        }
    }

    pub(crate) fn string(value: &str) -> Self {
        Self {
            control: Self::make_control_block(0, Tag::String),
            payload: Box::into_raw(Box::new(value.to_string())) as u64,
        }
    }

    pub(crate) fn owned(mut value: Self) -> Self {
        value.inc_ref();
        value
    }

    pub fn tag(&self) -> Tag {
        let raw = self.control & (0xFFu64 >> 56);
        unsafe { std::mem::transmute(raw as u8) }
    }

    pub fn as_bool(&self) -> bool {
        debug_assert_eq!(self.tag(), Tag::Bool, "invalid as_bool");
        self.payload != 0
    }

    pub fn as_int(&self) -> i64 {
        debug_assert_eq!(self.tag(), Tag::Int, "invalid as_int");
        self.payload.cast_signed()
    }

    pub fn as_float(&self) -> f64 {
        debug_assert_eq!(self.tag(), Tag::Float, "invalid as_float");
        f64::from_bits(self.payload)
    }

    pub fn as_string(&self) -> &mut String {
        debug_assert_eq!(self.tag(), Tag::String, "invalid as_string");
        unsafe { &mut *(self.payload as *mut String) }
    }

    unsafe fn reset(&mut self) {
        debug_assert_ne!(self.tag(), Tag::Dead, "reset called on dead value");

        unsafe {
            match self.tag() {
                Tag::String => drop_in_place(self.payload as *mut String),
                _ => {} // Primitive; do nothing
            }
        }

        self.control = Self::make_control_block(self.control, Tag::Dead);
    }

    fn inc_ref(&mut self) {
        debug_assert!(self.control < (1u64 << 56), "control block overflow");
        self.control += 1;
    }

    fn dec_ref(&mut self) {
        debug_assert!(self.control > 0, "control block underflow");
        self.control -= 1;

        if self.control == 0 {
            unsafe { self.reset() };
        }
    }
}

impl Drop for Value {
    fn drop(&mut self) {
        unsafe { self.reset() };
    }
}

#[derive(Debug)]
pub struct ValueRef<'a>(&'a mut Value);

impl<'a> ValueRef<'a> {
    pub fn new(value: &'a mut Value) -> Self {
        value.inc_ref();
        Self(value)
    }

    pub fn clone(&mut self) -> ValueRef<'a> {
        Self::new(unsafe { (&mut *self.0 as *mut Value).as_mut().unwrap_unchecked() })
    }

    pub(crate) fn replace(&'a mut self, other: Self) {
        other.0.inc_ref();
        self.0.dec_ref();
        // This references the same memory when other is being dropped
        // It is sound though because it only takes the pointer
        self.0 = unsafe { &mut *(other.0 as *mut Value) };
    }
}

impl Deref for ValueRef<'_> {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl DerefMut for ValueRef<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

impl Drop for ValueRef<'_> {
    fn drop(&mut self) {
        self.0.dec_ref();
    }
}
