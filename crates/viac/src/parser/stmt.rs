/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::stmt::{Stmt, StmtKind};

impl Parser<'_> {
    pub(super) fn parse_stmt(&mut self) -> Result<Stmt> {
        let token = self.peek()?;

        match token.kind {
            _ if self.is_expr_start() => {
                let expr = self.parse_expr()?;
                Ok(Stmt {
                    span: expr.span,
                    kind: if optional!(self, Semi) {
                        StmtKind::Discard(expr)
                    } else {
                        StmtKind::Consume(expr)
                    },
                })
            }
            _ => Err(Error::UnexpectedToken(token.span)),
        }
    }
}
