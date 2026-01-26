/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::rc::Rc;

use super::{Diagnostic, Note, Severity};
use crate::source::Source;
use crate::source::span::Span;

#[derive(Debug)]
pub struct Builder {
    diag: Diagnostic,
    pub src: Rc<Source>,
}

impl Builder {
    pub fn new(src: &Rc<Source>, severity: Severity) -> Self {
        Self {
            diag: Diagnostic {
                severity,
                code: None,
                message: "<no message>".to_string(),
                location: None,
                context: Vec::new(),
                notes: Vec::new(),
            },
            src: src.clone(),
        }
    }

    pub fn code(&mut self, code: &'static str) -> &mut Self {
        self.diag.code = Some(code);
        self
    }

    pub fn message(&mut self, msg: String) -> &mut Self {
        self.diag.message = msg;
        self
    }

    pub fn location(&mut self, loc: Span) -> &mut Self {
        self.diag.location = Some(loc);
        self
    }

    pub fn context(&mut self, ctxt: String) -> &mut Self {
        self.diag.context.push(ctxt);
        self
    }

    pub fn note(&mut self, note: Note) -> &mut Self {
        self.diag.notes.push(note);
        self
    }

    pub fn build(&self) -> Diagnostic {
        self.diag.clone()
    }
}
