/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::IntoDiag;
use super::renderer::Renderer;
use crate::source::Source;
use std::rc::Rc;

#[derive(Debug)]
pub struct Context<R: Renderer> {
    pub(crate) count: usize,
    pub(crate) renderer: R,
    pub(crate) src: Rc<Source>,
}

impl<R: Renderer> Context<R> {
    pub fn new(src: &Rc<Source>, renderer: R) -> Self {
        Self {
            count: 0,
            renderer,
            src: src.clone(),
        }
    }

    pub fn emit<D: IntoDiag>(&mut self, diag: D) -> Result<(), R::E> {
        let diag = diag.into_diag(&self.src);
        self.count += 1;
        self.renderer.render(&diag)
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
