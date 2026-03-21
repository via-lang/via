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
    mir::builder::MirBuilder,
    module::{error::Error, symbol::SymbolTable},
    parser::Parser,
    sema::context::SemContext,
    source::SourceBuf,
};

pub type Result<T> = std::result::Result<T, Error>;

pub mod state {
    use crate::mir;

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
    pub struct Bytecode;
}

use state::*;

pub struct Compiler<S>(pub S);

impl Compiler<Empty> {
    pub fn new() -> Self {
        Self(Empty)
    }

    pub fn tokenize(self, source: &SourceBuf) -> Compiler<Lexed> {
        let tt = Lexer::new(source).tokenize();
        Compiler(Lexed { tt })
    }
}

impl Compiler<Lexed> {
    pub fn parse(self, clinic: &mut Clinic) -> Option<Compiler<Parsed>> {
        dbg!(&self.0.tt);

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
        symbols: &mut SymbolTable,
        clinic: &mut Clinic,
        sema: &mut SemContext,
    ) -> Option<Compiler<Hir>> {
        dbg!(&self.0.ast);

        HirBuilder::new(clinic, symbols, sema, &self.0.ast)
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

    pub fn lower(self, symbols: &mut SymbolTable, clinic: &mut Clinic) -> Option<Compiler<Mir>> {
        dbg!(&self.0.hir);

        MirBuilder::new(symbols, clinic, &self.0.hir)
            .lower()
            .map(|mir| Compiler(Mir { mir }))
    }
}

impl Compiler<Mir> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn lower(self) -> Option<Compiler<Bytecode>> {
        dbg!(self.0.mir);

        Some(Compiler(Bytecode {}))
    }
}

impl Compiler<Bytecode> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn to_unit(self) -> CompilationUnit {
        CompilationUnit {}
    }
}

#[derive(Debug)]
pub struct CompilationUnit {}
