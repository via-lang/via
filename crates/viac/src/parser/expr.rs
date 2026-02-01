/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::{expr::Expr, node::Nodes, place, value};

yes_or_no!(AllowPrefix);

impl Parser<'_> {
    pub(super) fn is_expr_start(&self) -> bool {
        matches!(
            self.peek().map(|t| t.kind),
            Ok(
                Ident
                | KwTrue
                | KwFalse
                | KwNone
                | KwFn
                | KwSelf
                | Int { base: _ }
                | Float
                | String { terminated: _ }
                | Minus
                | Amp // unary
                | Tilde // unary
                | Bang // unary
                | Hash // attribute
                | LParen // group or tuple
                | LBrace // map
                | LBracket // array
            )
        )
    }

    fn parse_expr_primary(&mut self, allow_prefix: AllowPrefix) -> Result<Node<Expr>> {
        self.with_context(Context::ExprPrimary, |parser| {
            let token = parser.peek()?;
            match token.kind {
                Ident => {
                    parser.consume()?;
                    let span = token.span.clone();
                    Ok(Node {
                        node: Expr::Place(place::Symbol { token }.into()),
                        span,
                        attrs: None,
                    })
                }
                KwSelf => {
                    parser.consume()?;
                    Ok(Node {
                        node: Expr::Place(place::This {}.into()),
                        span: token.span,
                        attrs: None,
                    })
                }
                KwNone => {
                    parser.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::None {}.into()),
                        span: token.span,
                        attrs: None,
                    })
                }
                KwTrue => {
                    parser.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::True {}.into()),
                        span: token.span,
                        attrs: None,
                    })
                }
                KwFalse => {
                    parser.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::False {}.into()),
                        span: token.span,
                        attrs: None,
                    })
                }
                Int { base: _ } => {
                    parser.consume()?;
                    let span = token.span.clone();
                    Ok(Node {
                        node: Expr::Value(value::Integer { token }.into()),
                        span,
                        attrs: None,
                    })
                }
                Float => {
                    parser.consume()?;
                    let span = token.span.clone();
                    Ok(Node {
                        node: Expr::Value(value::Float { token }.into()),
                        span,
                        attrs: None,
                    })
                }
                String { terminated } => {
                    if !terminated {
                        return Err(Error::UnterminatedStringLiteral {
                            src: parser.src.clone(),
                            string: token.span.to_miette_span(),
                            quote: miette::SourceSpan::new((token.span.end - 1).into(), 1),
                        });
                    }
                    parser.consume()?;
                    let span = token.span.clone();
                    Ok(Node {
                        node: Expr::Value(value::String { token }.into()),
                        span,
                        attrs: None,
                    })
                }
                LParen => {
                    let first = parser.consume()?;
                    let inner = parser.parse_expr()?;
                    let first_elem = inner.span.clone();

                    let expr = if check!(parser, Comma) {
                        parser.push_context(Context::ExprTuple);
                        let mut exprs = vec![inner];

                        while optional!(parser, Comma) {
                            if check!(parser, RParen) {
                                break;
                            }
                            let next = parser.parse_expr()?;
                            exprs.push(next);
                        }

                        let last = expect_one!(parser, RParen)?;
                        let last_elem = exprs
                            .last()
                            .expect("somehow parsed empty tuple?")
                            .span
                            .clone();

                        Node {
                            node: Expr::Value(
                                value::Tuple {
                                    exprs: Nodes {
                                        nodes: exprs,
                                        span: SourceSpan::new(first_elem.begin, last_elem.end),
                                    },
                                }
                                .into(),
                            ),
                            span: SourceSpan::new(first.span.begin, last.span.end),
                            attrs: None,
                        }
                    } else {
                        parser.push_context(Context::ExprGroup);
                        let last = expect_one!(parser, RParen)?;
                        Node {
                            node: inner.node,
                            span: SourceSpan::new(first.span.begin, last.span.end),
                            attrs: None,
                        }
                    };

                    parser.pop_context();
                    Ok(expr)
                }
                LBracket => parser.with_context(Context::ExprArray, |parser| {
                    let exprs = parser.parse_list((LBracket, RBracket), Self::parse_expr)?;
                    let span = exprs.span.clone();

                    Ok(Node {
                        node: Expr::Value(value::Array { exprs }.into()),
                        span,
                        attrs: None,
                    })
                }),
                LBrace => parser.with_context(Context::ExprMap, |parser| {
                    let first = parser.consume()?;
                    let mut pairs = vec![];

                    while !check!(parser, RBrace) {
                        let key = parser.parse_expr()?;
                        expect_one!(parser, Col)?;
                        let value = parser.parse_expr()?;
                        pairs.push((key, value));

                        if !optional!(parser, Comma) {
                            break;
                        }
                    }

                    let last = expect_one!(parser, RBrace)?;
                    Ok(Node {
                        node: Expr::Value(value::Map { pairs }.into()),
                        span: SourceSpan::new(first.span.begin, last.span.end),
                        attrs: None,
                    })
                }),
                Amp if allow_prefix.into() => {
                    parser.consume()?;
                    let expr = parser.parse_expr()?;
                    let last = expr.span.clone();
                    Ok(Node {
                        node: Expr::Value(value::Reference { expr: expr.into() }.into()),
                        span: SourceSpan::new(token.span.begin, last.end),
                        attrs: None,
                    })
                }
                Minus | Bang | Tilde if allow_prefix.into() => {
                    parser.consume()?;

                    let inner = parser.parse_expr_primary(AllowPrefix::No)?;
                    let first = token.span.clone();
                    let last = inner.span.clone();

                    Ok(Node {
                        node: Expr::Value(
                            value::Unary {
                                op: token,
                                expr: inner.into(),
                            }
                            .into(),
                        ),
                        span: SourceSpan::new(first.begin, last.end),
                        attrs: None,
                    })
                }
                KwFn => {
                    parser.consume()?;
                    parser.push_context(Context::ExprLambda);

                    let params = check!(parser, LParen)
                        .then(|| parser.parse_list((LParen, RParen), Self::parse_param))
                        .transpose()?
                        .unwrap_or(Nodes {
                            nodes: vec![],
                            span: token.span.clone(),
                        });

                    let result = optional!(parser, Arrow)
                        .then(|| parser.parse_return_ty())
                        .transpose()?
                        .map(Into::into);

                    parser.pop_context();

                    let body = parser.parse_body(Self::parse_stmt)?;
                    let last = body.span.clone();

                    Ok(Node {
                        node: Expr::Value(
                            value::Lambda {
                                params,
                                result,
                                body,
                            }
                            .into(),
                        ),
                        span: SourceSpan::new(token.span.begin, last.end),
                        attrs: None,
                    })
                }
                Hash => {
                    let attr = parser.parse_attr()?;
                    let span = attr.span.clone();
                    Ok(Node {
                        node: Expr::Value(value::Attr { attr: attr.into() }.into()),
                        span,
                        attrs: None,
                    })
                }
                _ => Err(Error::UnexpectedToken {
                    src: parser.src.clone(),
                    span: token.span.to_miette_span(),
                    expected: vec![].into(),
                    got: parser.src.get_span(token.span).to_owned(),
                }),
            }
        })
    }

    fn parse_expr_postfix(&mut self) -> Result<Node<Expr>> {
        let mut expr = self.parse_expr_primary(AllowPrefix::Yes)?;
        loop {
            expr = match self.peek().map(|t| t.kind) {
                Ok(Dot) if matches!(self.peek_ahead(1).map(|t| t.kind), Ok(KwAwait)) => {
                    self.consume()?;
                    let tok = self.consume()?;
                    let first = expr.span.clone();
                    Node {
                        node: Expr::Value(value::Await { expr: expr.into() }.into()),
                        span: SourceSpan::new(first.begin, tok.span.end),
                        attrs: None,
                    }
                }
                Ok(Dot) => {
                    self.consume()?;
                    let field = expect_one!(self, Ident)?;
                    let first = expr.span.clone();
                    let last = field.span.clone();

                    Node {
                        node: Expr::Place(
                            place::Dynamic {
                                expr: expr.into(),
                                field,
                            }
                            .into(),
                        ),
                        span: SourceSpan::new(first.begin, last.end),
                        attrs: None,
                    }
                }
                Ok(ColCol) => {
                    self.consume()?;
                    let field = expect_one!(self, Ident)?;
                    let first = expr.span.clone();
                    let last = field.span.clone();

                    Node {
                        node: Expr::Place(
                            place::Static {
                                expr: expr.into(),
                                field,
                            }
                            .into(),
                        ),
                        span: SourceSpan::new(first.begin, last.end),
                        attrs: None,
                    }
                }
                Ok(LBracket) => {
                    self.consume()?;
                    let index = self.parse_expr()?;
                    let first = expr.span.clone();
                    let last = expect_one!(self, RBracket)?;

                    Node {
                        node: Expr::Place(
                            place::Subscript {
                                expr: expr.into(),
                                index: index.into(),
                            }
                            .into(),
                        ),
                        span: SourceSpan::new(first.begin, last.span.end),
                        attrs: None,
                    }
                }
                Ok(LParen) => {
                    let args = self.parse_list((LParen, RParen), Self::parse_expr)?;
                    let first = expr.span.clone();
                    let last = args.span.clone();

                    Node {
                        node: Expr::Value(
                            value::Call {
                                callee: expr.into(),
                                args,
                            }
                            .into(),
                        ),
                        span: SourceSpan::new(first.begin, last.end),
                        attrs: None,
                    }
                }
                Ok(DotDot) => {
                    self.consume()?;
                    let inclusive = optional!(self, Eq);
                    let end = self.parse_expr()?;
                    let first = expr.span.clone();
                    let last = end.span.clone();

                    Node {
                        node: Expr::Value(
                            value::Range {
                                lhs: expr.into(),
                                rhs: end.into(),
                                inclusive,
                            }
                            .into(),
                        ),
                        span: SourceSpan::new(first.begin, last.end),
                        attrs: None,
                    }
                }
                Ok(KwIf) => {
                    self.consume()?;

                    let first = expr.span.clone();
                    let cond = self.parse_expr()?;
                    expect_one!(self, KwElse)?;

                    let alt = self.parse_expr()?;
                    let last = alt.span.clone();

                    Node {
                        node: Expr::Value(
                            value::Ternary {
                                cond: cond.into(),
                                iftrue: expr.into(),
                                iffalse: alt.into(),
                            }
                            .into(),
                        ),
                        span: SourceSpan::new(first.begin, last.end),
                        attrs: None,
                    }
                }
                Ok(KwAs) => {
                    self.consume()?;

                    let ty = self.parse_cast_ty()?;
                    let first = expr.span.clone();
                    let last = ty.span.clone();

                    Node {
                        node: Expr::Value(
                            value::Cast {
                                expr: expr.into(),
                                ty: ty.into(),
                            }
                            .into(),
                        ),
                        span: SourceSpan::new(first.begin, last.end),
                        attrs: None,
                    }
                }
                Ok(Quest) => {
                    let tok = self.consume()?;
                    let first = expr.span.clone();
                    Node {
                        node: Expr::Value(value::Try { expr: expr.into() }.into()),
                        span: SourceSpan::new(first.begin, tok.span.end),
                        attrs: None,
                    }
                }
                _ => break Ok(expr),
            };
        }
    }

    fn parse_expr_binary(&mut self, min_prec: u8) -> Result<Node<Expr>> {
        let mut lhs = self.parse_expr_postfix()?;
        while let Ok(op) = self.peek() {
            let prec = match op.kind.prec() {
                Some(prec) if prec >= min_prec => prec,
                _ => break,
            };

            self.consume()?;
            let rhs = self.parse_expr_binary(prec + 1)?;
            let first = lhs.span.clone();
            let last = rhs.span.clone();

            lhs = Node {
                node: Expr::Value(
                    value::Binary {
                        op,
                        lhs: lhs.into(),
                        rhs: rhs.into(),
                    }
                    .into(),
                ),
                span: SourceSpan::new(first.begin, last.end),
                attrs: None,
            };
        }
        Ok(lhs)
    }

    pub(crate) fn parse_expr(&mut self) -> Result<Node<Expr>> {
        self.parse_expr_binary(0)
    }
}
