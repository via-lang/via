/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::{collections::HashMap, fmt};

use crate::{
    ast::Tree,
    clinic::{Clinic, Diagnostic, StageControl},
    lexer::{Lexer, token::Token},
    module::{binding::Binding, symbol::SymbolId},
    parser,
    source::SourceBuf,
};

pub type Result<T> = std::result::Result<T, Ice>;

#[derive(Debug)]
pub enum IceKind {
    ParseError,
    IrError,
    TypeError,
    BytecodeError,
}

#[derive(Debug)]
pub struct Ice {
    pub kind: IceKind,
    pub what: String,
}

impl fmt::Display for Ice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            IceKind::ParseError => write!(f, "parsing: {}", self.what),
            IceKind::IrError => write!(f, "ir-lowering: {}", self.what),
            IceKind::TypeError => write!(f, "type-checking: {}", self.what),
            IceKind::BytecodeError => write!(f, "bytecode-lowering: {}", self.what),
        }
    }
}

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
    pub struct Ir;

    #[derive(Debug)]
    pub struct Bytecode;
}

use state::*;

#[derive(Debug)]
struct Core<'a> {
    source: &'a SourceBuf,
    clinic: &'a mut Clinic,
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
    pub fn new(source: &'a SourceBuf, clinic: &'a mut Clinic) -> Self {
        Self {
            core: Core { source, clinic },
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
    pub fn parse(self) -> Result<Compiler<'a, Parsed>> {
        match parser::parse(&self) {
            Ok(ast) => Ok(self.with_state(|stage, _| Parsed { tt: stage.tt, ast })),
            Err(e) => {
                self.core.clinic.report(Diagnostic {
                    report: miette::Report::new(e),
                    control: StageControl::Terminate,
                });
                Err(Ice {
                    kind: IceKind::ParseError,
                    what: "unimplemented".to_string(),
                })
            }
        }
    }
}

impl<'a> Compiler<'a, Parsed> {
    pub fn lower(self) -> Result<Compiler<'a, Ir>> {
        Err(Ice {
            kind: IceKind::IrError,
            what: "unimplemented".to_string(),
        })
    }
}

impl<'a> Compiler<'a, Ir> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn typecheck(self) -> Result<Self> {
        Err(Ice {
            kind: IceKind::TypeError,
            what: "unimplemented".to_string(),
        })
    }

    pub fn lower(self) -> Result<Compiler<'a, Bytecode>> {
        Err(Ice {
            kind: IceKind::BytecodeError,
            what: "unimplemented".to_string(),
        })
    }
}

impl<'a> Compiler<'a, Bytecode> {
    pub fn optimize(self) -> Self {
        self
    }

    pub fn to_unit(self) -> CompilationUnit {
        todo!()
    }
}

#[derive(Debug)]
pub struct CompilationUnit {
    pub(super) bindings: HashMap<SymbolId, Binding>,
}

pub fn compile(src: &SourceBuf, clinic: &mut Clinic) -> Result<CompilationUnit> {
    Ok(Compiler::new(src, clinic)
        .tokenize()
        .parse()?
        .lower()?
        .typecheck()?
        .optimize()
        .lower()?
        .optimize()
        .to_unit())
}
