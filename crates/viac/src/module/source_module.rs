use via_macros::Access;
use via_vm::{Executable, executor::Executor};

use super::{Compiler, Module, SymbolTable, def::DefContext};
use crate::{
    clinic::Clinic,
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
            .0
            .exe;

        let int = Executor::new(&exe, None).run();
        dbg!(&int);

        Some(Self {
            source,
            st,
            sem,
            def,
            exe,
        })
    }
}
