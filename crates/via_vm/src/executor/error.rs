use crate::{instr::Instr, value::Value};

#[derive(Debug)]
pub struct ExecError {
    pub pc: *const Instr,
    pub err: Value,
}
