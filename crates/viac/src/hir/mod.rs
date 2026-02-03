/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod block;
pub mod builder;
pub mod counter;
pub mod error;
pub mod expr;
pub mod instr;
pub mod stmt;
pub mod term;

use std::fmt;

use crate::{
    clinic::Clinic,
    module::compiler::{Compiler, state::Parsed},
};

use block::{Block, BlockId};
use builder::IrBuilder;
use counter::Counter;
use instr::{LocalId, TempId};

#[derive(Debug, Default)]
pub struct Hir {
    blocks: Vec<Block>,
    temp_id: Counter<TempId>,
    local_id: Counter<LocalId>,
}

impl Hir {
    pub fn get(&self, id: BlockId) -> &Block {
        self.blocks
            .get(id.inner() as usize)
            .expect("BlockIds must be always valid")
    }

    pub fn get_mut(&mut self, id: BlockId) -> &mut Block {
        self.blocks
            .get_mut(id.inner() as usize)
            .expect("BlockIds must be always valid")
    }
}

impl fmt::Display for Hir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in &self.blocks {
            write!(f, "{block}")?;
        }
        Ok(())
    }
}

pub(crate) fn lower(c: &Compiler<Parsed>, clinic: &mut Clinic) -> Hir {
    IrBuilder::new(c.source(), &c.stage().ast, clinic).lower()
}
