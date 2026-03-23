use std::{collections::HashMap, fmt};

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
