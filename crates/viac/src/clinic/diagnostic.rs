/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt::Debug;

use crate::{module::tree::ModuleId, source::SourceSpan};

#[derive(Debug)]
pub enum Header {
    Help,
    Hint,
    Note,
}

#[derive(Debug)]
pub struct Label {
    pub span: SourceSpan,
    pub message: String,
    pub header: Header,
}

#[derive(Debug)]
pub enum Severity {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct Report {
    pub origin: ModuleId,
    pub code: Option<&'static str>,
    pub message: String,
    pub severity: Severity,
    pub span: Option<SourceSpan>,
    pub labels: Vec<Label>,
}

pub trait Diagnostic: Debug {
    fn severity(&self) -> Severity;
    fn as_report(&self) -> Report;
}
