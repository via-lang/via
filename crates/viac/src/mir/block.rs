use std::fmt;

use super::{instr::Instr, term::Term};
use derive_more::From;

#[derive(From, Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockId(u32);

impl BlockId {
    pub(super) fn inner(self) -> u32 {
        self.0
    }
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Debug)]
pub struct Block {
    pub id: BlockId,
    pub instrs: Vec<Instr>,
    pub term: Term,
}

impl Block {
    pub fn new(id: BlockId) -> Self {
        Self {
            id,
            instrs: vec![],
            term: Term::Halt,
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}:", self.id)?;
        for instr in &self.instrs {
            write!(f, "  {instr}")?;
        }
        write!(f, "  {}", self.term)?;
        Ok(())
    }
}
