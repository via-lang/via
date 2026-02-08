/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::collections::{HashMap, hash_map::Entry};

use bitflags::bitflags;

use super::{
    Module,
    error::Result,
    tree::{ModuleId, ModulePath, ModuleTree},
};
use crate::{clinic::Clinic, module::loader::ModuleLoader};

bitflags! {
    #[derive(Debug)]
    pub struct ModulePerms: u8 {
        const None = 0;
    }
}

#[derive(Debug)]
pub struct ModuleContext {
    pub(super) tree: ModuleTree,
    pub(super) clinic: Clinic,
    pub(super) modules: HashMap<ModuleId, Box<dyn Module>>,
}

impl ModuleContext {
    pub fn new() -> Self {
        Self {
            tree: ModuleTree::new(),
            clinic: Clinic::new(),
            modules: HashMap::new(),
        }
    }

    pub fn get(&self, id: ModuleId) -> Option<&dyn Module> {
        self.modules.get(&id).map(Box::as_ref)
    }

    pub fn load(
        &mut self,
        loader: &mut impl ModuleLoader,
        path: impl Into<ModulePath>,
    ) -> Result<ModuleId> {
        let path = path.into();
        let id = self.tree.insert(&path);

        match self.modules.entry(id) {
            Entry::Occupied(_) => {}
            Entry::Vacant(e) => {
                let module = loader.load_module(&mut self.clinic, path)?;
                e.insert(module);
            }
        }
        Ok(id)
    }
}

impl Default for ModuleContext {
    fn default() -> Self {
        Self::new()
    }
}
