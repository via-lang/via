/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod binding;
pub mod context;
pub mod error;
pub mod symbol;
pub mod tree;

use std::{collections::HashMap, rc::Rc};

use miette::Report;

use crate::{
    ast::{node::Node, stmt::Stmt},
    clinic::{Clinic, Diagnostic, StageControl},
    lexer::{self, token::Token},
    parser,
    source::SourceBuf,
};
use binding::Binding;
use error::{Error, Result};
use symbol::SymbolId;

#[derive(Debug, Clone)]
pub struct Fixture {
    pub tt: Rc<[Token]>,
    pub ast: Rc<[Node<Stmt>]>,
}

#[derive(Debug)]
pub enum ModuleKind {
    Source { source: SourceBuf, fixture: Fixture },
}

#[derive(Debug)]
pub struct Module {
    kind: ModuleKind,
    bindings: HashMap<SymbolId, Binding>,
}

impl Module {
    pub(crate) fn new(src: &SourceBuf) -> Result<Module> {
        let mut clinic = Clinic::default();

        let tt = lexer::tokenize(src);
        let ast = clinic.run_stage(|clinic| {
            parser::parse(src, &tt)
                .map_err(|e| {
                    clinic.report(Diagnostic {
                        report: Report::new(e),
                        control: StageControl::Terminate,
                    })
                })
                .ok()
        });

        clinic
            .finish()
            .then(|| Self {
                kind: ModuleKind::Source {
                    source: src.clone(),
                    fixture: Fixture {
                        tt,
                        ast: ast.unwrap(),
                    },
                },
                bindings: HashMap::new(),
            })
            .ok_or(Error::CompilationError)
    }

    pub fn kind(&self) -> &ModuleKind {
        &self.kind
    }

    pub fn get_symbol(&self, symbol: SymbolId) -> Option<&Binding> {
        self.bindings.get(&symbol)
    }
}
