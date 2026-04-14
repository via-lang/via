use std::{cell::RefCell, rc::Rc};

use bimap::BiHashMap;

use super::instr::{LocalId, TempId};
use crate::{
    counter::{Counter, SnapCounter},
    symbol::SymbolId,
};

#[derive(Debug)]
pub(super) struct Env<'a> {
    parent: Option<&'a Env<'a>>,
    local_id: Rc<RefCell<SnapCounter<LocalId>>>,
    pub temp_id: Counter<TempId>,
    map: BiHashMap<LocalId, SymbolId>,
}

impl Drop for Env<'_> {
    fn drop(&mut self) {
        self.local_id.borrow_mut().restore();
    }
}

impl<'a> Env<'a> {
    pub fn new(local_id: Rc<RefCell<SnapCounter<LocalId>>>, parent: Option<&'a Env>) -> Self {
        local_id.borrow_mut().save();
        Self {
            parent,
            local_id,
            temp_id: Counter::default(),
            map: BiHashMap::new(),
        }
    }

    pub fn lookup(&self, symbol: SymbolId) -> Option<LocalId> {
        self.map
            .get_by_right(&symbol)
            .copied()
            .or_else(|| self.parent.and_then(|parent| parent.lookup(symbol)))
    }

    pub fn lookup_symbol(&self, id: LocalId) -> SymbolId {
        self.map.get_by_left(&id).copied().unwrap_or_else(|| {
            self.parent
                .map(|parent| parent.lookup_symbol(id))
                .expect("LocalId not found")
        })
    }

    pub fn insert(&mut self, id: SymbolId) -> LocalId {
        let local = self.local_id.borrow_mut().bump();
        self.map.insert(local, id);
        local
    }
}
