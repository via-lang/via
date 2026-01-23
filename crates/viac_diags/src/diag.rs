/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use termcolor::Color;
use viac_source::span::Span;

#[derive(Debug, Clone)]
pub struct Diag {
    pub kind: Option<DiagKind>,
    pub message: String,
    pub location: Option<Span>,
    pub context: Vec<String>,
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DiagKind {
    Info,
    Warn,
    Error,
}

#[derive(Debug, Clone)]
pub enum Note {
    Note(String),
    Help(String),
}

#[derive(Debug)]
pub(crate) struct HeaderInfo(pub Color, pub &'static str);

impl From<DiagKind> for HeaderInfo {
    fn from(value: DiagKind) -> Self {
        match value {
            DiagKind::Info => Self(Color::Cyan, "info:"),
            DiagKind::Warn => Self(Color::Yellow, "warning:"),
            DiagKind::Error => Self(Color::Red, "error:"),
        }
    }
}
