/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use std::rc::Rc;

use crate::diag::{Diag, DiagKind, Note};
use viac_source::Source;
use viac_source::span::Span;

#[derive(Debug)]
pub struct Builder {
    diag: Diag,
    pub source: Option<Rc<Source>>,
}

impl Builder {
    pub fn new(source: Option<Rc<Source>>) -> Self {
        Self {
            diag: Diag {
                kind: None,
                message: "<no message>".to_string(),
                location: None,
                context: Vec::new(),
                notes: Vec::new(),
            },
            source,
        }
    }

    pub fn kind(&mut self, kind: DiagKind) -> &mut Self {
        self.diag.kind = Some(kind);
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

    pub(crate) fn build(&self) -> Diag {
        self.diag.clone()
    }
}
