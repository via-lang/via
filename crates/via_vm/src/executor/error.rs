/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::{instr::Instr, value::Value};

#[derive(Debug)]
pub struct Error {
    pub pc: *const Instr,
    pub err: Value,
}
