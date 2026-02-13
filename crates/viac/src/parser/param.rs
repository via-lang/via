/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::param::{Param, ParamKind, ParamQuals, Params, ThisParam};

yes_or_no!(pub AllowSelfParam);
yes_or_no!(pub AllowNamedParam);

#[derive(Debug)]
pub enum OmitEmptyParams {
    Yes { fallback: SourceSpan },
    No,
}

enum ParamVariant {
    This { quals: ParamQuals, span: SourceSpan },
    Param(Param),
}

impl Parser<'_> {
    fn parse_param(
        &mut self,
        tree: &mut Tree,
        allow_named: AllowNamedParam,
    ) -> Result<ParamVariant> {
        if !bool::from(allow_named) {
            let ty = self.parse_param_ty(tree)?;
            let span = ty.span();

            return Ok(ParamVariant::Param(Param {
                kind: ParamKind::Anonymous {
                    ty: tree.insert(ty),
                },
                span,
            }));
        }

        let first = self.peek()?;

        let mut quals = ParamQuals::None;

        if optional!(self, Amp) {
            quals |= ParamQuals::Borrow;
        }

        if optional!(self, KwMut) {
            quals |= ParamQuals::Mutable;
        }

        let token = self.peek()?;

        match token.kind {
            TokenKind::KwSelf => {
                let last = self.consume()?; // consume `self`
                Ok(ParamVariant::This {
                    quals,
                    span: SourceSpan::merge(first.span, last.span),
                })
            }
            TokenKind::Ident { .. } => {
                self.consume()?;

                expect_one!(self, Col)?;

                let ty = self.parse_param_ty(tree)?;
                let span = ty.span();

                Ok(ParamVariant::Param(Param {
                    kind: ParamKind::Named {
                        quals,
                        name: token,
                        ty: tree.insert(ty),
                    },
                    span: SourceSpan::merge(first.span, span),
                }))
            }
            _ => Err(Error::UnexpectedToken {
                span: token.span.into(),
                expected: vec!["identifier", "`self`"].into(),
                got: self.src.get_span(&token.span).to_owned(),
            }),
        }
    }

    pub(super) fn parse_params(
        &mut self,
        tree: &mut Tree,
        omit_empty: OmitEmptyParams,
        allow_named: AllowNamedParam,
    ) -> Result<Params> {
        if !check!(self, LParen) {
            match omit_empty {
                OmitEmptyParams::Yes { fallback } => {
                    return Ok(Params {
                        this: None,
                        inner: vec![],
                        span: fallback,
                    });
                }
                _ => {
                    expect_one!(self, LParen)?;
                }
            }
        }

        let first = expect_one!(self, LParen)?;

        let mut this = None;
        let mut inner = vec![];

        while !check!(self, RParen) {
            match self.parse_param(tree, allow_named)? {
                ParamVariant::This { quals, span } => {
                    if inner.is_empty() {
                        this = Some(ThisParam { quals, span });
                        continue;
                    }
                    return Err(Error::UnexpectedSelfParam { span: span.into() });
                }
                ParamVariant::Param(param) => inner.push(param),
            };

            if !optional!(self, Comma) {
                break;
            }
        }

        let last = expect_one!(self, RParen)?;

        Ok(Params {
            this,
            inner,
            span: SourceSpan::merge(first.span, last.span),
        })
    }
}
