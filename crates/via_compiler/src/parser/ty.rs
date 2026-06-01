use super::prelude::*;
use crate::ast::{Ty, TyKind};

impl<'a> Parser<'a> {
    pub(crate) fn parse_type(&mut self, tree: &mut Tree) -> Result<Ty> {
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
                        let ty = self.parse_type(tree)?;
                        expect_one!(self, RParen)?;
                        ty
                    }
                }
            }
            LBracket => {
                let open = self.consume()?;
                let ty = self.parse_type(tree)?;

                if optional!(self, Semi) {
                    let size = self.parse_expr(tree)?;
                    let close = expect_one!(self, RBracket)?;

                    Ty {
                        kind: TyKind::Array {
                            ty: tree.alloc_ty(ty),
                            size: tree.alloc_expr(size),
                        },
                        span: SourceSpan::merge(open.span, close.span),
                    }
                } else {
                    let close = expect_one!(self, RBracket)?;

                    Ty {
                        kind: TyKind::Vector(tree.alloc_ty(ty)),
                        span: SourceSpan::merge(open.span, close.span),
                    }
                }
            }
            Ident(name) => {
                self.consume()?;
                Ty {
                    kind: match &name[..] {
                        "Bool" => TyKind::Bool,
                        "Int" => TyKind::Int,
                        "Float" => TyKind::Float,
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
