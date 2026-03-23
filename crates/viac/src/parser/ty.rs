use super::prelude::*;
use crate::ast::ty::{Ty, TyKind};

impl<'a> Parser<'a> {
    pub(crate) fn parse_type(&mut self, _tree: &mut Tree) -> Result<Ty> {
        let token = self.peek()?;
        let lhs = match token.kind {
            KwNone => Ty {
                kind: TyKind::None,
                span: token.span,
            },
            KwBool => Ty {
                kind: TyKind::Bool,
                span: token.span,
            },
            KwInt => Ty {
                kind: TyKind::Int,
                span: token.span,
            },
            KwFloat => Ty {
                kind: TyKind::Float,
                span: token.span,
            },
            _ => {
                return Err(Error::UnexpectedToken(token.span));
            }
        };

        Ok(lhs)
    }
}
