/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::source::span::Span;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct ModulePath {
    pub span: Span,
    pub path: Rc<[String]>,
}

impl From<&ModulePath> for PathBuf {
    fn from(value: &ModulePath) -> Self {
        let mut path = PathBuf::new();
        value.path.iter().for_each(|node| path.push(node));
        path
    }
}

impl fmt::Display for ModulePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.iter().join("."))
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct ModuleId(u32);

impl From<u32> for ModuleId {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ModuleTree {
    root: ModuleNode,
    next_id: u32,
}

impl ModuleTree {
    pub fn new() -> Self {
        Self {
            root: ModuleNode::new(0.into()),
            next_id: 1,
        }
    }

    pub fn insert(&mut self, path: &ModulePath) -> ModuleId {
        let mut current = &mut self.root;
        for segment in path.path.iter() {
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
        for segment in path.path.iter() {
            if let Some(child) = current.children.get(segment) {
                current = child;
            } else {
                return None;
            }
        }
        Some(current.id)
    }
}
