/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::ty::{Ty, TyKind};

impl Parser<'_> {
    pub(crate) fn parse_type(&mut self) -> Result<Ty> {
        let token = self.peek()?;
        let mut lhs = match token.kind {
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

        loop {
            lhs = match self.peek().map(|t| t.kind) {
                _ => break Ok(lhs),
            };
        }
    }
}
