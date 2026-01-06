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
        $crate::compiler::source::Span {
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

#[derive(Debug)]
pub struct Source(pub String);

impl Source {
    pub fn slice(&self, span: Span) -> &str {
        let begin = span.begin as usize;
        let end = span.end as usize;

        &self.0[begin..end]
    }

    pub fn span_of(&self, sub: &str) -> Span {
        let begin_start = self.0.as_ptr() as usize;
        let sub_start = sub.as_ptr() as usize;

        assert!(
            sub_start >= begin_start && sub_start + sub.len() <= begin_start + self.0.len(),
            "substring does not belong to source"
        );

        let begin = sub_start - begin_start;
        let end = begin + sub.len();

        span![begin as u32, end as u32]
    }

    pub fn span_of_range(&self, begin: &str, end: &str) -> Span {
        span![self.span_of(begin).begin, self.span_of(end).end]
    }
}
