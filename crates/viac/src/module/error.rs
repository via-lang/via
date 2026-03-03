/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::tree::ModulePath;
use crate::clinic::PrettyVec;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CompilationError,
    OsError(std::io::Error),
    ModuleNotFound {
        path: ModulePath,
    },
    AmbigiousModulePath {
        path: ModulePath,
        candidates: PrettyVec<String>,
    },
}
