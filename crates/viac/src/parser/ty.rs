/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::{
    Tree,
    ty::{self, TyId},
};

yes_or_no!(pub AllowRaiseClause);

impl Parser<'_> {
    pub(crate) fn parse_type(
        &mut self,
        tree: &mut Tree,
        allow_raise: AllowRaiseClause,
    ) -> Result<TyId> {
        let token = self.peek()?;
        let mut lhs = match token.kind {
            KwNone | KwBool | KwInt | KwFloat | KwString => tree.insert(
                ty::Builtin {
                    span: token.span,
                    token: self.consume()?,
                }
                .into(),
            ),
            LParen => {
                self.consume()?.span;

                let ty = self.parse_type(tree, allow_raise)?;
                expect_one!(self, RParen)?.span;
                ty
            }
            LBracket => {
                self.consume()?;

                let ty = self.parse_type(tree, AllowRaiseClause::No)?;
                let last = expect_one!(self, RBracket)?;

                tree.insert(
                    ty::Array {
                        span: SourceSpan::new(token.span.begin, last.span.end),
                        ty: ty.into(),
                    }
                    .into(),
                )
            }
            LBrace => self.with_context(Context::TypeMap, |parser| -> Result<TyId> {
                parser.consume()?;
                let key = parser.parse_type(tree, AllowRaiseClause::No)?;

                expect_one!(parser, Col)?;

                let value = parser.parse_type(tree, AllowRaiseClause::No)?;
                let last = expect_one!(parser, RBrace)?;

                Ok(tree.insert(
                    ty::Map {
                        span: SourceSpan::new(token.span.begin, last.span.end),
                        key: key.into(),
                        value: value.into(),
                    }
                    .into(),
                ))
            })?,
            KwFn => self.with_context(Context::TypeFn, |parser| -> Result<TyId> {
                parser.consume()?;

                let params = parser.parse_list(tree, (LParen, RParen), |parser, tree| {
                    parser.parse_type(tree, AllowRaiseClause::No)
                })?;

                expect_one!(parser, Arrow)?;

                let result = parser.parse_return_ty(tree)?;
                let last = tree.get(result).span();

                Ok(tree.insert(
                    ty::Function {
                        span: SourceSpan::new(token.span.begin, last.end),
                        params: params.inner,
                        result: result.into(),
                    }
                    .into(),
                ))
            })?,
            KwType => self.with_context(Context::TypeId, |parser| -> Result<TyId> {
                parser.consume()?;
                expect_one!(parser, LParen)?;

                let expr = parser.parse_expr(tree)?;
                let last = expect_one!(parser, RParen)?;

                Ok(tree.insert(
                    ty::TypeOf {
                        span: SourceSpan::new(token.span.begin, last.span.end),
                        expr: expr.into(),
                    }
                    .into(),
                ))
            })?,
            _ => {
                return Err(Error::UnexpectedToken {
                    src: self.src.clone(),
                    span: token.span.to_miette_span(),
                    expected: vec![].into(),
                    got: self.src.get_span(token.span).to_owned(),
                });
            }
        };

        loop {
            let first = tree.get(lhs).span();
            lhs = match self.peek().map(|t| t.kind) {
                Ok(Quest) => {
                    self.consume()?;
                    tree.insert(
                        ty::Optional {
                            span: first,
                            ty: lhs.into(),
                        }
                        .into(),
                    )
                }
                Ok(Pipe) => self.with_context(Context::TypeUnion, |parser| -> Result<TyId> {
                    parser.consume()?;

                    let rhs = parser.parse_type(tree, AllowRaiseClause::No)?;
                    let last = tree.get(rhs).span();

                    Ok(tree.insert(
                        ty::Union {
                            span: SourceSpan::new(first.begin, last.end),
                            lhs: lhs.into(),
                            rhs: rhs.into(),
                        }
                        .into(),
                    ))
                })?,
                Ok(KwRaise) => {
                    let token = self.consume()?;

                    if !bool::from(allow_raise) {
                        return Err(Error::UnexpectedRaiseClause {
                            src: self.src.clone(),
                            span: token.span.to_miette_span(),
                        });
                    }

                    let rhs = self.parse_type(tree, AllowRaiseClause::No)?;
                    let last = tree.get(rhs).span();

                    tree.insert(
                        ty::Effect {
                            span: SourceSpan::new(first.begin, last.end),
                            lhs: lhs.into(),
                            rhs: rhs.into(),
                        }
                        .into(),
                    )
                }
                _ => break Ok(lhs),
            };
        }
    }

    pub(crate) fn parse_return_ty(&mut self, tree: &mut Tree) -> Result<TyId> {
        self.parse_type(tree, AllowRaiseClause::Yes)
    }

    pub(crate) fn parse_param_ty(&mut self, tree: &mut Tree) -> Result<TyId> {
        self.parse_type(tree, AllowRaiseClause::No)
    }

    pub(crate) fn parse_cast_ty(&mut self, tree: &mut Tree) -> Result<TyId> {
        self.parse_type(tree, AllowRaiseClause::No)
    }
}
