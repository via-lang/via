/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt;

macro_rules! span {
    ($begin:expr, $end:expr) => {
        $crate::source::span::Span {
            begin: $begin,
            end: $end,
        }
    };
}

pub(crate) use span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub begin: u32,
    pub end: u32,
}

impl Span {
    pub fn length(&self) -> usize {
        (self.end - self.begin) as usize
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{} -> {}]", self.begin, self.end)
    }
}
