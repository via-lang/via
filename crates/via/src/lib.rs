mod source_module;

use std::path::Path;

pub use source_module::*;
use via_compiler::{
    db::{CompilerDb, SourceProgram},
    hir,
};

pub trait Module {}

pub trait ModuleLoader {
    type Error;

    fn load_module(&mut self, key: impl AsRef<str>) -> Result<Box<dyn Module>, Self::Error>;
}

pub fn __compile(path: &Path) {
    let db = CompilerDb::default();
    let program = SourceProgram::new(
        &db,
        path.file_stem()
            .expect("Expected file")
            .to_string_lossy()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect(),
        std::fs::read_to_string(path).expect("IO Error"),
    );

    hir::lower_program_to_hir(&db, program);
}
