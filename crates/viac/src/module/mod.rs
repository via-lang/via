/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod compiler;
pub mod context;
pub mod error;
pub mod loader;
pub mod symbol;
pub mod tree;

use std::fmt::Debug;

use crate::{
    clinic::Clinic,
    source::{SourceBuf, SourceSpan},
};
use compiler::CompilationUnit;
use symbol::SymbolTable;

pub trait Module: Debug {
    fn source(&self) -> Option<&SourceBuf> {
        None
    }

    fn get_trace(&self, span: SourceSpan) -> String;
}

#[derive(Debug)]
pub struct SourceModule {
    #[allow(unused)]
    symbols: SymbolTable,
    source: SourceBuf,
    unit: CompilationUnit,
}

impl Module for SourceModule {
    fn source(&self) -> Option<&SourceBuf> {
        Some(&self.source)
    }

    fn get_trace(&self, _span: SourceSpan) -> String {
        format!("{}", self.source.name())
    }
}

impl SourceModule {
    pub(crate) fn new(src: &SourceBuf, clinic: &mut Clinic) -> Option<Self> {
        let mut symbols = SymbolTable::new();
        compiler::compile(src, &mut symbols, clinic).map(|unit| Self {
            symbols,
            source: src.clone(),
            unit,
        })
    }
}
