/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt;

use itertools::Itertools;
use miette::SourceSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StageControl {
    Ok,
    Terminate,
}

#[derive(Debug)]
pub struct Diagnostic {
    pub report: miette::Report,
    pub control: StageControl,
}

#[derive(Default, Debug)]
pub struct Clinic {
    diags: Vec<Diagnostic>,
}

impl Clinic {
    pub fn report(&mut self, d: Diagnostic) {
        self.diags.push(d);
    }

    pub fn emit(&mut self) -> bool {
        for diag in self.diags.iter() {
            println!("{:?}", diag.report);
            match diag.control {
                StageControl::Ok => {}
                StageControl::Terminate => return true,
            }
        }
        self.diags.clear();
        false
    }
}

#[derive(Debug)]
pub struct PrettyVec<T: fmt::Display>(pub Vec<T>);

impl<T: fmt::Display> From<Vec<T>> for PrettyVec<T> {
    fn from(value: Vec<T>) -> Self {
        PrettyVec(value)
    }
}

impl<T: fmt::Display> fmt::Display for PrettyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.0[..] {
                [] => String::new(),
                [a] => a.to_string(),
                [a, b] => format!("{a} or {b}"),
                _ => {
                    let (head, last) = self.0.split_at(self.0.len() - 1);
                    format!("{} or {}", head.iter().join(", "), last[0])
                }
            }
        )
    }
}

pub trait SourceSpanTupleExt {
    fn into_span(self) -> SourceSpan;
}

impl SourceSpanTupleExt for (SourceSpan, SourceSpan) {
    fn into_span(self) -> SourceSpan {
        let (a, b) = self;
        let start = a.offset();
        let end = b.offset() + b.len();
        SourceSpan::new(start.into(), end - start)
    }
}
