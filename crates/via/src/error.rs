use std::fmt;

use super::ModulePath;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CompilationError,
    OsError(std::io::Error),
    UnrecognizedExtension,
    ModuleNotFound(ModulePath),
    AmbigiousModulePath(ModulePath),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
