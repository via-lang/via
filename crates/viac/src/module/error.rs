/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use miette::Diagnostic;
use thiserror::Error;

use super::tree::ModulePath;
use crate::clinic::PrettyVec;

#[derive(Debug)]
pub enum CompilerError {
    Parse(crate::parser::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("compilation error")]
    CompilationError,

    #[error("os error")]
    #[label("os error: {err}")]
    #[diagnostic(code(module::os_error))]
    OsError(std::io::Error),

    #[error("module not found")]
    #[label("'{path}' does not correspond to any module within search parameters")]
    #[diagnostic(code(module::not_found))]
    ModuleNotFound { path: ModulePath },

    #[error("module path is ambigious")]
    #[label("'{path}' is ambigious between {candidates}")]
    #[diagnostic(code(module::ambig))]
    AmbigiousModulePath {
        path: ModulePath,
        candidates: PrettyVec<String>,
    },
}
