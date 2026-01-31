/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::{
    collections::{HashMap, hash_map::Entry},
    fs,
    path::{Path, PathBuf},
};

use bitflags::bitflags;

use super::{
    Module,
    error::{Error, Result},
    tree::{ModuleId, ModulePath, ModuleTree},
};
use crate::{clinic::PrettyVec, source::SourceBuf};

bitflags! {
    #[derive(Debug)]
    pub struct ModulePerms: u8 {
        const None = 0;
    }
}

#[derive(Debug, Default)]
pub struct ModuleContext {
    pub(super) tree: ModuleTree,
    pub(super) modules: HashMap<ModuleId, Module>,
    pub(super) paths: Vec<PathBuf>,
}

impl ModuleContext {
    pub fn new(root: &Path) -> Self {
        Self {
            tree: ModuleTree::new(),
            modules: HashMap::new(),
            paths: vec![root.to_path_buf()],
        }
    }

    pub fn get(&self, id: ModuleId) -> Option<&Module> {
        self.modules.get(&id)
    }

    pub fn load(&mut self, fs_path: &Path, path: impl Into<ModulePath>) -> Result<ModuleId> {
        let path = path.into();
        let id = self.tree.insert(&path);

        match self.modules.entry(id) {
            Entry::Occupied(_) => {}
            Entry::Vacant(e) => {
                let code = fs::read_to_string(fs_path).map_err(Error::OsError)?;
                let source = SourceBuf::new(
                    format!("<module:{path} @ {}>", fs_path.to_string_lossy()),
                    code,
                );

                let module = Module::new(&source)?;
                e.insert(module);
            }
        }

        Ok(id)
    }

    pub fn resolve(&mut self, path: ModulePath) -> Result<ModuleId> {
        if let Some(id) = self.tree.get(&path) {
            return Ok(id);
        }

        let mut candidates = vec![];
        for mut dir in self.paths.clone() {
            path.0.iter().for_each(|node| dir.push(node));

            if let Ok(path) = fs::canonicalize(dir)
                && let Ok(meta) = fs::metadata(&path)
                && meta.is_file()
            {
                candidates.push(path);
            }
        }

        match &candidates.as_slice() {
            [] => Err(Error::ModuleNotFound { path }),
            [c] => self.load(c.as_path(), path),
            [_, _, ..] => Err(Error::AmbigiousModulePath {
                path,
                candidates: PrettyVec::from(
                    candidates
                        .iter()
                        .map(|c| c.to_string_lossy().to_string())
                        .collect::<Vec<String>>(),
                ),
            }),
        }
    }
}
