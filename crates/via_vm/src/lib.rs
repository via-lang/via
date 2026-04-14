mod arena;
pub mod executor;
pub mod instr;
pub mod stack;
pub mod value;

use instr::Instr;

#[derive(Default, Debug)]
pub struct Executable {
    pub instrs: Vec<Instr>,
}

impl Executable {
    pub fn new() -> Self {
        Self::default()
    }
}
