/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::ty::{self, Ty};

yes_or_no!(pub AllowRaiseClause);

impl Parser<'_> {
    pub(crate) fn parse_type(&mut self, allow_raise: AllowRaiseClause) -> Result<Node<Ty>> {
        let token = self.peek()?;
        let mut lhs = match token.kind {
            KwNone | KwBool | KwInt | KwFloat | KwString => Node {
                node: ty::Builtin {
                    token: self.consume()?,
                }
                .into(),
                span: token.span,
                attrs: None,
            },
            LParen => {
                let first = self.consume()?.span;
                let ty = self.parse_type(allow_raise)?;
                let last = expect_one!(self, RParen)?.span;
                Node {
                    node: ty.node,
                    span: SourceSpan::new(first.begin, last.end),
                    attrs: None,
                }
            }
            LBracket => {
                self.consume()?;
                let ty = self.parse_type(AllowRaiseClause::No)?;
                let last = expect_one!(self, RBracket)?;

                Node {
                    node: ty::Array { ty: ty.into() }.into(),
                    span: SourceSpan::new(token.span.begin, last.span.end),
                    attrs: None,
                }
            }
            LBrace => self.with_context(Context::TypeMap, |parser| -> Result<Node<Ty>> {
                parser.consume()?;
                let key = parser.parse_type(AllowRaiseClause::No)?;

                expect_one!(parser, Col)?;

                let value = parser.parse_type(AllowRaiseClause::No)?;
                let last = expect_one!(parser, RBrace)?;

                Ok(Node {
                    node: ty::Map {
                        key: key.into(),
                        value: value.into(),
                    }
                    .into(),
                    span: SourceSpan::new(token.span.begin, last.span.end),
                    attrs: None,
                })
            })?,
            KwFn => self.with_context(Context::TypeFn, |parser| -> Result<Node<Ty>> {
                parser.consume()?;
                let params = parser.parse_list((LParen, RParen), |parser| {
                    parser.parse_type(AllowRaiseClause::No)
                })?;

                expect_one!(parser, Arrow)?;

                let result = parser.parse_return_ty()?;
                let last = result.span.clone();

                Ok(Node {
                    node: ty::Function {
                        params,
                        result: result.into(),
                    }
                    .into(),
                    span: SourceSpan::new(token.span.begin, last.end),
                    attrs: None,
                })
            })?,
            KwType => self.with_context(Context::TypeId, |parser| -> Result<Node<Ty>> {
                parser.consume()?;
                expect_one!(parser, LParen)?;

                let expr = parser.parse_expr()?;
                let last = expect_one!(parser, RParen)?;

                Ok(Node {
                    node: ty::TypeOf { expr: expr.into() }.into(),
                    span: SourceSpan::new(token.span.begin, last.span.end),
                    attrs: None,
                })
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
            let first = lhs.span.clone();
            lhs = match self.peek().map(|t| t.kind) {
                Ok(Quest) => {
                    self.consume()?;
                    Node {
                        node: ty::Optional { ty: lhs.into() }.into(),
                        span: first,
                        attrs: None,
                    }
                }
                Ok(Pipe) => {
                    self.with_context(Context::TypeUnion, |parser| -> Result<Node<Ty>> {
                        parser.consume()?;
                        let rhs = parser.parse_type(AllowRaiseClause::No)?;
                        let last = rhs.span.clone();
                        Ok(Node {
                            node: ty::Union {
                                lhs: lhs.into(),
                                rhs: rhs.into(),
                            }
                            .into(),
                            span: SourceSpan::new(first.begin, last.end),
                            attrs: None,
                        })
                    })?
                }
                Ok(KwRaise) => {
                    let token = self.consume()?;
                    if !bool::from(allow_raise) {
                        return Err(Error::UnexpectedRaiseClause {
                            src: self.src.clone(),
                            span: token.span.to_miette_span(),
                        });
                    }

                    let rhs = self.parse_type(AllowRaiseClause::No)?;
                    let last = rhs.span.clone();
                    Node {
                        node: ty::Effect {
                            lhs: lhs.into(),
                            rhs: rhs.into(),
                        }
                        .into(),
                        span: SourceSpan::new(first.begin, last.end),
                        attrs: None,
                    }
                }
                _ => break Ok(lhs),
            };
        }
    }

    pub(crate) fn parse_return_ty(&mut self) -> Result<Node<Ty>> {
        self.parse_type(AllowRaiseClause::Yes)
    }

    pub(crate) fn parse_param_ty(&mut self) -> Result<Node<Ty>> {
        self.parse_type(AllowRaiseClause::No)
    }

    pub(crate) fn parse_cast_ty(&mut self) -> Result<Node<Ty>> {
        self.parse_type(AllowRaiseClause::No)
    }
}
