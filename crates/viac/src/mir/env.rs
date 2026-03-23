use bimap::BiHashMap;

use super::instr::{LocalId, TempId};
use crate::{
    counter::{Counter, Id},
    mir::block::BlockId,
    module::symbol::SymbolId,
};

#[derive(Debug)]
pub(super) struct LoopEnv {
    pub control: BlockId,
    pub exit: BlockId,
}

#[derive(Debug)]
pub(super) struct Env {
    map: BiHashMap<LocalId, SymbolId>,
    counter: Counter<LocalId>,
    pub temp_id: Counter<TempId>,
    pub loop_env: Option<LoopEnv>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            map: BiHashMap::new(),
            counter: Counter::default(),
            temp_id: Counter::default(),
            loop_env: None,
        }
    }

    #[allow(unused)]
    pub fn get(&self, id: LocalId) -> SymbolId {
        *self
            .map
            .get_by_left(&id)
            .expect("HIR Env queries must be always valid")
    }

    pub fn lookup(&self, symbol: SymbolId) -> Option<LocalId> {
        self.map.get_by_right(&symbol).copied()
    }

    pub fn push(&mut self, id: SymbolId) -> LocalId {
        let local = self.counter.bump();
        self.map.insert(local, id);
        local
    }
}
