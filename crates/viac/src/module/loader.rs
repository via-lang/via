use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{
    Module, SourceModule,
    error::{Error, Result},
    tree::ModulePath,
};
use crate::{clinic::Clinic, source::SourceBuf};

pub trait ModuleLoader {
    fn load_module(
        &mut self,
        clinic: &mut Clinic,
        path: impl Into<ModulePath>,
    ) -> Result<Box<dyn Module>>;
}

#[derive(Debug)]
pub struct FsLoader {
    search_paths: Vec<PathBuf>,
}

impl FsLoader {
    pub fn new(root: &Path) -> Self {
        Self {
            search_paths: vec![root.to_path_buf()],
        }
    }

    pub fn add_search_path(&mut self, path: &Path) -> &mut Self {
        self.search_paths.push(path.to_path_buf());
        self
    }

    fn resolve(&mut self, path: ModulePath) -> Result<PathBuf> {
        let mut candidates = vec![];
        for mut dir in self.search_paths.clone() {
            path.0.iter().for_each(|node| dir.push(node));

            if let Ok(path) = fs::canonicalize(dir)
                && let Ok(meta) = fs::metadata(&path)
                && meta.is_file()
            {
                candidates.push(path);
            }
        }

        match candidates.as_slice() {
            [c] => Ok(c.clone()),
            [] => Err(Error::ModuleNotFound(path)),
            [_, _, ..] => Err(Error::AmbigiousModulePath(path)),
        }
    }
}

impl ModuleLoader for FsLoader {
    fn load_module(
        &mut self,
        clinic: &mut Clinic,
        path: impl Into<ModulePath>,
    ) -> Result<Box<dyn Module>> {
        let path = path.into();
        let fs_path = self.resolve(path.clone())?;

        let code = fs::read_to_string(&fs_path).map_err(Error::OsError)?;
        let name = format!("<{path} @ {}>", fs_path.to_string_lossy());

        let source = SourceBuf::new(name, code);

        SourceModule::new(source, clinic)
            .map(|m| -> Box<dyn Module> { Box::new(m) })
            .ok_or(Error::CompilationError)
    }
}
