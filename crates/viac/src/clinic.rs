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
use miette::{Diagnostic, Severity, SourceSpan};
use thiserror::Error;

use crate::{hir, parser, source::SourceBuf};

#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Parser(#[from] parser::error::Error),

    #[error(transparent)]
    #[diagnostic(transparent)]
    Hir(#[from] hir::error::Error),
}

#[derive(Debug)]
pub struct Clinic {
    healthy: bool,
    diags: Vec<Error>,
}

impl Clinic {
    pub fn new() -> Self {
        Self {
            healthy: true,
            diags: Vec::new(),
        }
    }

    pub fn healthy(&self) -> bool {
        self.healthy
    }

    pub fn report(&mut self, e: Error) {
        if let Some(Severity::Error) = dbg!(&e).severity() {
            self.healthy = false;
        }
        self.diags.push(e);
    }

    pub fn collect(&mut self) -> Vec<Error> {
        self.diags.drain(..).collect_vec()
    }

    pub fn emit(&mut self, src: &SourceBuf) {
        for diag in self.diags.drain(..) {
            let report = miette::Report::new(diag).with_source_code(src.clone());
            println!("{report:?}");
        }
    }
}

impl Default for Clinic {
    fn default() -> Self {
        Self::new()
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
