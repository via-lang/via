use std::{cell::RefCell, rc::Rc};

use bimap::BiHashMap;

use super::instr::{LocalId, TempId};
use crate::{
    counter::{Counter, SnapCounter},
    symbol::Symbol,
};

#[derive(Debug)]
pub(super) struct Env<'a> {
    #[allow(unused)]
    parent: Option<&'a Env<'a>>,
    local_id: Rc<RefCell<SnapCounter<LocalId>>>,
    pub temp_id: Counter<TempId>,
    map: BiHashMap<LocalId, Symbol>,
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

    #[allow(unused)]
    pub fn lookup(&self, symbol: Symbol) -> Option<LocalId> {
        self.map
            .get_by_right(&symbol)
            .copied()
            .or_else(|| self.parent.and_then(|parent| parent.lookup(symbol)))
    }

    #[allow(unused)]
    pub fn lookup_symbol(&self, id: LocalId) -> Symbol {
        self.map.get_by_left(&id).copied().unwrap_or_else(|| {
            self.parent
                .map(|parent| parent.lookup_symbol(id))
                .expect("LocalId not found")
        })
    }

    pub fn insert(&mut self, id: Symbol) -> LocalId {
        let local = self.local_id.borrow_mut().bump();
        self.map.insert(local, id);
        local
    }
}
