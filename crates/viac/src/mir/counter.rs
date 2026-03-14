/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

pub trait Id: Display + Copy + PartialEq + Add<Output = Self> + AddAssign + From<u32> {}

#[derive(Debug)]
pub struct Counter<T: Id>(T);

impl<T: Id> Counter<T> {
    pub fn new(init: u32) -> Self {
        Self(T::from(init))
    }

    pub fn bump<const N: usize>(&mut self) -> [T; N] {
        let start = self.0;
        self.0 += T::from(N as u32);
        std::array::from_fn(|i| start + T::from(i as u32))
    }

    pub fn reset(&mut self) -> T {
        let value = self.0;
        self.0 = T::from(0);
        value
    }

    pub fn restore(&mut self, n: T) {
        self.0 = n;
    }
}

impl<T: Id> Default for Counter<T> {
    fn default() -> Self {
        Self::new(0)
    }
}
