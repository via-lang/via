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
use viac_ast::expr::Expr;
use viac_ast::place;
use viac_ast::value;

impl<'a> Parser<'a> {
    pub(super) fn is_expr_start(&self) -> bool {
        matches!(
            self.peek().map(|t| t.kind),
            Ok(Identifier
                | KwTrue
                | KwFalse
                | KwNone
                | KwFn
                | KwSelf
                | LitInt
                | LitBint
                | LitXint
                | LitFloat
                | LitString
                | OpMinus
                | OpAmp
                | OpTilde
                | OpBang
                | ParenOpen
                | BraceOpen
                | BracketOpen)
        )
    }

    fn parse_expr_primary(&mut self, allow_prefix: bool) -> Result<Node<Expr>> {
        self.with_context(Context::ExprPrimary, |p| {
            let token = p.peek()?;
            match token.kind {
                Identifier => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Place(place::Symbol { token }.into()),
                        span: token.span,
                    })
                }
                KwSelf => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Place(place::This {}.into()),
                        span: token.span,
                    })
                }
                KwNone => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::None {}.into()),
                        span: token.span,
                    })
                }
                KwTrue => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::True {}.into()),
                        span: token.span,
                    })
                }
                KwFalse => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::False {}.into()),
                        span: token.span,
                    })
                }
                LitInt | LitXint | LitBint => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::Integer { token }.into()),
                        span: token.span,
                    })
                }
                LitFloat => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::Float { token }.into()),
                        span: token.span,
                    })
                }
                LitString => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::String { token }.into()),
                        span: token.span,
                    })
                }
                ParenOpen => {
                    let first = p.consume()?;
                    let inner = p.parse_expr()?;
                    let first_elem = inner.span;

                    let expr = if p.check(Comma) {
                        p.push_context(Context::ExprTuple);
                        let mut exprs = vec![inner];

                        while p.optional(Comma) {
                            if p.check(ParenClose) {
                                break;
                            }
                            let next = p.parse_expr()?;
                            exprs.push(next);
                        }

                        let last = p.expect_consume(ParenClose)?;
                        let last_elem = exprs.last().expect("parsed empty tuple").span;

                        Node {
                            node: Expr::Value(
                                value::Tuple {
                                    exprs: NodeList {
                                        list: exprs,
                                        span: span![first_elem.begin, last_elem.end],
                                    },
                                }
                                .into(),
                            ),
                            span: span![first.span.begin, last.span.end],
                        }
                    } else {
                        p.push_context(Context::ExprGroup);
                        let last = p.expect_consume(ParenClose)?;
                        Node {
                            node: inner.node,
                            span: span![first.span.begin, last.span.end],
                        }
                    };

                    p.pop_context();
                    Ok(expr)
                }
                BracketOpen => p.with_context(Context::ExprArray, |p| {
                    let exprs = p.parse_list((BracketOpen, BracketClose), Self::parse_expr)?;
                    let span = exprs.span;

                    Ok(Node {
                        node: Expr::Value(value::Array { exprs }.into()),
                        span,
                    })
                }),
                BraceOpen => p.with_context(Context::ExprMap, |p| {
                    let first = p.consume()?;
                    let mut pairs = vec![];

                    while !p.check(BraceClose) {
                        let key = p.parse_expr()?;
                        p.expect_consume(Colon)?;
                        let value = p.parse_expr()?;
                        pairs.push((key, value));

                        if !p.optional(Comma) {
                            break;
                        }
                    }

                    let last = p.expect_consume(BraceClose)?;
                    Ok(Node {
                        node: Expr::Value(value::Map { pairs }.into()),
                        span: span![first.span.begin, last.span.end],
                    })
                }),
                OpAmp if allow_prefix => {
                    p.consume()?;
                    let strong = p.optional(OpQuote);
                    let mutable = p.optional(KwMut);
                    let expr = p.parse_expr()?;
                    let last = expr.span;

                    Ok(Node {
                        node: Expr::Value(
                            value::Reference {
                                strong,
                                mutable,
                                expr: expr.into(),
                            }
                            .into(),
                        ),
                        span: span![token.span.begin, last.end],
                    })
                }
                OpMinus | OpBang | OpTilde if allow_prefix => {
                    p.consume()?;
                    let inner = p.parse_expr_primary(false)?;
                    let last = inner.span;
                    Ok(Node {
                        node: Expr::Value(
                            value::Unary {
                                op: token,
                                expr: inner.into(),
                            }
                            .into(),
                        ),
                        span: span![token.span.begin, last.end],
                    })
                }
                KwFn => p.with_context(Context::ExprLambda, |p| {
                    p.consume()?;
                    let params = p
                        .check(ParenOpen)
                        .then(|| {
                            Ok(p.parse_list((ParenOpen, ParenClose), |p| {
                                let name = p.expect_consume(Identifier)?;
                                p.expect_consume(Colon)?;
                                let ty = p.parse_type()?;
                                let last = ty.span;

                                Ok(Node {
                                    node: Param {
                                        name,
                                        ty: ty.into(),
                                    },
                                    span: span![name.span.begin, last.end],
                                })
                            })?)
                        })
                        .transpose()?
                        .unwrap_or(NodeList {
                            list: vec![],
                            span: token.span,
                        });

                    let result = p.check(Arrow).then(|| p.parse_return_ty()).transpose()?;
                    let body = p.parse_body(Self::parse_stmt)?;
                    let last = body.span;

                    Ok(Node {
                        node: Expr::Value(
                            value::Lambda {
                                params,
                                result: result.map(Into::into),
                                body,
                            }
                            .into(),
                        ),
                        span: span![token.span.begin, last.end],
                    })
                }),
                OpHash => {
                    let attr = p.parse_attr()?;
                    let span = attr.span;
                    Ok(Node {
                        node: Expr::Value(value::Attr { attr: attr.into() }.into()),
                        span,
                    })
                }
                _ => p.error(ErrorKind::UnexpectedToken {
                    expected: vec![Identifier],
                    got: token,
                }),
            }
        })
    }

    fn parse_expr_postfix(&mut self) -> Result<Node<Expr>> {
        let mut expr = self.parse_expr_primary(true)?;
        loop {
            if let Ok(token) = self.peek() {
                match token.kind {
                    Period => {
                        self.consume()?;
                        let field = self.expect_consume(Identifier)?;
                        let first = expr.span;

                        expr = Node {
                            node: Expr::Place(
                                place::Dynamic {
                                    expr: expr.into(),
                                    field,
                                }
                                .into(),
                            ),
                            span: span![first.begin, field.span.end],
                        };
                    }
                    ColonColon => {
                        self.consume()?;
                        let field = self.expect_consume(Identifier)?;
                        let first = expr.span;

                        expr = Node {
                            node: Expr::Place(
                                place::Static {
                                    expr: expr.into(),
                                    field,
                                }
                                .into(),
                            ),
                            span: span![first.begin, field.span.end],
                        };
                    }
                    BracketOpen => {
                        self.consume()?;
                        let index = self.parse_expr()?;
                        let first = expr.span;
                        let last = self.expect_consume(BracketClose)?;

                        expr = Node {
                            node: Expr::Place(
                                place::Subscript {
                                    expr: expr.into(),
                                    index: index.into(),
                                }
                                .into(),
                            ),
                            span: span![first.begin, last.span.end],
                        };
                    }
                    OpDotDot => {
                        self.consume()?;
                        let inclusive = self.optional(OpEq);
                        let end = self.parse_expr()?;
                        let first = expr.span;
                        let last = end.span;

                        expr = Node {
                            node: Expr::Value(
                                value::Range {
                                    lhs: expr.into(),
                                    rhs: end.into(),
                                    inclusive,
                                }
                                .into(),
                            ),
                            span: span![first.begin, last.end],
                        }
                    }
                    KwIf => {
                        self.consume()?;

                        let first = expr.span;
                        let cond = self.parse_expr()?;
                        self.expect_consume(KwElse)?;

                        let alt = self.parse_expr()?;
                        let last = alt.span;

                        expr = Node {
                            node: Expr::Value(
                                value::Ternary {
                                    cond: cond.into(),
                                    iftrue: expr.into(),
                                    iffalse: alt.into(),
                                }
                                .into(),
                            ),
                            span: span![first.begin, last.end],
                        }
                    }
                    KwAs => {
                        self.consume()?;

                        let first = expr.span;
                        let ty = self.parse_cast_ty()?;
                        let last = ty.span;

                        expr = Node {
                            node: Expr::Value(
                                value::Cast {
                                    expr: expr.into(),
                                    ty: ty.into(),
                                }
                                .into(),
                            ),
                            span: span![first.begin, last.end],
                        }
                    }
                    _ => {}
                }
            }
            break;
        }
        Ok(expr)
    }

    fn parse_expr_binary(&mut self, min_prec: u8) -> Result<Node<Expr>> {
        let mut lhs = self.parse_expr_postfix()?;
        loop {
            let op = match self.peek() {
                Ok(token) => token,
                _ => break,
            };

            let prec = match op.kind.bin_prec() {
                Some(prec) if prec >= min_prec => prec,
                _ => break,
            };

            self.consume()?;
            let rhs = self.parse_expr_binary(prec + 1)?;
            let first = lhs.span;
            let last = rhs.span;

            lhs = Node {
                node: Expr::Value(
                    value::Binary {
                        op,
                        lhs: lhs.into(),
                        rhs: rhs.into(),
                    }
                    .into(),
                ),
                span: span![first.begin, last.end],
            };
        }
        Ok(lhs)
    }

    pub(crate) fn parse_expr(&mut self) -> Result<Node<Expr>> {
        self.parse_expr_binary(0)
    }
}
