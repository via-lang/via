/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    counter::Counter,
    instr::{LocalId, ValueId},
};

#[derive(Debug)]
pub struct Env {
    pub value_id: Counter<ValueId>,
    pub local_id: Counter<LocalId>,
}
