/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::Diagnostic;
use crate::builder::Builder;
use crate::renderer::Renderer;
use std::rc::Rc;
use viac_source::Source;

#[derive(Debug)]
pub struct Context<R: Renderer> {
    pub(crate) count: usize,
    pub(crate) renderer: R,
    pub(crate) source: Option<Rc<Source>>,
}

impl<R: Renderer> Context<R> {
    pub fn new(renderer: R, source: Option<&Rc<Source>>) -> Self {
        Self {
            count: 0,
            renderer,
            source: source.cloned(),
        }
    }

    pub fn emit<D: Diagnostic>(&mut self, diag: D) -> Result<(), R::Error> {
        let mut builder = Builder::new(self.source.clone());
        diag.build(&mut builder);

        let diag = builder.build();
        self.count += 1;
        self.renderer.render(self.source.clone(), &diag)
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
