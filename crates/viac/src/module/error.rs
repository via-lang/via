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

use super::{compiler::Ice, tree::ModulePath};
use crate::clinic::PrettyVec;

#[derive(Debug)]
pub enum CompilerError {
    Parse(crate::parser::error::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Diagnostic, Debug)]
pub enum Error {
    #[error("internal compiler error during compilation step {err}")]
    #[diagnostic(
        code(module::ice),
        help(
            "!!! THIS ERROR IS NOT SUPPOSED TO HAPPEN !!! report at https://github.com/via-lang/via"
        )
    )]
    IcError { err: Ice },

    #[error("os error: {err}")]
    #[diagnostic(code(module::os_error))]
    OsError { err: std::io::Error },

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
