/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::error::{Error, Result};
use super::tree::{ModuleId, ModulePath, ModuleTree};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

pub struct LookupContext {
    pub file: PathBuf,
    pub lib: PathBuf,
}

pub struct ModuleContext {
    tree: ModuleTree,
    index: HashMap<ModuleId, PathBuf>,
}

impl ModuleContext {
    pub fn resolve(&mut self, path: ModulePath, ctxt: LookupContext) -> Result<ModuleId> {
        if let Some(id) = self.tree.get(&path) {
            return Ok(id);
        }

        let mut candidates = Vec::<Rc<Path>>::new();
        let dirs = [
            ctxt.lib.to_path_buf(),
            ctxt.file
                .parent()
                .expect("working file context must have parent node during module resolution")
                .to_path_buf(),
            std::env::current_dir()
                .expect("compiler working directory must be present during module resolution"),
        ];

        for mut dir in dirs {
            dir.push(PathBuf::from(&path));
            let Ok(path) = fs::canonicalize(dir) else {
                continue;
            };
            let Ok(meta) = fs::metadata(&path) else {
                continue;
            };

            if meta.is_file() && !meta.is_symlink() {
                candidates.push(Rc::from(path.into_boxed_path()));
            }
        }

        match &candidates.as_slice() {
            [c] => {
                let id = self.tree.insert(&path);
                self.index.insert(id, c.to_path_buf());
                Ok(id)
            }
            [_, _, ..] => Err(Error::AmbigiousModulePath { path, candidates }),
            _ => Err(Error::ModuleNotFound { path }),
        }
    }
}
