/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::tree::ModulePath;
use crate::diags::builder::Builder;
use crate::diags::{Diagnostic, IntoDiagnostic, Note, Severity};
use crate::source::Source;
use std::path::Path;
use std::rc::Rc;
use via_proc_macros::DiagCode;

#[derive(DiagCode, Debug)]
#[diag(prefix = "E", start = 100)]
pub enum Error {
    ModuleNotFound {
        path: ModulePath,
    },
    AmbigiousModulePath {
        path: ModulePath,
        candidates: Vec<Rc<Path>>,
    },
}

impl IntoDiagnostic for Error {
    fn into_diagnostic(self, src: &Rc<Source>) -> Diagnostic {
        let mut b = Builder::new(src, Severity::Error);
        b.context("while importing module".to_string());

        match self {
            Self::ModuleNotFound { path } => b
                .message(format!("module not found '{}'", path))
                .location(path.span)
                .note(Note::Note("module path does not correspond to any module that is discoverable by the compiler".to_string()))
                .note(Note::Help("try adding its parent directory to the import search path using `--import=<path>`".to_string()))
                .note(Note::Help("or import it using a path relative to the importee module".to_string()))
                .build(),
            Self::AmbigiousModulePath { path, candidates } => {
                b.message(format!("ambigious module path '{}'", path))
                    .location(path.span)
                    .note(Note::Note(format!("found {} equally qualified candidates", candidates.len())));
                for (i, cand) in candidates.iter().enumerate() {
                    let msg = format!("#{i} => {}", cand.to_str().unwrap_or("<error>"));
                    b.note(Note::Note(msg));
                }
                b.note(Note::Help("try disambiguating between candidate modules".to_string())).build()
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
