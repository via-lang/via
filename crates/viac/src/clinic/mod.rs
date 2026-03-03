/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

mod diagnostic;
mod renderer;
mod term;

use std::fmt;

use itertools::Itertools;

pub use diagnostic::{Diagnostic, Report, Severity};
pub use renderer::Renderer;

#[derive(Debug)]
pub struct Clinic {
    healthy: bool,
    diags: Vec<Box<dyn Diagnostic>>,
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

    pub fn report(&mut self, diag: impl Diagnostic + 'static) {
        self.healthy = !matches!(diag.severity(), Severity::Error);
        self.diags.push(Box::new(diag));
    }

    pub fn collect(&mut self) -> Vec<Box<dyn Diagnostic>> {
        self.diags.drain(..).collect_vec()
    }

    pub fn emit(&mut self, renderer: &mut impl Renderer) {
        for diag in self.diags.drain(..) {
            let report = diag.as_report();
            renderer.render(report);
        }
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
        match self.0.as_slice() {
            [] => write!(f, ""),
            [a] => write!(f, "{a}"),
            [a, b] => write!(f, "{a} or {b}"),
            _ => {
                let (head, last) = self.0.split_at(self.0.len() - 1);
                let left = head.iter().join(", ");
                let right = &last[0];
                write!(f, "{left} or {right}")
            }
        }
    }
}
