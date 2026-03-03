/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod slot;

use std::{array, mem::MaybeUninit};

use crate::value::Value;

use slot::Slot;

pub struct Stack<const N: usize> {
    data: Box<[MaybeUninit<Slot>; N]>,
    sp: usize,
}

impl<const N: usize> Stack<N> {
    pub fn new() -> Self {
        Self {
            data: Box::new(array::from_fn(|_| MaybeUninit::uninit())),
            sp: 0,
        }
    }

    pub fn push(&mut self, value: Slot) -> *mut Slot {
        debug_assert!(self.sp < N, "stack overflow");
        let mut data = self.data[self.sp];
        self.sp += 1;
        data.write(value);
        data.as_mut_ptr()
    }

    pub fn pop(&mut self) -> Slot {
        debug_assert_ne!(self.sp, 0, "stack underflow");
        self.sp -= 1;
        unsafe { self.data[self.sp].assume_init_read() }
    }
}
