/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::{
    ast::Tree,
    clinic::Clinic,
    hir::{self, builder::HirBuilder},
    lexer::{Lexer, token::Token},
    mir::{self, builder::MirBuilder},
    module::{error::Error, symbol::SymbolTable},
    parser::Parser,
    source::SourceBuf,
};

pub type Result<T> = std::result::Result<T, Error>;

pub mod state {
    use crate::lir;

    use super::*;

    #[derive(Debug)]
    pub struct Empty;

    #[derive(Debug)]
    pub struct Lexed {
        pub tt: Box<[Token]>,
    }

    #[derive(Debug)]
    pub struct Parsed {
        pub ast: Tree,
    }

    #[derive(Debug)]
    pub struct Hir {
        pub hir: hir::Hir,
    }

    #[derive(Debug)]
    pub struct Mir {
        pub mir: mir::Mir,
    }

    #[derive(Debug)]
    pub struct Lir {
        pub lir: lir::Lir,
    }

    #[derive(Debug)]
    pub struct Bytecode;
}

use state::*;

#[derive(Debug)]
struct Core<'cx> {
    source: &'cx SourceBuf,
}

#[derive(Debug)]
pub struct Compiler<'cx, S> {
    stage: S,
    core: Core<'cx>,
}

impl<'cx, S> Compiler<'cx, S> {
    pub fn source(&self) -> &SourceBuf {
        self.core.source
    }

    pub fn stage(&self) -> &S {
        &self.stage
    }

    fn with_state<O, F>(self, f: F) -> Compiler<'cx, O>
    where
        F: FnOnce(S, &Core) -> O,
    {
        let Compiler { stage, core } = self;
        Compiler {
            stage: f(stage, &core),
            core,
        }
    }
}

impl<'cx> Compiler<'cx, Empty> {
    pub fn new(source: &'cx SourceBuf) -> Self {
        Self {
            core: Core { source },
            stage: Empty,
        }
    }

    pub fn tokenize(self) -> Compiler<'cx, Lexed> {
        self.with_state(|_, core| Lexed {
            tt: Lexer::new(core.source).tokenize(),
        })
    }
}

impl<'cx> Compiler<'cx, Lexed> {
    pub fn parse(self, clinic: &mut Clinic) -> Option<Compiler<'cx, Parsed>> {
        let mut parser = Parser::new(&self.stage.tt);

        match parser.parse() {
            Ok(ast) => Some(self.with_state(|_, _| Parsed { ast })),
            Err(e) => {
                clinic.report(e);
                None
            }
        }
    }
}

impl<'cx> Compiler<'cx, Parsed> {
    pub fn lower(
        self,
        symbols: &mut SymbolTable,
        clinic: &mut Clinic,
    ) -> Option<Compiler<'cx, Hir>> {
        let mut hir_builder = HirBuilder::new(symbols, clinic, &self.stage.ast);

        match hir_builder.lower() {
            Some(hir) => Some(self.with_state(|_, _| Hir { hir })),
            None => None,
        }
    }
}

impl<'cx> Compiler<'cx, Hir> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn typecheck(self) -> Option<Self> {
        Some(self)
    }

    pub fn lower(
        self,
        symbols: &'cx mut SymbolTable,
        clinic: &'cx mut Clinic,
    ) -> Option<Compiler<'cx, Mir>> {
        let mut mir_builder = MirBuilder::new(symbols, clinic, &self.stage.hir);

        match mir_builder.lower() {
            Some(mir) => Some(self.with_state(|_, _| Mir { mir })),
            None => None,
        }
    }
}

impl<'cx> Compiler<'cx, Mir> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn lower(self) -> Option<Compiler<'cx, Lir>> {
        Some(Compiler {
            stage: Lir { lir: todo!() },
            core: self.core,
        })
    }
}

impl<'cx> Compiler<'cx, Lir> {
    pub fn lower(self) -> Compiler<'cx, Bytecode> {
        Compiler {
            stage: Bytecode {},
            core: self.core,
        }
    }
}

impl<'cx> Compiler<'cx, Bytecode> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn to_unit(self) -> CompilationUnit {
        CompilationUnit {}
    }
}

#[derive(Debug)]
pub struct CompilationUnit {}

pub fn compile(
    source: &SourceBuf,
    symbols: &mut SymbolTable,
    clinic: &mut Clinic,
) -> Option<CompilationUnit> {
    Some(
        Compiler::new(source)
            .tokenize()
            .parse(clinic)?
            .lower(symbols, clinic)?
            .typecheck()?
            .optimize()
            .lower(symbols, clinic)?
            .optimize()
            .lower()?
            .lower()
            .to_unit(),
    )
}
