/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

// Stupid miette proc macro magic producing false warnings
#![allow(unused_assignments)]

use miette::Diagnostic;
use thiserror::Error;

use super::tree::ModulePath;
use crate::clinic::PrettyVec;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("compilation error")]
    #[diagnostic()]
    CompilationError,

    #[error("os error: {0}")]
    #[diagnostic(code(module::os_error))]
    OsError(#[from] std::io::Error),

    #[error("'{path}' does not correspond to any module within search parameters")]
    #[diagnostic(code(module::not_found))]
    ModuleNotFound { path: ModulePath },

    #[error("'{path}' is ambigious between {candidates}")]
    #[diagnostic(code(module::ambig))]
    AmbigiousModulePath {
        path: ModulePath,
        candidates: PrettyVec<String>,
    },
}
