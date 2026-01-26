/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod builder;
pub mod context;
pub mod renderer;

use crate::source::{Source, span::Span};
use std::rc::Rc;

pub trait IntoDiagnostic {
    fn into_diagnostic(self, src: &Rc<Source>) -> Diagnostic;
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Option<&'static str>,
    pub message: String,
    pub location: Option<Span>,
    pub context: Vec<String>,
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Severity {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub enum Note {
    Note(String),
    Help(String),
}
