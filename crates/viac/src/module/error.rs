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
use crate::diags::{Diag, DiagKind, IntoDiag};
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

impl IntoDiag for Error {
    fn into_diag(self, src: &Rc<Source>) -> Diag {
        let mut builder = Builder::new(&src, DiagKind::Error);
        builder.context("while importing module".to_string());

        match self {
            Self::ModuleNotFound { path } => builder
                .message(format!("module not found '{}'", path))
                .location(path.span)
                .build(),
            Self::AmbigiousModulePath {
                path,
                candidates: _,
            } => builder
                .message(format!("ambigious module path '{}'", path))
                .location(path.span)
                .build(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
