/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::ty::Ty;
use crate::{module::symbol::SymbolId, node::NodeId};

#[derive(Debug)]
pub struct FuncSig {
    pub sym: SymbolId,
    // TODO: Represent optional self parameter
    pub parms: Vec<NodeId<Ty>>,
    pub ret: NodeId<Ty>,
}

#[derive(Debug)]
pub enum Intrinsic {
    Bytecode(fn()),
}

#[derive(Debug)]
pub enum FuncImpl {
    Intr(Intrinsic),
}
