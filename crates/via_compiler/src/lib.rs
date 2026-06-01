pub mod ast;
pub mod clinic;
pub mod core;
mod counter;
pub mod def;
pub mod exe;
pub mod hir;
pub mod lexer;
mod macros;
pub mod mir;
pub mod node;
pub mod parser;
pub mod sema;
pub mod source;

use self::{
    ast::Tree,
    clinic::Clinic,
    def::DefContext,
    exe::ExeBuilder,
    hir::HirBuilder,
    lexer::{Lexer, Token},
    mir::MirBuilder,
    parser::Parser,
    sema::SemContext,
    source::SourceBuf,
};

pub mod symbol {
    use string_interner::{DefaultStringInterner, DefaultSymbol};

    pub type StringInterner = DefaultStringInterner;

    pub type Symbol = DefaultSymbol;

    pub trait IntoSymbol {
        fn into_symbol(self, it: &mut StringInterner) -> Symbol;
    }

    impl IntoSymbol for &str
    where
        Self: 'static,
    {
        fn into_symbol(self, it: &mut StringInterner) -> Symbol {
            it.get_or_intern_static(self)
        }
    }

    impl IntoSymbol for String {
        fn into_symbol(self, it: &mut StringInterner) -> Symbol {
            it.get_or_intern(&self)
        }
    }

    impl IntoSymbol for Symbol {
        fn into_symbol(self, _it: &mut StringInterner) -> Symbol {
            self
        }
    }
}

pub mod state {
    use via_vm::Executable;

    use super::*;

    pub struct Empty;

    pub struct Injected;

    pub struct Lexed {
        pub tt: Box<[Token]>,
    }

    pub struct Parsed {
        pub ast: Tree,
    }

    pub struct Hir {
        pub hir: hir::Hir,
    }

    pub struct Mir {
        pub mir: mir::Mir,
    }

    #[derive(Debug)]
    pub struct Exe {
        pub exe: Executable,
    }
}

use state::*;
use symbol::*;

#[must_use]
pub struct Compiler<S>(pub S);

impl Compiler<Empty> {
    pub fn new() -> Self {
        Self(Empty)
    }

    pub fn inject_core(
        self,
        interner: &mut StringInterner,
        sem_ctxt: &mut SemContext,
        def_ctxt: &mut DefContext,
    ) -> Option<Compiler<Injected>> {
        core::open(interner, sem_ctxt, def_ctxt).expect("::core injection failure");
        Some(Compiler(Injected))
    }
}

impl Default for Compiler<Empty> {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler<Injected> {
    pub fn tokenize(self, source: &SourceBuf) -> Compiler<Lexed> {
        Compiler(Lexed {
            tt: Lexer::new(source).tokenize(),
        })
    }
}

impl Compiler<Lexed> {
    pub fn parse(self, clinic: &mut Clinic) -> Option<Compiler<Parsed>> {
        let mut parser = Parser::new(&self.0.tt);

        match parser.parse() {
            Ok(ast) => Some(Compiler(Parsed { ast })),
            Err(e) => {
                clinic.report(e);
                None
            }
        }
    }
}

impl Compiler<Parsed> {
    pub fn lower(
        self,
        interner: &mut StringInterner,
        sema: &mut SemContext,
        def_ctxt: &mut DefContext,
        clinic: &mut Clinic,
    ) -> Option<Compiler<Hir>> {
        HirBuilder::new(clinic, interner, sema, def_ctxt, &self.0.ast)
            .lower()
            .map(|hir| Compiler(Hir { hir }))
    }
}

impl Compiler<Hir> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn typecheck(self) -> Option<Self> {
        Some(self)
    }

    pub fn lower(
        self,
        interner: &mut StringInterner,
        sem_ctxt: &mut SemContext,
        def_ctxt: &mut DefContext,
        clinic: &mut Clinic,
    ) -> Option<Compiler<Mir>> {
        MirBuilder::new(interner, sem_ctxt, def_ctxt, clinic, &self.0.hir)
            .lower()
            .map(|mir| Compiler(Mir { mir }))
    }
}

impl Compiler<Mir> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn lower(self) -> Option<Compiler<Exe>> {
        Some(Compiler(Exe {
            exe: ExeBuilder::new(&self.0.mir).build(),
        }))
    }
}
