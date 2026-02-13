/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use bimap::BiHashMap;

use super::{
    counter::Counter,
    instr::{LocalId, TempId},
};
use crate::{mir::block::BlockId, module::symbol::SymbolId};

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
        let local = *self.counter.bump::<1>().first().unwrap();
        self.map.insert(local, id);
        local
    }

    pub fn set_loop_env(&mut self, loop_env: Option<LoopEnv>) {
        self.loop_env = loop_env;
    }
}
