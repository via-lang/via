/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::{
    hir::{block::Block, env::Env},
    module::symbol::SymbolId,
    sema::ty::Ty,
};

#[derive(Debug)]
pub struct Function {
    symbol: SymbolId,
    result: Ty,
    params: Vec<Ty>,
    entry: Block,
    blocks: Vec<Block>,
    env: Env,
}
