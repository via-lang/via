mod compiler;
mod def;
pub mod error;
mod loader;
mod source_module;
mod symbol;

use std::fmt;

use bitflags::bitflags;

use crate::{
    clinic::Clinic,
    source::{SourceBuf, SourceSpan},
    traits::Access,
};

use error::*;

pub use {compiler::*, def::*, loader::*, source_module::*, symbol::*};

pub trait Module: Access<DefContext> {
    fn trace(&self, span: SourceSpan) -> String;
}

use std::{
    collections::{HashMap, hash_map::Entry},
    fs,
    path::Path,
};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModulePath(pub Box<[String]>);

impl ModulePath {
    pub fn new<I, S>(iter: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self(
            iter.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }
}

impl<S> From<S> for ModulePath
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self::new(vec![value.into()])
    }
}

impl fmt::Display for ModulePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.join(".").fmt(f)
    }
}

#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct ModuleId(u32);

impl From<u32> for ModuleId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Default, Debug)]
struct ModuleNode {
    id: ModuleId,
    children: HashMap<String, ModuleNode>,
}

impl ModuleNode {
    fn new(id: ModuleId) -> Self {
        Self {
            id,
            children: HashMap::new(),
        }
    }
}

#[derive(Default, Debug)]
pub struct ModuleTree {
    root: ModuleNode,
    next_id: u32,
}

impl ModuleTree {
    pub fn new() -> Self {
        Self {
            root: ModuleNode::new(0.into()),
            next_id: 0,
        }
    }

    pub fn insert(&mut self, path: &ModulePath) -> ModuleId {
        let mut current = &mut self.root;
        for segment in path.0.iter() {
            current = current.children.entry(segment.clone()).or_insert_with(|| {
                let id = self.next_id.into();
                self.next_id += 1;
                ModuleNode::new(id)
            });
        }
        current.id
    }

    pub fn get(&self, path: &ModulePath) -> Option<ModuleId> {
        let mut current = &self.root;
        for segment in path.0.iter() {
            let _ = current
                .children
                .get(segment)
                .inspect(|child| current = child)?;
        }
        Some(current.id)
    }
}
