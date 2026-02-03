/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::{
    fmt::Debug,
    ops::{Add, AddAssign},
};

pub trait Id: Debug + Copy + PartialEq + Add<Output = Self> + AddAssign + From<usize> {}

#[derive(Debug)]
pub struct Counter<T: Id>(T);

impl<T: Id> Counter<T> {
    pub fn new(init: usize) -> Self {
        Self(T::from(init))
    }

    pub fn bump<const N: usize>(&mut self) -> [T; N] {
        let start = self.0;
        self.0 += T::from(N);
        std::array::from_fn(|i| start + T::from(i))
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
