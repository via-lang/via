/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Parser;
use super::prelude::*;
use viac_ast::ty::{self, Ty};

impl<'a> Parser<'a> {
    pub(crate) fn parse_type(&mut self) -> Result<Node<Ty>> {
        let token = self.peek()?;
        let mut lhs = match token.kind {
            KwNone | KwBool | KwInt | KwFloat | KwString => Node {
                node: ty::Builtin {
                    token: self.consume()?,
                }
                .into(),
                span: token.span,
            },
            BracketOpen => {
                self.consume()?;
                let ty = self.parse_type()?;
                let last = self.expect_consume(BracketClose)?;

                Node {
                    node: ty::Array { ty: ty.into() }.into(),
                    span: span![token.span.begin, last.span.end],
                }
            }
            BraceOpen => self.with_context(Context::TypeMap, |p| {
                p.consume()?;
                let key = p.parse_type()?;

                p.expect_consume(Colon)?;

                let value = p.parse_type()?;
                let last = p.expect_consume(BraceClose)?;

                Ok(Node {
                    node: ty::Map {
                        key: key.into(),
                        value: value.into(),
                    }
                    .into(),
                    span: span![token.span.begin, last.span.end],
                })
            })?,
            KwFn => self.with_context(Context::TypeLambda, |p| {
                p.consume()?;
                let params = p.parse_list((ParenOpen, ParenClose), Self::parse_type)?;
                p.expect_consume(Arrow)?;
                let result = p.parse_type()?;
                let last = result.span;

                Ok(Node {
                    node: ty::Function {
                        params,
                        result: result.into(),
                    }
                    .into(),
                    span: span![token.span.begin, last.end],
                })
            })?,
            KwType => self.with_context(Context::TypeId, |p| {
                p.consume()?;
                p.expect_consume(ParenOpen)?;

                let expr = p.parse_expr()?;
                let last = p.expect_consume(ParenClose)?;

                Ok(Node {
                    node: ty::TypeOf { expr: expr.into() }.into(),
                    span: span![token.span.begin, last.span.end],
                })
            })?,
            _ => {
                return self.error(ErrorKind::UnexpectedToken {
                    expected: vec![],
                    got: token,
                });
            }
        };

        loop {
            let token = self.peek()?;
            let first = lhs.span;
            let postfix = match token.kind {
                Question => {
                    self.consume()?;
                    Node {
                        node: ty::Optional { ty: lhs.into() }.into(),
                        span: span![first.begin, token.span.end],
                    }
                }
                OpPipe => {
                    self.consume()?;
                    let rhs = self.parse_type()?;
                    let last = rhs.span;
                    Node {
                        node: ty::Union {
                            lhs: lhs.into(),
                            rhs: rhs.into(),
                        }
                        .into(),
                        span: span![first.begin, last.end],
                    }
                }
                _ => break Ok(lhs),
            };
            lhs = postfix;
        }
    }

    pub(crate) fn parse_return_ty(&mut self) -> Result<Node<Ty>> {
        self.with_context(Context::TypeRet, |p| {
            p.expect_consume(Arrow)?;
            p.parse_type()
        })
    }

    pub(crate) fn parse_param_ty(&mut self) -> Result<Node<Ty>> {
        self.parse_type()
    }

    pub(crate) fn parse_cast_ty(&mut self) -> Result<Node<Ty>> {
        self.parse_type()
    }
}
