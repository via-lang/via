/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt::Debug;

pub enum Severity {
    Info,
    Warning,
    Error,
}

pub trait Diagnostic: Debug {
    fn severity(&self) -> Severity;
}

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

    pub fn emit(&mut self) {
        for diag in self.diags.drain(..) {
            println!("{diag:?}")
        }
    }
}
