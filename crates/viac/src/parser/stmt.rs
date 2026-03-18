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
            KwLet => {
                self.consume()?;

                let ident = self.consume()?;
                let ident = match ident.kind {
                    Ident(ident) => ident,
                    _ => return Err(Error::UnexpectedToken(ident.span)),
                };

                let ty = optional!(self, Col)
                    .then(|| self.parse_type())
                    .transpose()?;

                expect_one!(self, Eq)?;

                let expr = self.parse_expr()?;
                let semi = expect_one!(self, Semi)?;

                Ok(Stmt {
                    kind: StmtKind::Let {
                        ident,
                        ty: ty.map(Box::new),
                        expr: Box::new(expr),
                    },
                    span: SourceSpan::merge(token.span, semi.span),
                })
            }
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
