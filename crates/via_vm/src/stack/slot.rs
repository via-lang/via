/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::value::Value;

#[derive(Debug)]
pub enum SlotKind {
    Value,
    Frame,
}

#[derive(Debug)]
pub struct Slot {
    #[cfg(debug_assertions)]
    pub kind: SlotKind,
    pub ptr: usize,
}

impl Slot {
    pub fn value(ptr: *mut Value) -> Self {
        Self {
            #[cfg(debug_assertions)]
            kind: SlotKind::Value,
            ptr: ptr as usize,
        }
    }

    pub fn frame(ptr: *mut ()) -> Self {
        Self {
            #[cfg(debug_assertions)]
            kind: SlotKind::Frame,
            ptr: ptr as usize,
        }
    }
}
