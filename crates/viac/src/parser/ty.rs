use super::prelude::*;
use crate::ast::{Ty, TyKind};

impl<'a> Parser<'a> {
    pub(crate) fn parse_type(&mut self, _tree: &mut Tree) -> Result<Ty> {
        let token = self.peek()?;
        let lhs = match token.kind {
            LParen => {
                self.consume()?;

                match self.peek()?.kind {
                    RParen => {
                        self.consume()?;
                        Ty {
                            kind: TyKind::Unit,
                            span: token.span,
                        }
                    }
                    _ => {
                        let ty = self.parse_type(_tree)?;
                        expect_one!(self, RParen)?;
                        ty
                    }
                }
            }
            Ident(name) => {
                self.consume()?;
                Ty {
                    kind: match &name[..] {
                        "bool" => TyKind::Bool,
                        "int" => TyKind::Int,
                        "float" => TyKind::Float,
                        _ => return Err(Error::UnexpectedToken(token.span)),
                    },
                    span: token.span,
                }
            }
            _ => {
                return Err(Error::UnexpectedToken(token.span));
            }
        };

        Ok(lhs)
    }
}
