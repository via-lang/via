use crate::{instruction::Instruction, value::Value};

#[derive(Debug)]
pub struct ExecError {
    pub pc: *const Instruction,
    pub err: Value,
}
