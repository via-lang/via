mod closure;
mod executor;
mod heap;
mod instr;
mod stack;
mod traits;
mod value;

pub use {closure::*, executor::*, heap::*, instr::*, value::*};

#[derive(Default, Debug)]
pub struct Executable {
    pub instrs: Vec<Instr>,
}

impl Executable {
    pub fn new() -> Self {
        Self::default()
    }
}
