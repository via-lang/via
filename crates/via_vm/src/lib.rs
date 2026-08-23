mod closure;
mod conversion;
mod executor;
mod heap;
mod instruction;
mod stack;
mod stats;
mod value;

pub use {closure::*, conversion::*, executor::*, heap::*, instruction::*, stats::*, value::*};

#[derive(Default, Debug)]
pub struct Executable {
    pub instrs: Vec<Instr>,
}

impl Executable {
    pub fn new() -> Self {
        Self::default()
    }
}
