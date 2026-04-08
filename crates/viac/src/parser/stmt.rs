use super::prelude::*;
use crate::ast::{Stmt, StmtKind};

impl<'a> Parser<'a> {
    pub(super) fn parse_stmt(&mut self, tree: &mut Tree) -> Result<Stmt> {
        let token = self.peek()?;
        let stmt = match token.kind {
            KwLet => {
                self.consume()?;

                let ident = self.consume()?;
                let ident = match ident.kind {
                    Ident(ident) => ident,
                    _ => return Err(Error::UnexpectedToken(ident.span)),
                };

                let ty = optional!(self, Colon)
                    .then(|| self.parse_type(tree))
                    .transpose()?;

                expect_one!(self, Eq)?;

                let expr = self.parse_expr(tree)?;
                let semi = expect_one!(self, Semi)?;

                Stmt {
                    kind: StmtKind::Let {
                        ident,
                        ty: ty.map(|ty| tree.alloc_ty(ty)),
                        expr: tree.alloc_expr(expr),
                    },
                    span: SourceSpan::merge(token.span, semi.span),
                }
            }
            _ if self.is_expr_start() => {
                let expr = self.parse_expr(tree)?;
                Stmt {
                    span: expr.span,
                    kind: if optional!(self, Semi) {
                        StmtKind::Discard(tree.alloc_expr(expr))
                    } else {
                        StmtKind::Consume(tree.alloc_expr(expr))
                    },
                }
            }
            _ => return Err(Error::UnexpectedToken(token.span)),
        };

        Ok(stmt)
    }
}
