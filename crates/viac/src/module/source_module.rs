use via_macros::Access;

use super::{Compiler, Executable, Module, SymbolTable, def::DefContext};
use crate::{
    clinic::Clinic,
    macros::ice_panic,
    sema::SemContext,
    source::{SourceBuf, SourceSpan},
    traits::Access,
};

#[allow(unused)]
#[derive(Access)]
pub struct SourceModule {
    #[getter]
    source: SourceBuf,
    #[getter]
    st: SymbolTable,
    #[getter]
    sem: SemContext,
    #[getter]
    def: DefContext,
    exe: Executable,
}

impl Module for SourceModule {
    fn trace(&self, span: SourceSpan) -> String {
        format!("<[{}..{}] @ {}>", span.begin, span.end, self.source.name())
    }
}

impl SourceModule {
    pub(crate) fn new(source: SourceBuf, clinic: &mut Clinic) -> Option<Self> {
        let mut st = SymbolTable::new();
        let mut sem = SemContext::new();
        let mut def = DefContext::new();

        let exe = Compiler::new()
            .tokenize(&source)
            .parse(clinic)?
            .lower(&mut st, &mut sem, &mut def, clinic)?
            .typecheck()?
            .optimize()
            .lower(&mut st, &mut sem, &mut def, clinic)?
            .optimize()
            .lower()?
            .to_executable();

        // dbg!(&st);
        // dbg!(&sem);
        // dbg!(&def);

        Some(Self {
            source,
            st,
            sem,
            def,
            exe,
        })
    }
}
