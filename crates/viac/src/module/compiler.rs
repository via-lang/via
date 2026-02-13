/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::collections::HashMap;

use crate::{
    ast::Tree,
    clinic::{Clinic, Error as CompilationError},
    hir,
    lexer::{Lexer, token::Token},
    mir,
    module::{
        binding::Binding,
        error::Error,
        symbol::{SymbolId, SymbolTable},
    },
    parser,
    source::SourceBuf,
};

pub type Result<T> = std::result::Result<T, Error>;

pub mod state {
    use super::*;

    #[derive(Debug)]
    pub struct Empty;

    #[derive(Debug)]
    pub struct Lexed {
        pub tt: Box<[Token]>,
    }

    #[derive(Debug)]
    pub struct Parsed {
        pub tt: Box<[Token]>,
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
    pub struct Bytecode;
}

use state::*;

#[derive(Debug)]
struct Core<'a> {
    source: &'a SourceBuf,
}

#[derive(Debug)]
pub struct Compiler<'a, S> {
    stage: S,
    core: Core<'a>,
}

impl<'a, S> Compiler<'a, S> {
    pub fn source(&self) -> &SourceBuf {
        self.core.source
    }

    pub fn stage(&self) -> &S {
        &self.stage
    }

    fn with_state<O, F>(self, f: F) -> Compiler<'a, O>
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

impl<'a> Compiler<'a, Empty> {
    pub fn new(source: &'a SourceBuf) -> Self {
        Self {
            core: Core { source },
            stage: Empty,
        }
    }

    pub fn tokenize(self) -> Compiler<'a, Lexed> {
        self.with_state(|_, core| Lexed {
            tt: Lexer::new(core.source).tokenize(),
        })
    }
}

impl<'a> Compiler<'a, Lexed> {
    pub fn parse(self, clinic: &mut Clinic) -> Option<Compiler<'a, Parsed>> {
        match parser::parse(&self) {
            Ok(ast) => Some(self.with_state(|stage, _| Parsed { tt: stage.tt, ast })),
            Err(e) => {
                clinic.report(CompilationError::Parser(e));
                None
            }
        }
    }
}

impl<'a> Compiler<'a, Parsed> {
    pub fn lower(
        self,
        symbols: &mut SymbolTable,
        clinic: &mut Clinic,
    ) -> Option<Compiler<'a, Hir>> {
        hir::lower(&self, symbols, clinic).map(|hir| {
            println!("{hir:#?}");
            self.with_state(|_, _| state::Hir { hir })
        })
    }
}

impl<'a> Compiler<'a, Hir> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn typecheck(self) -> Option<Self> {
        Some(self)
    }

    pub fn lower(
        self,
        symbols: &mut SymbolTable,
        clinic: &mut Clinic,
    ) -> Option<Compiler<'a, Mir>> {
        mir::lower(&self, symbols, clinic).map(|mir| {
            println!("{mir}");
            self.with_state(|_, _| state::Mir { mir })
        })
    }
}

impl<'a> Compiler<'a, Mir> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn lower(self) -> Option<Compiler<'a, Bytecode>> {
        Some(Compiler {
            stage: Bytecode {},
            core: self.core,
        })
    }
}

impl<'a> Compiler<'a, Bytecode> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn to_unit(self) -> CompilationUnit {
        CompilationUnit {
            bindings: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct CompilationUnit {
    pub(super) bindings: HashMap<SymbolId, Binding>,
}

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
            .to_unit(),
    )
}
