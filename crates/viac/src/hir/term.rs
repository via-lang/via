/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::fmt;

use super::{block::BlockId, instr::ValueId};

#[derive(Debug)]
pub enum Term {
    Halt,
    Break,
    Continue,
    Raise {
        value: ValueId,
    },
    Return {
        value: Option<ValueId>,
    },
    Jump {
        block: BlockId,
    },
    Branch {
        cond: ValueId,
        iftrue: BlockId,
        iffalse: BlockId,
    },
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Halt => writeln!(f, "halt"),
            _ => todo!(),
        }
    }
}
