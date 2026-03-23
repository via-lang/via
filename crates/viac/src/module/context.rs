use std::{
    collections::{HashMap, hash_map::Entry},
    fs,
    path::Path,
};

use bitflags::bitflags;

use super::{
    Module, SourceModule,
    error::{Error, Result},
    tree::{ModuleId, ModulePath, ModuleTree},
};
use crate::{clinic::Clinic, module::loader::ModuleLoader, source::SourceBuf};

pub const ROOT_MODULE_NAME: &str = "main";

bitflags! {
    pub struct ModulePerms: u8 {
        const None = 0;
    }
}

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

    pub fn load_module(
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

    pub fn load_file(&mut self, path: &Path) -> Result<ModuleId> {
        let code = fs::read_to_string(path).map_err(Error::OsError)?;
        let name = format!("<main @ {}>", path.to_string_lossy());

        let source = SourceBuf::new(name, code);
        let module = SourceModule::new(source, &mut self.clinic);

        self.clinic.emit();

        let id = self.tree.insert(&ROOT_MODULE_NAME.into());
        let module = module
            .map(|m| -> Box<dyn Module> { Box::new(m) })
            .ok_or(Error::CompilationError)?;

        self.modules.insert(id, module);

        Ok(id)
    }
}

impl Default for ModuleContext {
    fn default() -> Self {
        Self::new()
    }
}
