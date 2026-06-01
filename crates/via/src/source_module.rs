use via_compiler::{
    Compiler,
    clinic::Clinic,
    def::DefContext,
    sema::SemContext,
    source::{SourceBuf, SourceSpan},
    symbol::StringInterner,
};
use via_macros::Access;
use via_vm::{Executable, Executor};

use super::{Access, Module};

#[allow(unused)]
#[derive(Access)]
pub struct SourceModule {
    #[getter]
    source: SourceBuf,
    #[getter]
    interner: StringInterner,
    #[getter]
    sem_ctxt: SemContext,
    #[getter]
    def_ctxt: DefContext,
    exe: Executable,
}

impl Module for SourceModule {
    fn trace(&self, span: SourceSpan) -> String {
        format!("<[{}..{}] @ {}>", span.begin, span.end, self.source.name())
    }
}

impl SourceModule {
    pub(crate) fn new(source: SourceBuf, clinic: &mut Clinic) -> Option<Self> {
        let mut interner = StringInterner::new();
        let mut sem_ctxt = SemContext::new();
        let mut def_ctxt = DefContext::new();

        let exe = Compiler::new()
            .inject_core(&mut interner, &mut sem_ctxt, &mut def_ctxt)?
            .tokenize(&source)
            .parse(clinic)?
            .lower(&mut interner, &mut sem_ctxt, &mut def_ctxt, clinic)?
            .typecheck()?
            .optimize()
            .lower(&mut interner, &mut sem_ctxt, &mut def_ctxt, clinic)?
            .optimize()
            .lower()?
            .0
            .exe;

        let mut exec = Executor::new(&exe, None);
        let int = exec.run();

        println!("{int:?}");
        println!("{exec:?}");

        Some(Self {
            source,
            interner,
            sem_ctxt,
            def_ctxt,
            exe,
        })
    }
}
