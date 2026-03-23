pub mod block;
pub mod builder;
pub mod env;
pub mod error;
pub mod instr;
pub mod term;

use std::fmt;

use block::{Block, BlockId};

#[derive(Debug, Default)]
pub struct Mir {
    blocks: Vec<Block>,
}

impl Mir {
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

impl fmt::Display for Mir {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for block in &self.blocks {
            write!(f, "{block}")?;
        }
        Ok(())
    }
}
