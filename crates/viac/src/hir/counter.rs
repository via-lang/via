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

pub trait Id: Copy + Add<Output = Self> + AddAssign + From<usize> {}

#[derive(Debug)]
pub struct Counter<T: Id>(T);

impl<T: Id> Counter<T> {
    pub fn next<const N: usize>(&mut self) -> [T; N] {
        let start = self.0;
        self.0 += T::from(N);
        std::array::from_fn(|i| start + T::from(i))
    }
}
