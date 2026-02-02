/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{block::BlockId, instr::ValueId};

#[derive(Debug)]
pub enum Term {
    Break,
    Continue,
    Return {
        value: Option<ValueId>,
    },
    Raise {
        value: ValueId,
    },
    Branch {
        block: BlockId,
    },
    CondBranch {
        cond: ValueId,
        iftrue: BlockId,
        iffalse: BlockId,
    },
}
