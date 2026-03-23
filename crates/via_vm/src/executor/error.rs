use crate::{instr::Instr, value::Value};

#[derive(Debug)]
pub struct Error {
    pub pc: *const Instr,
    pub err: Value,
}
