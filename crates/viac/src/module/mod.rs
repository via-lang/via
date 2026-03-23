pub mod compiler;
pub mod context;
pub mod error;
pub mod loader;
pub mod symbol;
pub mod tree;

use crate::{
    clinic::Clinic,
    sema::context::SemContext,
    source::{SourceBuf, SourceSpan},
};
use compiler::{CompilationUnit, Compiler};
use symbol::SymbolTable;

pub use context::*;
pub use loader::*;

pub trait Module {
    fn source(&self) -> Option<&SourceBuf> {
        None
    }

    fn get_trace(&self, span: SourceSpan) -> String;
}

#[allow(unused)]
pub struct SourceModule {
    source: SourceBuf,
    symbols: SymbolTable,
    sema: SemContext,
    unit: CompilationUnit,
}

impl Module for SourceModule {
    fn source(&self) -> Option<&SourceBuf> {
        Some(&self.source)
    }

    fn get_trace(&self, _span: SourceSpan) -> String {
        self.source.name().to_string()
    }
}

impl SourceModule {
    pub(crate) fn new(source: SourceBuf, clinic: &mut Clinic) -> Option<Self> {
        let mut symbols = SymbolTable::new();
        let mut sema = SemContext::new();

        let unit = Compiler::new()
            .tokenize(&source)
            .parse(clinic)?
            .lower(&mut symbols, clinic, &mut sema)?
            .typecheck()?
            .optimize()
            .lower(&mut symbols, clinic)?
            .optimize()
            .lower()?
            .to_unit();

        Some(Self {
            source,
            symbols,
            sema,
            unit,
        })
    }
}
