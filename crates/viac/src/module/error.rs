use std::fmt;

use super::ModulePath;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    CompilationError,
    OsError(std::io::Error),
    ModuleNotFound(ModulePath),
    AmbigiousModulePath(ModulePath),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CompilationError => write!(f, "compilation error"),
            Self::OsError(e) => write!(f, "{e}"),
            Self::ModuleNotFound(path) => write!(f, "module not found: {path}"),
            Self::AmbigiousModulePath(path) => write!(f, "ambigious module path: {path}"),
        }
    }
}

impl std::error::Error for Error {}
