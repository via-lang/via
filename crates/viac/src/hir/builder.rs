/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{Hir, block::Block};
use crate::{
    ast::Tree,
    clinic::{Clinic, Diagnostic, StageControl},
    hir::{block::BlockId, instr::Instr, term::Term},
    source::SourceBuf,
};

#[derive(Debug)]
pub struct IrBuilder<'a> {
    pub(super) source: SourceBuf,
    pub(super) ast: &'a Tree,
    pub(super) clinic: &'a mut Clinic,
}

impl<'a> IrBuilder<'a> {
    pub fn new(source: &SourceBuf, ast: &'a Tree, clinic: &'a mut Clinic) -> Self {
        Self {
            source: source.clone(),
            ast,
            clinic,
        }
    }

    pub fn block(&mut self, hir: &mut Hir) -> BlockId {
        let len = hir.blocks.len();
        let id = BlockId::from(len as u32);
        hir.blocks.push(Block::new(id));
        id
    }

    pub fn terminate(&mut self, hir: &mut Hir, block: BlockId, term: Term) {
        hir.get_mut(block).term = term;
    }

    pub fn push(&mut self, hir: &mut Hir, block: BlockId, instr: Instr) {
        hir.get_mut(block).instrs.push(instr);
    }

    pub(crate) fn lower(&mut self) -> Hir {
        let mut hir = Hir::default();
        let mut current = self.block(&mut hir);

        for stmt in &self.ast.stmts {
            match self.lower_stmt(&mut hir, current, self.ast.get(*stmt)) {
                Ok(b) => current = b,
                Err(e) => {
                    self.clinic.report(Diagnostic {
                        report: miette::Report::new(e),
                        control: StageControl::Terminate,
                    });
                    continue;
                }
            };
        }

        hir
    }
}
