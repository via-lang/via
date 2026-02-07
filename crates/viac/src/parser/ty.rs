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
    ty::{self, Ty},
};

yes_or_no!(pub AllowRaiseClause);

impl Parser<'_> {
    pub(crate) fn parse_type(
        &mut self,
        tree: &mut Tree,
        allow_raise: AllowRaiseClause,
    ) -> Result<Ty> {
        let token = self.peek()?;
        let mut lhs = match token.kind {
            KwNone | KwBool | KwInt | KwFloat | KwString => ty::Builtin {
                span: token.span,
                token: self.consume()?,
            }
            .into(),
            LParen => {
                self.consume()?;

                let ty = self.parse_type(tree, allow_raise)?;
                expect_one!(self, RParen)?;
                ty
            }
            LBracket => {
                self.consume()?;

                let ty = self.parse_type(tree, AllowRaiseClause::No)?;
                let last = expect_one!(self, RBracket)?;

                ty::Array {
                    span: SourceSpan::new(token.span.begin, last.span.end),
                    ty: tree.insert(ty),
                }
                .into()
            }
            LBrace => self.with_context(Context::TypeMap, |parser| -> Result<Ty> {
                parser.consume()?;
                let key = parser.parse_type(tree, AllowRaiseClause::No)?;

                expect_one!(parser, Col)?;

                let value = parser.parse_type(tree, AllowRaiseClause::No)?;
                let last = expect_one!(parser, RBrace)?;

                Ok(ty::Map {
                    span: SourceSpan::new(token.span.begin, last.span.end),
                    key: tree.insert(key),
                    value: tree.insert(value),
                }
                .into())
            })?,
            KwFn => self.with_context(Context::TypeFn, |parser| -> Result<Ty> {
                parser.consume()?;

                let params = parser.parse_list(tree, (LParen, RParen), |parser, tree| {
                    parser.parse_type(tree, AllowRaiseClause::No)
                })?;

                expect_one!(parser, Arrow)?;

                let result = parser.parse_return_ty(tree)?;
                let last = result.span();

                Ok(ty::Function {
                    span: SourceSpan::new(token.span.begin, last.end),
                    params: params.inner,
                    result: tree.insert(result),
                }
                .into())
            })?,
            KwType => self.with_context(Context::TypeId, |parser| -> Result<Ty> {
                parser.consume()?;
                expect_one!(parser, LParen)?;

                let expr = parser.parse_expr(tree)?;
                let last = expect_one!(parser, RParen)?;

                Ok(ty::TypeOf {
                    span: SourceSpan::new(token.span.begin, last.span.end),
                    expr: tree.insert(expr),
                }
                .into())
            })?,
            _ => {
                return Err(Error::UnexpectedToken {
                    span: token.span.to_miette_span(),
                    expected: vec![].into(),
                    got: self.src.get_span(&token.span).to_owned(),
                });
            }
        };

        loop {
            let first = lhs.span();
            lhs = match self.peek().map(|t| t.kind) {
                Ok(Quest) => {
                    self.consume()?;
                    ty::Optional {
                        span: first,
                        ty: tree.insert(lhs),
                    }
                    .into()
                }
                Ok(Pipe) => self.with_context(Context::TypeUnion, |parser| -> Result<Ty> {
                    parser.consume()?;

                    let rhs = parser.parse_type(tree, AllowRaiseClause::No)?;
                    let last = rhs.span();

                    Ok(ty::Union {
                        span: SourceSpan::new(first.begin, last.end),
                        lhs: tree.insert(lhs),
                        rhs: tree.insert(rhs),
                    }
                    .into())
                })?,
                Ok(KwRaise) => {
                    let token = self.consume()?;

                    if !bool::from(allow_raise) {
                        return Err(Error::UnexpectedRaiseClause {
                            span: token.span.to_miette_span(),
                        });
                    }

                    let rhs = self.parse_type(tree, AllowRaiseClause::No)?;
                    let last = rhs.span();

                    ty::Effect {
                        span: SourceSpan::new(first.begin, last.end),
                        lhs: tree.insert(lhs),
                        rhs: tree.insert(rhs),
                    }
                    .into()
                }
                _ => break Ok(lhs),
            };
        }
    }

    pub(crate) fn parse_return_ty(&mut self, tree: &mut Tree) -> Result<Ty> {
        self.parse_type(tree, AllowRaiseClause::Yes)
    }

    pub(crate) fn parse_param_ty(&mut self, tree: &mut Tree) -> Result<Ty> {
        self.parse_type(tree, AllowRaiseClause::No)
    }

    pub(crate) fn parse_cast_ty(&mut self, tree: &mut Tree) -> Result<Ty> {
        self.parse_type(tree, AllowRaiseClause::No)
    }
}
