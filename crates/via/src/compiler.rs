use via_compiler::{
    ast::Tree,
    clinic::Clinic,
    def::{DefContext, traits},
    exe::ExeBuilder,
    hir::{self, HirBuilder},
    lexer::{Lexer, Token},
    mir::{self, MirBuilder},
    parser::Parser,
    sema::SemContext,
    source::SourceBuf,
    symbol::SymbolTable,
};

pub mod state {
    use via_vm::Executable;

    use super::*;

    pub struct Empty;

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

pub struct Compiler<S>(pub S);

impl Compiler<Empty> {
    pub fn new() -> Self {
        Self(Empty)
    }

    pub fn tokenize(self, source: &SourceBuf) -> Compiler<Lexed> {
        Compiler(Lexed {
            tt: Lexer::new(source).tokenize(),
        })
    }
}

impl Default for Compiler<Empty> {
    fn default() -> Self {
        Self::new()
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
        st: &mut SymbolTable,
        sema: &mut SemContext,
        def: &mut DefContext,
        clinic: &mut Clinic,
    ) -> Option<Compiler<Hir>> {
        traits::register_builtin(st, sema, def)
            .inspect_err(|e| clinic.report(*e))
            .ok()?;

        HirBuilder::new(clinic, st, sema, def, &self.0.ast)
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
        st: &mut SymbolTable,
        sem: &mut SemContext,
        def: &mut DefContext,
        clinic: &mut Clinic,
    ) -> Option<Compiler<Mir>> {
        MirBuilder::new(st, sem, def, clinic, &self.0.hir)
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
