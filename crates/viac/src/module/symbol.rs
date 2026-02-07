/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt;

use bimap::BiMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SymbolId(u32);

impl fmt::Display for SymbolId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "@{}", self.0)
    }
}

#[derive(Default, Debug)]
pub struct SymbolTable {
    table: BiMap<String, SymbolId>,
    next_id: u32,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: BiMap::default(),
            next_id: 0,
        }
    }

    pub fn intern(&mut self, name: impl Into<String>) -> SymbolId {
        let name = name.into();
        if let Some(id) = self.table.get_by_left(&name) {
            return *id;
        }

        let id = SymbolId(self.next_id);
        self.next_id += 1;
        self.table.insert(name, id);
        id
    }
}
