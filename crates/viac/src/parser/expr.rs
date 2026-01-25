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
use crate::ast::expr::Expr;
use crate::ast::place;
use crate::ast::value;

yes_or_no!(AllowPrefix);

impl Parser {
    pub(super) fn is_expr_start(&self) -> bool {
        matches!(
            self.peek().map(|t| t.kind),
            Ok(
                Identifier
                | KwTrue
                | KwFalse
                | KwNone
                | KwFn
                | KwSelf
                | LitInt { base: _ }
                | LitFloat
                | LitString { terminated: _ }
                | OpMinus
                | OpAmp // unary
                | OpTilde // unary
                | OpBang // unary
                | OpHash // attribute
                | ParenOpen // group or tuple
                | BraceOpen // map
                | BracketOpen // array
            )
        )
    }

    fn parse_expr_primary(&mut self, allow_prefix: AllowPrefix) -> Result<Node<Expr>> {
        self.with_context(Context::ExprPrimary, |p| {
            let token = p.peek()?;
            match token.kind {
                Identifier => {
                    p.consume()?;
                    let span = token.span;
                    Ok(Node {
                        node: Expr::Place(place::Symbol { token }.into()),
                        span,
                        attrs: vec![],
                    })
                }
                KwSelf => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Place(place::This {}.into()),
                        span: token.span,
                        attrs: vec![],
                    })
                }
                KwNone => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::None {}.into()),
                        span: token.span,
                        attrs: vec![],
                    })
                }
                KwTrue => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::True {}.into()),
                        span: token.span,
                        attrs: vec![],
                    })
                }
                KwFalse => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::False {}.into()),
                        span: token.span,
                        attrs: vec![],
                    })
                }
                LitInt { base: _ } => {
                    p.consume()?;
                    let span = token.span;
                    Ok(Node {
                        node: Expr::Value(value::Integer { token }.into()),
                        span,
                        attrs: vec![],
                    })
                }
                LitFloat => {
                    p.consume()?;
                    let span = token.span;
                    Ok(Node {
                        node: Expr::Value(value::Float { token }.into()),
                        span,
                        attrs: vec![],
                    })
                }
                LitString { terminated } => {
                    if !terminated {
                        return p.error(ErrorKind::UnterminatedStringLiteral { tok: token });
                    }
                    p.consume()?;
                    let span = token.span;
                    Ok(Node {
                        node: Expr::Value(value::String { token }.into()),
                        span,
                        attrs: vec![],
                    })
                }
                ParenOpen => {
                    let first = p.consume()?;
                    let inner = p.parse_expr()?;
                    let first_elem = inner.span;

                    let expr = if check_token!(p, Comma) {
                        p.push_context(Context::ExprTuple);
                        let mut exprs = vec![inner];

                        while optional_token!(p, Comma) {
                            if check_token!(p, ParenClose) {
                                break;
                            }
                            let next = p.parse_expr()?;
                            exprs.push(next);
                        }

                        let last = expect_token!(p, ParenClose)?;
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
                            attrs: vec![],
                        }
                    } else {
                        p.push_context(Context::ExprGroup);
                        let last = expect_token!(p, ParenClose)?;
                        Node {
                            node: inner.node,
                            span: span![first.span.begin, last.span.end],
                            attrs: vec![],
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
                        attrs: vec![],
                    })
                }),
                BraceOpen => p.with_context(Context::ExprMap, |p| {
                    let first = p.consume()?;
                    let mut pairs = vec![];

                    while !check_token!(p, BraceClose) {
                        let key = p.parse_expr()?;
                        expect_token!(p, Colon)?;
                        let value = p.parse_expr()?;
                        pairs.push((key, value));

                        if !optional_token!(p, Comma) {
                            break;
                        }
                    }

                    let last = expect_token!(p, BraceClose)?;
                    Ok(Node {
                        node: Expr::Value(value::Map { pairs }.into()),
                        span: span![first.span.begin, last.span.end],
                        attrs: vec![],
                    })
                }),
                OpAmp if allow_prefix.into() => {
                    p.consume()?;
                    let expr = p.parse_expr()?;
                    let last = expr.span;

                    Ok(Node {
                        node: Expr::Value(value::Reference { expr: expr.into() }.into()),
                        span: span![token.span.begin, last.end],
                        attrs: vec![],
                    })
                }
                OpMinus | OpBang | OpTilde if allow_prefix.into() => {
                    p.consume()?;

                    let inner = p.parse_expr_primary(AllowPrefix::No)?;
                    let first = token.span;
                    let last = inner.span;

                    Ok(Node {
                        node: Expr::Value(
                            value::Unary {
                                op: token,
                                expr: inner.into(),
                            }
                            .into(),
                        ),
                        span: span![first.begin, last.end],
                        attrs: vec![],
                    })
                }
                KwFn => {
                    p.consume()?;
                    p.push_context(Context::ExprLambda);

                    let params = check_token!(p, OpPipe)
                        .then(|| Ok(p.parse_list((OpPipe, OpPipe), Self::parse_param)?))
                        .transpose()?
                        .unwrap_or(NodeList {
                            list: vec![],
                            span: token.span,
                        });

                    let result = p.parse_return_ty()?;
                    p.pop_context();

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
                        attrs: vec![],
                    })
                }
                OpHash => {
                    let attr = p.parse_attr()?;
                    let span = attr.span;
                    Ok(Node {
                        node: Expr::Value(value::Attr { attr: attr.into() }.into()),
                        span,
                        attrs: vec![],
                    })
                }
                _ => p.error(ErrorKind::UnexpectedToken {
                    exp: vec![].into(),
                    got: token,
                }),
            }
        })
    }

    fn parse_expr_postfix(&mut self) -> Result<Node<Expr>> {
        let mut expr = self.parse_expr_primary(AllowPrefix::Yes)?;
        loop {
            if let Ok(token) = self.peek() {
                match token.kind {
                    Period => {
                        self.consume()?;
                        let field = expect_token!(self, Identifier)?;
                        let first = expr.span;
                        let last = field.span;

                        expr = Node {
                            node: Expr::Place(
                                place::Dynamic {
                                    expr: expr.into(),
                                    field,
                                }
                                .into(),
                            ),
                            span: span![first.begin, last.end],
                            attrs: vec![],
                        };
                    }
                    ColonColon => {
                        self.consume()?;
                        let field = expect_token!(self, Identifier)?;
                        let first = expr.span;
                        let last = field.span;

                        expr = Node {
                            node: Expr::Place(
                                place::Static {
                                    expr: expr.into(),
                                    field,
                                }
                                .into(),
                            ),
                            span: span![first.begin, last.end],
                            attrs: vec![],
                        };
                    }
                    BracketOpen => {
                        self.consume()?;
                        let index = self.parse_expr()?;
                        let first = expr.span;
                        let last = expect_token!(self, BracketClose)?;

                        expr = Node {
                            node: Expr::Place(
                                place::Subscript {
                                    expr: expr.into(),
                                    index: index.into(),
                                }
                                .into(),
                            ),
                            span: span![first.begin, last.span.end],
                            attrs: vec![],
                        };
                    }
                    OpDotDot => {
                        self.consume()?;
                        let inclusive = optional_token!(self, OpEq);
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
                            attrs: vec![],
                        }
                    }
                    KwIf => {
                        self.consume()?;

                        let first = expr.span;
                        let cond = self.parse_expr()?;
                        expect_token!(self, KwElse)?;

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
                            attrs: vec![],
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
                            attrs: vec![],
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

            let prec = match op.kind.prec() {
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
                attrs: vec![],
            };
        }
        Ok(lhs)
    }

    pub(crate) fn parse_expr(&mut self) -> Result<Node<Expr>> {
        self.parse_expr_binary(0)
    }
}
