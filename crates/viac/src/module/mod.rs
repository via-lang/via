/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod binding;
pub mod compiler;
pub mod context;
pub mod error;
pub mod symbol;
pub mod tree;

use std::fmt::Debug;

use crate::{clinic::Clinic, source::SourceBuf};
use binding::Binding;
use compiler::CompilationUnit;
use error::{Error, Result};
use symbol::SymbolId;
use symbol::SymbolTable;

pub trait Module: Debug {
    fn get_symbol(&self, sym: SymbolId) -> Option<&Binding>;
}

#[derive(Debug)]
pub struct SourceModule {
    source: SourceBuf,
    symbols: SymbolTable,
    unit: CompilationUnit,
}

impl Module for SourceModule {
    fn get_symbol(&self, symbol: SymbolId) -> Option<&Binding> {
        self.unit.bindings.get(&symbol)
    }
}

impl SourceModule {
    pub(crate) fn new(src: &SourceBuf, clinic: &mut Clinic) -> Result<Self> {
        let symbols = SymbolTable::new();

        match compiler::compile(src, clinic) {
            Ok(unit) => Ok(Self {
                source: src.clone(),
                symbols,
                unit,
            }),
            Err(ice) => Err(Error::IcError { err: ice }),
        }
    }

    pub fn source(&self) -> &SourceBuf {
        &self.source
    }
}
