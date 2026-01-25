/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Parser;
use super::macros::yes_or_no;
use super::prelude::*;
use crate::ast::ty::{self, Ty};

yes_or_no!(pub AllowEffect);

impl Parser {
    pub(crate) fn parse_type(&mut self, allow_effect: AllowEffect) -> Result<Node<Ty>> {
        let token = self.peek()?;
        let mut lhs = match token.kind {
            KwNone | KwBool | KwInt | KwFloat | KwString => Node {
                node: ty::Builtin {
                    token: self.consume()?,
                }
                .into(),
                span: token.span,
                attrs: vec![],
            },
            ParenOpen => {
                let first = self.consume()?.span;
                let ty = self.parse_type(allow_effect)?;
                let last = expect_token!(self, ParenClose)?.span;
                Node {
                    node: ty.node,
                    span: span![first.begin, last.end],
                    attrs: vec![],
                }
            }
            BracketOpen => {
                self.consume()?;
                let ty = self.parse_type(AllowEffect::No)?;
                let last = expect_token!(self, BracketClose)?;

                Node {
                    node: ty::Array { ty: ty.into() }.into(),
                    span: span![token.span.begin, last.span.end],
                    attrs: vec![],
                }
            }
            BraceOpen => self.with_context(Context::TypeMap, |p| {
                p.consume()?;
                let key = p.parse_type(AllowEffect::No)?;

                expect_token!(p, Colon)?;

                let value = p.parse_type(AllowEffect::No)?;
                let last = expect_token!(p, BraceClose)?;

                Ok(Node {
                    node: ty::Map {
                        key: key.into(),
                        value: value.into(),
                    }
                    .into(),
                    span: span![token.span.begin, last.span.end],
                    attrs: vec![],
                })
            })?,
            KwFn => self.with_context(Context::TypeFn, |p| {
                p.consume()?;
                let params =
                    p.parse_list((ParenOpen, ParenClose), |p| p.parse_type(AllowEffect::No))?;

                expect_token!(p, Arrow)?;

                let result = p.parse_return_ty()?;
                let last = result.span;

                Ok(Node {
                    node: ty::Function {
                        params,
                        result: result.into(),
                    }
                    .into(),
                    span: span![token.span.begin, last.end],
                    attrs: vec![],
                })
            })?,
            KwType => self.with_context(Context::TypeId, |p| {
                p.consume()?;
                expect_token!(p, ParenOpen)?;

                let expr = p.parse_expr()?;
                let last = expect_token!(p, ParenClose)?;

                Ok(Node {
                    node: ty::TypeOf { expr: expr.into() }.into(),
                    span: span![token.span.begin, last.span.end],
                    attrs: vec![],
                })
            })?,
            _ => {
                return self.error(ErrorKind::UnexpectedToken {
                    exp: vec![].into(),
                    got: token,
                });
            }
        };

        loop {
            let first = lhs.span;
            lhs = match self.peek().map(|t| t.kind) {
                Ok(Question) => {
                    let tok = self.consume()?;
                    if matches!(lhs.node, Ty::Optional(_)) {
                        return self.error(ErrorKind::MultiplePostfixOptional { tok });
                    }

                    Node {
                        node: ty::Optional { ty: lhs.into() }.into(),
                        span: span![first.begin, token.span.end],
                        attrs: vec![],
                    }
                }
                Ok(OpPipe) => {
                    self.consume()?;
                    let rhs = self.parse_type(AllowEffect::No)?;
                    let last = rhs.span;
                    Node {
                        node: ty::Union {
                            lhs: lhs.into(),
                            rhs: rhs.into(),
                        }
                        .into(),
                        span: span![first.begin, last.end],
                        attrs: vec![],
                    }
                }
                Ok(KwRaise) => {
                    let tok = self.consume()?;
                    if !bool::from(allow_effect) {
                        return self.error(ErrorKind::DisallowedEffect { tok });
                    }

                    let rhs = self.parse_type(AllowEffect::No)?;
                    let last = rhs.span;
                    Node {
                        node: ty::Effect {
                            lhs: lhs.into(),
                            rhs: rhs.into(),
                        }
                        .into(),
                        span: span![first.begin, last.end],
                        attrs: vec![],
                    }
                }
                _ => break Ok(lhs),
            };
        }
    }

    pub(crate) fn parse_return_ty(&mut self) -> Result<Node<Ty>> {
        self.parse_type(AllowEffect::Yes)
    }

    pub(crate) fn parse_param_ty(&mut self) -> Result<Node<Ty>> {
        self.parse_type(AllowEffect::No)
    }

    pub(crate) fn parse_cast_ty(&mut self) -> Result<Node<Ty>> {
        self.parse_type(AllowEffect::No)
    }
}
