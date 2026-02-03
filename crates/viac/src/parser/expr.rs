/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::prelude::*;
use crate::ast::{Tree, aux::Nodes, expr::Expr, place, value};

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

    fn parse_expr_primary(&mut self, tree: &mut Tree, allow_prefix: AllowPrefix) -> Result<Expr> {
        self.with_context(Context::ExprPrimary, |parser| {
            let token = parser.peek()?;
            let span = token.span.clone();

            match token.kind {
                Ident => {
                    parser.consume()?;
                    Ok(Expr::Place(
                        place::Symbol {
                            span: span.clone(),
                            symbol: parser.src.get_span(span).to_owned(),
                        }
                        .into(),
                    ))
                }
                KwSelf => {
                    parser.consume()?;
                    Ok(Expr::Place(place::This { span }.into()))
                }
                KwNone => {
                    parser.consume()?;
                    Ok(Expr::Value(value::None { span }.into()))
                }
                KwTrue => {
                    parser.consume()?;
                    Ok(Expr::Value(value::True { span }.into()))
                }
                KwFalse => {
                    parser.consume()?;
                    Ok(Expr::Value(value::False { span }.into()))
                }
                Int { base: _ } => {
                    parser.consume()?;
                    let span = token.span.clone();
                    Ok(Expr::Value(
                        value::Integer {
                            span: span.clone(),
                            value: parser
                                .src
                                .get_span(span.clone())
                                .parse::<i64>()
                                .expect("lexically valid integer literal must be parsable"),
                        }
                        .into(),
                    ))
                }
                Float => {
                    parser.consume()?;
                    let span = token.span.clone();
                    Ok(Expr::Value(
                        value::Float {
                            span: span.clone(),
                            value: parser
                                .src
                                .get_span(span.clone())
                                .parse::<f64>()
                                .expect("lexically valid float literal must be parsable"),
                        }
                        .into(),
                    ))
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
                    Ok(Expr::Value(
                        value::String {
                            span: span.clone(),
                            string: parser.src.get_span(span).to_owned(),
                        }
                        .into(),
                    ))
                }
                LParen => {
                    parser.consume()?;

                    let inner = parser.parse_expr(tree)?;
                    let first_elem = inner.span();

                    let expr = if check!(parser, Comma) {
                        parser.push_context(Context::ExprTuple);
                        let mut exprs = vec![inner];

                        while optional!(parser, Comma) {
                            if check!(parser, RParen) {
                                break;
                            }
                            let next = parser.parse_expr(tree)?;
                            exprs.push(next);
                        }

                        expect_one!(parser, RParen)?;

                        let last_elem = exprs.last().expect("somehow parsed empty tuple?").span();

                        Expr::Value(
                            value::Tuple {
                                span: SourceSpan::new(first_elem.begin, last_elem.end),
                                exprs: exprs.iter().map(|e| tree.insert(e.clone())).collect(),
                            }
                            .into(),
                        )
                    } else {
                        parser.push_context(Context::ExprGroup);
                        expect_one!(parser, RParen)?;
                        inner
                    };

                    parser.pop_context();
                    Ok(expr)
                }
                LBracket => parser.with_context(Context::ExprArray, |parser| {
                    let exprs = parser.parse_list(tree, (LBracket, RBracket), Self::parse_expr)?;
                    let span = exprs.span.clone();

                    Ok(Expr::Value(
                        value::Array {
                            span,
                            exprs: exprs.inner,
                        }
                        .into(),
                    ))
                }),
                LBrace => parser.with_context(Context::ExprMap, |parser| {
                    let first = parser.consume()?;
                    let mut pairs = vec![];

                    while !check!(parser, RBrace) {
                        let key = parser.parse_expr(tree)?;

                        expect_one!(parser, Col)?;

                        let value = parser.parse_expr(tree)?;
                        pairs.push((key, value));

                        if !optional!(parser, Comma) {
                            break;
                        }
                    }

                    let last = expect_one!(parser, RBrace)?;
                    Ok(Expr::Value(
                        value::Map {
                            span: SourceSpan::new(first.span.begin, last.span.end),
                            pairs: pairs
                                .iter()
                                .map(|(k, v)| (tree.insert(k.clone()), tree.insert(v.clone())))
                                .collect(),
                        }
                        .into(),
                    ))
                }),
                Amp if allow_prefix.into() => {
                    parser.consume()?;
                    let expr = parser.parse_expr(tree)?;
                    let last = expr.span();
                    Ok(Expr::Value(
                        value::Reference {
                            span: SourceSpan::new(token.span.begin, last.end),
                            expr: tree.insert(expr),
                        }
                        .into(),
                    ))
                }
                Minus | Bang | Tilde if allow_prefix.into() => {
                    parser.consume()?;

                    let expr = parser.parse_expr_primary(tree, AllowPrefix::No)?;
                    let first = token.span.clone();
                    let last = expr.span();

                    Ok(Expr::Value(
                        value::Unary {
                            span: SourceSpan::new(first.begin, last.end),
                            op: token,
                            expr: tree.insert(expr),
                        }
                        .into(),
                    ))
                }
                KwFn => {
                    parser.consume()?;
                    parser.push_context(Context::ExprLambda);

                    let params = check!(parser, LParen)
                        .then(|| parser.parse_list(tree, (LParen, RParen), Self::parse_param))
                        .transpose()?
                        .unwrap_or(Nodes {
                            inner: vec![],
                            span: token.span.clone(),
                        });

                    let result = optional!(parser, Arrow)
                        .then(|| parser.parse_return_ty(tree))
                        .transpose()?
                        .map(|t| tree.insert(t));

                    parser.pop_context();

                    let body = parser.parse_body(tree, Self::parse_stmt)?;
                    let last = body.span.clone();

                    Ok(Expr::Value(
                        value::Lambda {
                            span: SourceSpan::new(token.span.begin, last.end),
                            params: params.inner,
                            result,
                            body,
                        }
                        .into(),
                    ))
                }
                Hash => {
                    let attr = parser.parse_attr(tree)?;
                    let span = tree.get(attr).span();
                    Ok(Expr::Value(value::Attr { span, attr }.into()))
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

    fn parse_expr_postfix(&mut self, tree: &mut Tree) -> Result<Expr> {
        let mut expr = self.parse_expr_primary(tree, AllowPrefix::Yes)?;
        loop {
            expr = match self.peek().map(|t| t.kind) {
                Ok(Dot) if matches!(self.peek_ahead(1).map(|t| t.kind), Ok(KwAwait)) => {
                    self.consume()?;

                    let tok = self.consume()?;
                    let first = expr.span();

                    Expr::Value(
                        value::Await {
                            span: SourceSpan::new(first.begin, tok.span.end),
                            expr: tree.insert(expr),
                        }
                        .into(),
                    )
                }
                Ok(Dot) => {
                    self.consume()?;

                    let field = expect_one!(self, Ident)?;
                    let first = expr.span();
                    let last = field.span.clone();

                    Expr::Place(
                        place::Dynamic {
                            span: SourceSpan::new(first.begin, last.end),
                            expr: tree.insert(expr),
                            field,
                        }
                        .into(),
                    )
                }
                Ok(ColCol) => {
                    self.consume()?;

                    let field = expect_one!(self, Ident)?;
                    let first = expr.span();
                    let last = field.span.clone();

                    Expr::Place(
                        place::Static {
                            span: SourceSpan::new(first.begin, last.end),
                            expr: tree.insert(expr),
                            field,
                        }
                        .into(),
                    )
                }
                Ok(LBracket) => {
                    self.consume()?;

                    let index = self.parse_expr(tree)?;
                    let first = expr.span();
                    let last = expect_one!(self, RBracket)?;

                    Expr::Place(
                        place::Subscript {
                            span: SourceSpan::new(first.begin, last.span.end),
                            expr: tree.insert(expr),
                            index: tree.insert(index),
                        }
                        .into(),
                    )
                }
                Ok(LParen) => {
                    let args = self.parse_list(tree, (LParen, RParen), Self::parse_expr)?;
                    let first = expr.span();
                    let last = args.span.clone();

                    Expr::Value(
                        value::Call {
                            span: SourceSpan::new(first.begin, last.end),
                            callee: tree.insert(expr),
                            args: args.inner,
                        }
                        .into(),
                    )
                }
                Ok(DotDot) => {
                    self.consume()?;
                    let inclusive = optional!(self, Eq);
                    let end = self.parse_expr(tree)?;
                    let first = expr.span();
                    let last = end.span();

                    Expr::Value(
                        value::Range {
                            span: SourceSpan::new(first.begin, last.end),
                            lhs: tree.insert(expr),
                            rhs: tree.insert(end),
                            inclusive,
                        }
                        .into(),
                    )
                }
                Ok(KwIf) => {
                    self.consume()?;

                    let first = expr.span();
                    let cond = self.parse_expr(tree)?;

                    expect_one!(self, KwElse)?;

                    let alt = self.parse_expr(tree)?;
                    let last = alt.span();

                    Expr::Value(
                        value::Ternary {
                            span: SourceSpan::new(first.begin, last.end),
                            cond: tree.insert(cond),
                            iftrue: tree.insert(expr),
                            iffalse: tree.insert(alt),
                        }
                        .into(),
                    )
                }
                Ok(KwAs) => {
                    self.consume()?;

                    let ty = self.parse_cast_ty(tree)?;
                    let first = expr.span();
                    let last = ty.span();

                    Expr::Value(
                        value::Cast {
                            span: SourceSpan::new(first.begin, last.end),
                            expr: tree.insert(expr),
                            ty: tree.insert(ty),
                        }
                        .into(),
                    )
                }
                Ok(Quest) => {
                    let tok = self.consume()?;
                    let first = expr.span();

                    Expr::Value(
                        value::Try {
                            span: SourceSpan::new(first.begin, tok.span.end),
                            expr: tree.insert(expr),
                        }
                        .into(),
                    )
                }
                _ => break Ok(expr),
            };
        }
    }

    fn parse_expr_binary(&mut self, tree: &mut Tree, min_prec: u8) -> Result<Expr> {
        let mut lhs = self.parse_expr_postfix(tree)?;
        while let Ok(op) = self.peek() {
            let prec = match op.kind.prec() {
                Some(prec) if prec >= min_prec => prec,
                _ => break,
            };

            self.consume()?;
            let rhs = self.parse_expr_binary(tree, prec + 1)?;
            let first = lhs.span();
            let last = rhs.span();

            lhs = Expr::Value(
                value::Binary {
                    span: SourceSpan::new(first.begin, last.end),
                    op,
                    lhs: tree.insert(lhs),
                    rhs: tree.insert(rhs),
                }
                .into(),
            );
        }
        Ok(lhs)
    }

    pub(crate) fn parse_expr(&mut self, tree: &mut Tree) -> Result<Expr> {
        self.parse_expr_binary(tree, 0)
    }
}
