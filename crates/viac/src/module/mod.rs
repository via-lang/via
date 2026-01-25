/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod binding;
pub mod context;
pub mod error;
pub mod symbol;
pub mod tree;

use binding::Binding;
use std::collections::HashMap;
use symbol::SymbolId;

#[derive(Debug)]
pub struct Module {
    bindings: HashMap<SymbolId, Binding>,
}
