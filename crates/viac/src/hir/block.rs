/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{function::Function, instr::Instr, term::Term};

#[derive(Debug, PartialEq, Eq)]
pub struct BlockId(u32);

#[derive(Debug)]
pub struct Block {
    pub items: Vec<BlockItem>,
    pub term: Term,
}

#[derive(Debug)]
pub enum BlockItem {
    Instr(Instr),
    Block(Block),
    Function(Function),
}
