/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::context::Context;
use crate::error::{Error, ErrorKind, Result};
use via_macros::bug;
use viac_ast::attr::{self, Attr};
use viac_ast::control::{self, Control};
use viac_ast::decl::{self, Decl};
use viac_ast::expr::Expr;
use viac_ast::extra::{NodeList, Param};
use viac_ast::node::{Ast, IntoNode, Node};
use viac_ast::place;
use viac_ast::stmt::Stmt;
use viac_ast::ty::{self, Ty};
use viac_ast::value;
use viac_lexer::token::{Token, TokenKind};
use viac_source::source::Source;
use viac_source::span;

pub struct Parser<'a> {
    source: &'a Source,
    tokens: &'a [Token],
    position: usize,
    contexts: Vec<Context>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a Source, tokens: &'a [Token]) -> Self {
        Self {
            source,
            tokens,
            position: 0,
            contexts: Vec::new(),
        }
    }

    fn push_context(&mut self, ctx: Context) {
        self.contexts.push(ctx);
    }

    fn pop_context(&mut self) {
        self.contexts.pop();
    }

    fn with_context<T>(&mut self, ctx: Context, f: impl FnOnce(&mut Self) -> T) -> T {
        self.push_context(ctx);
        let result = f(self);
        self.pop_context();
        result
    }

    fn error<T>(&self, kind: ErrorKind) -> Result<T> {
        Err(Error {
            kind,
            contexts: self.contexts.clone(),
        })
    }

    fn peek(&self) -> Result<Token> {
        self.tokens.get(self.position).cloned().ok_or_else(|| {
            self.error::<Token>(ErrorKind::UnexpectedEndOfFile)
                .err()
                .unwrap()
        })
    }

    fn peek_ahead(&self, ahead: u32) -> Result<Token> {
        self.tokens
            .get(self.position + ahead as usize)
            .cloned()
            .ok_or_else(|| {
                self.error::<Token>(ErrorKind::UnexpectedEndOfFile)
                    .err()
                    .unwrap()
            })
    }

    fn consume(&mut self) -> Result<Token> {
        self.peek().map(|token| {
            self.position += 1;
            token
        })
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().is_ok_and(|token| token.kind == kind)
    }

    fn check_ahead(&self, kind: TokenKind, ahead: u32) -> bool {
        self.peek_ahead(ahead).is_ok_and(|token| token.kind == kind)
    }

    #[allow(dead_code)]
    fn expect(&self, kind: TokenKind) -> Result<Token> {
        let token = self.peek()?;
        (token.kind == kind).then_some(token).ok_or_else(|| {
            self.error::<Token>(ErrorKind::UnexpectedToken {
                expected: vec![kind],
                got: token,
            })
            .err()
            .unwrap()
        })
    }

    fn expect_consume(&mut self, kind: TokenKind) -> Result<Token> {
        let token = self.consume()?;
        (token.kind == kind).then_some(token).ok_or_else(|| {
            self.error::<Token>(ErrorKind::UnexpectedToken {
                expected: vec![kind],
                got: token,
            })
            .err()
            .unwrap()
        })
    }

    fn optional(&mut self, kind: TokenKind) -> bool {
        self.check(kind)
            .then(|| self.consume().is_ok())
            .unwrap_or(false)
    }

    fn parse_body<F, T>(&mut self, parse: F) -> Result<NodeList<T>>
    where
        F: FnMut(&mut Self) -> Result<Node<T>>,
        T: Ast,
    {
        self.parse_list((TokenKind::BraceOpen, TokenKind::BraceClose), parse)
    }

    fn parse_list<F, T>(
        &mut self,
        brackets: (TokenKind, TokenKind),
        mut parse: F,
    ) -> Result<NodeList<T>>
    where
        F: FnMut(&mut Self) -> Result<Node<T>>,
        T: Ast,
    {
        let first = self.expect_consume(brackets.0)?;
        let mut body = vec![];

        loop {
            let node = parse(self)?;
            body.push(node);
            if self.check(TokenKind::Comma) {
                self.consume()?;
            } else {
                break;
            }
        }

        let last = self.expect_consume(brackets.1)?;
        Ok(NodeList {
            list: body,
            span: span![first.span.begin, last.span.end],
        })
    }

    fn parse_attr(&mut self) -> Result<Node<Attr>> {
        self.with_context(Context::Attr, |p| {
            let first = p.expect_consume(TokenKind::OpHash)?;
            let name = p.expect_consume(TokenKind::Identifier)?;
            let span = span![first.span.begin, name.span.end];

            match p.source.slice(name.span) {
                "native" => Ok(Node::new(attr::Native {}.into(), span)),
                "inline" => Ok(Node::new(attr::Inline {}.into(), span)),
                "distinct" => p.with_context(Context::AttrDistinct, |p| {
                    let first = p.expect_consume(TokenKind::ParenOpen)?;
                    let ty = p.parse_type()?;
                    let last = p.expect_consume(TokenKind::ParenClose)?;

                    Ok(Node {
                        node: attr::Distinct { ty: ty.into() }.into(),
                        span: span![first.span.begin, last.span.end],
                    })
                }),
                _ => p.error(ErrorKind::UnexpectedToken {
                    expected: vec![],
                    got: name,
                }),
            }
        })
    }

    fn is_expr_start(&self) -> bool {
        matches!(
            self.peek().map(|t| t.kind),
            Ok(TokenKind::Identifier
                | TokenKind::KwTrue
                | TokenKind::KwFalse
                | TokenKind::KwNone
                | TokenKind::KwFn
                | TokenKind::KwSelf
                | TokenKind::LitInt
                | TokenKind::LitBint
                | TokenKind::LitXint
                | TokenKind::LitFloat
                | TokenKind::LitString
                | TokenKind::OpMinus
                | TokenKind::OpAmp
                | TokenKind::OpTilde
                | TokenKind::OpBang
                | TokenKind::ParenOpen
                | TokenKind::BraceOpen
                | TokenKind::BracketOpen)
        )
    }

    fn parse_expr_primary(&mut self) -> Result<Node<Expr>> {
        self.with_context(Context::ExprPrimary, |p| {
            let token = p.peek()?;
            match token.kind {
                TokenKind::Identifier => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Place(place::Symbol { token }.into()),
                        span: token.span,
                    })
                }
                TokenKind::KwSelf => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Place(place::This {}.into()),
                        span: token.span,
                    })
                }
                TokenKind::KwNone => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::None {}.into()),
                        span: token.span,
                    })
                }
                TokenKind::KwTrue => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::True {}.into()),
                        span: token.span,
                    })
                }
                TokenKind::KwFalse => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::False {}.into()),
                        span: token.span,
                    })
                }
                TokenKind::LitInt | TokenKind::LitXint | TokenKind::LitBint => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::Integer { token }.into()),
                        span: token.span,
                    })
                }
                TokenKind::LitFloat => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::Float { token }.into()),
                        span: token.span,
                    })
                }
                TokenKind::LitString => {
                    p.consume()?;
                    Ok(Node {
                        node: Expr::Value(value::String { token }.into()),
                        span: token.span,
                    })
                }
                TokenKind::ParenOpen => {
                    let first = p.consume()?;
                    let inner = p.parse_expr()?;
                    let expr = if p.check(TokenKind::Comma) {
                        p.push_context(Context::ExprTuple);
                        let mut exprs = vec![inner];

                        while p.optional(TokenKind::Comma) {
                            if p.check(TokenKind::ParenClose) {
                                break;
                            }
                            let next = p.parse_expr()?;
                            exprs.push(next);
                        }

                        let last = p.expect_consume(TokenKind::ParenClose)?;

                        Node {
                            node: Expr::Value(value::Tuple { exprs }.into()),
                            span: span![first.span.begin, last.span.end],
                        }
                    } else {
                        p.push_context(Context::ExprGroup);
                        let last = p.expect_consume(TokenKind::ParenClose)?;
                        Node {
                            node: inner.node,
                            span: span![first.span.begin, last.span.end],
                        }
                    };

                    p.pop_context();
                    Ok(expr)
                }
                TokenKind::BracketOpen => {
                    let first = p.consume()?;
                    let mut exprs = Vec::new();

                    while !p.check(TokenKind::BracketClose) {
                        exprs.push(p.parse_expr()?);

                        if !p.optional(TokenKind::Comma) {
                            break;
                        }
                    }

                    let last = p.expect_consume(TokenKind::BracketClose)?;
                    Ok(Node {
                        node: Expr::Value(value::Array { exprs }.into()),
                        span: span![first.span.begin, last.span.end],
                    })
                }
                TokenKind::BraceOpen => {
                    let first = p.consume()?;
                    let mut pairs = vec![];

                    while !p.check(TokenKind::BraceClose) {
                        let key = p.parse_expr()?;
                        p.expect_consume(TokenKind::Colon)?;
                        let value = p.parse_expr()?;
                        pairs.push((key, value));

                        if !p.optional(TokenKind::Comma) {
                            break;
                        }
                    }

                    let last = p.expect_consume(TokenKind::BraceClose)?;
                    Ok(Node {
                        node: Expr::Value(value::Map { pairs }.into()),
                        span: span![first.span.begin, last.span.end],
                    })
                }
                TokenKind::OpMinus | TokenKind::OpBang | TokenKind::OpAmp | TokenKind::OpTilde => {
                    p.consume()?;
                    let inner = p.parse_expr()?;
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
                TokenKind::KwFn => p.with_context(Context::ExprLambda, |p| {
                    p.consume()?;
                    let params = p
                        .check(TokenKind::ParenOpen)
                        .then(|| {
                            Ok(
                                p.parse_list((TokenKind::ParenOpen, TokenKind::ParenClose), |p| {
                                    let name = p.expect_consume(TokenKind::Identifier)?;
                                    p.expect_consume(TokenKind::Colon)?;
                                    let ty = p.parse_type()?;
                                    let last = ty.span;

                                    Ok(Node {
                                        node: Param {
                                            name,
                                            ty: ty.into(),
                                        },
                                        span: span![name.span.begin, last.end],
                                    })
                                })?,
                            )
                        })
                        .transpose()?
                        .unwrap_or(NodeList {
                            list: vec![],
                            span: token.span,
                        });

                    let result = p.with_context(Context::ReturnType, |p| {
                        p.check(TokenKind::Arrow)
                            .then(|| {
                                p.consume()?;
                                Ok(p.parse_type()?)
                            })
                            .transpose()
                    })?;

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
                TokenKind::OpHash => {
                    let attr = p.parse_attr()?;
                    let span = attr.span;
                    Ok(Node {
                        node: Expr::Value(value::Attr { attr: attr.into() }.into()),
                        span,
                    })
                }
                _ => p.error(ErrorKind::UnexpectedToken {
                    expected: vec![TokenKind::Identifier],
                    got: token,
                }),
            }
        })
    }

    fn parse_expr_postfix(&mut self) -> Result<Node<Expr>> {
        let mut expr = self.parse_expr_primary()?;
        loop {
            if let Ok(token) = self.peek() {
                match token.kind {
                    TokenKind::Period => {
                        self.consume()?;
                        let field = self.expect_consume(TokenKind::Identifier)?;
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
                    TokenKind::ColonColon => {
                        self.consume()?;
                        let field = self.expect_consume(TokenKind::Identifier)?;
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
                    TokenKind::BracketOpen => {
                        self.consume()?;
                        let index = self.parse_expr()?;
                        let first = expr.span;
                        let last = self.expect_consume(TokenKind::BracketClose)?;

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
                    TokenKind::OpDotDot => {
                        self.consume()?;
                        let inclusive = self.optional(TokenKind::OpEq);
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

    pub(crate) fn parse_type(&mut self) -> Result<Node<Ty>> {
        let token = self.peek()?;
        let mut lhs = match token.kind {
            TokenKind::KwNone
            | TokenKind::KwBool
            | TokenKind::KwInt
            | TokenKind::KwFloat
            | TokenKind::KwString => Node {
                node: ty::Builtin {
                    token: self.consume()?,
                }
                .into(),
                span: token.span,
            },
            TokenKind::BracketOpen => {
                self.consume()?;
                let ty = self.parse_type()?;
                let last = self.expect_consume(TokenKind::BracketClose)?;

                Node {
                    node: ty::Array { ty: ty.into() }.into(),
                    span: span![token.span.begin, last.span.end],
                }
            }
            TokenKind::BraceOpen => self.with_context(Context::TypeMap, |p| {
                p.consume()?;
                let key = p.parse_type()?;

                p.expect_consume(TokenKind::Colon)?;

                let value = p.parse_type()?;
                let last = p.expect_consume(TokenKind::BraceClose)?;

                Ok(Node {
                    node: ty::Map {
                        key: key.into(),
                        value: value.into(),
                    }
                    .into(),
                    span: span![token.span.begin, last.span.end],
                })
            })?,
            TokenKind::KwFn => self.with_context(Context::TypeLambda, |p| {
                p.consume()?;
                let params = p.parse_list(
                    (TokenKind::ParenOpen, TokenKind::ParenClose),
                    Self::parse_type,
                )?;
                p.expect_consume(TokenKind::Arrow)?;
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
            TokenKind::KwType => self.with_context(Context::TypeId, |p| {
                p.consume()?;
                p.expect_consume(TokenKind::ParenOpen)?;

                let expr = p.parse_expr()?;
                let last = p.expect_consume(TokenKind::ParenClose)?;

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
                TokenKind::Question => {
                    self.consume()?;
                    Node {
                        node: ty::Optional { ty: lhs.into() }.into(),
                        span: span![first.begin, token.span.end],
                    }
                }
                TokenKind::OpPipe => {
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

    fn parse_control_return(&mut self) -> Result<Node<control::Return>> {
        self.with_context(Context::ControlReturn, |p| {
            let first = p.expect_consume(TokenKind::KwReturn)?;
            let expr = p.with_context(Context::ReturnType, |p| {
                Ok(p.is_expr_start().then(|| p.parse_expr()).transpose()?)
            })?;

            let last = match &expr {
                Some(e) => e.span,
                _ => first.span,
            };

            Ok(Node {
                node: control::Return {
                    expr: expr.map(Into::into),
                },
                span: span![first.span.begin, last.end],
            })
        })
    }

    fn parse_control_raise(&mut self) -> Result<Node<control::Raise>> {
        self.with_context(Context::ControlRaise, |p| {
            let first = p.expect_consume(TokenKind::KwRaise)?.span;
            let expr = p.parse_expr()?;
            let last = expr.span;

            Ok(Node {
                node: control::Raise { expr: expr.into() },
                span: span![first.begin, last.end],
            })
        })
    }

    fn parse_control_if(&mut self) -> Result<Node<control::If>> {
        self.with_context(Context::ControlIf, |p| {
            let first = p.expect_consume(TokenKind::KwIf)?.span;
            let cond = p.parse_expr()?;
            let body = p.parse_body(Self::parse_stmt)?;
            let mut last = body.span;
            let mut elseif = vec![];

            p.with_context(Context::ControlElseIf, |p| {
                while p.check(TokenKind::KwElse) && p.check_ahead(TokenKind::KwIf, 1) {
                    p.consume()?;
                    p.consume()?;
                    let cond = p.parse_expr()?;
                    let body = p.parse_body(Self::parse_stmt)?;
                    last = body.span;
                    elseif.push((cond, body));
                }
                Ok(())
            })?;

            let else_body = p.with_context(Context::ControlElse, |p| {
                Ok(p.check(TokenKind::KwElse)
                    .then(|| {
                        p.consume()?;
                        let body = p.parse_body(Self::parse_stmt)?;
                        last = body.span;
                        Ok(body)
                    })
                    .transpose()?)
            })?;

            Ok(Node {
                node: control::If {
                    cond: cond.into(),
                    body,
                    elseif,
                    else_body: else_body.map(Into::into),
                },
                span: span![first.begin, last.end],
            })
        })
    }

    fn parse_control_while(&mut self) -> Result<Node<control::While>> {
        self.with_context(Context::ControlWhile, |p| {
            let first = p.expect_consume(TokenKind::KwWhile)?.span;
            let cond = p.parse_expr()?;
            let body = p.parse_body(Self::parse_stmt)?;
            let last = body.span;

            Ok(Node {
                node: control::While {
                    cond: cond.into(),
                    body,
                },
                span: span![first.begin, last.end],
            })
        })
    }

    fn parse_control_for(&mut self) -> Result<Node<control::For>> {
        self.with_context(Context::ControlFor, |p| {
            let first = p.expect_consume(TokenKind::KwFor)?.span;
            let param = p.expect_consume(TokenKind::Identifier)?;
            let ty = p
                .check(TokenKind::Colon)
                .then(|| {
                    p.consume()?;
                    Ok(p.parse_type()?)
                })
                .transpose()?;

            p.expect_consume(TokenKind::KwIn)?;

            let expr = p.parse_expr()?;
            let body = p.parse_body(Self::parse_stmt)?;
            let last = body.span;

            Ok(Node {
                node: control::For {
                    param: (param, ty.map(Into::into)),
                    expr: expr.into(),
                    body,
                },
                span: span![first.begin, last.end],
            })
        })
    }

    fn parse_control(&mut self) -> Result<Node<Control>> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwBreak => self.consume().map(|token| Node {
                    node: control::Break {}.into(),
                    span: token.span,
                }),
                TokenKind::KwContinue => self.consume().map(|token| Node {
                    node: control::Continue {}.into(),
                    span: token.span,
                }),
                TokenKind::KwReturn => self.parse_control_return().map(IntoNode::into_node),
                TokenKind::KwRaise => self.parse_control_raise().map(IntoNode::into_node),
                TokenKind::KwWhile => self.parse_control_while().map(IntoNode::into_node),
                TokenKind::KwFor => self.parse_control_for().map(IntoNode::into_node),
                TokenKind::KwIf => self.parse_control_if().map(IntoNode::into_node),
                _ => self.error(
                    ErrorKind::UnexpectedToken {
                        expected: vec![],
                        got: token,
                    }
                    .into(),
                ),
            }
        } else {
            self.error(ErrorKind::UnexpectedEndOfFile)
        }
    }

    fn parse_decl_variable(&mut self) -> Result<Node<decl::Variable>> {
        self.with_context(Context::DeclVariable, |p| {
            let first = p.expect_consume(TokenKind::KwVar)?.span;
            let symbol = p.expect_consume(TokenKind::Identifier)?;
            let ty = p
                .check(TokenKind::Colon)
                .then(|| {
                    p.consume()?;
                    Ok(p.parse_type()?)
                })
                .transpose()?;

            p.expect_consume(TokenKind::OpEq)?;

            let expr = p.parse_expr()?;
            let last = expr.span;

            Ok(Node {
                node: decl::Variable {
                    symbol: symbol,
                    ty: ty.map(Into::into),
                    expr: expr.into(),
                },
                span: span![first.begin, last.end],
            })
        })
    }

    fn parse_decl_function(&mut self) -> Result<Node<decl::Function>> {
        self.with_context(Context::DeclFunction, |p| {
            let first = p.expect_consume(TokenKind::KwFn)?.span;
            let symbol = p.expect_consume(TokenKind::Identifier)?;

            p.expect_consume(TokenKind::ParenOpen)?;

            let params = p.with_context(Context::ParameterList, |p| {
                let mut params = Vec::new();
                loop {
                    let symbol = p.expect_consume(TokenKind::Identifier)?;
                    p.expect_consume(TokenKind::Colon)?;

                    let ty = p.parse_type()?;
                    params.push(Param {
                        name: symbol,
                        ty: ty.into(),
                    });

                    if p.check(TokenKind::Comma) {
                        p.consume()?;
                    } else {
                        break Ok(params);
                    }
                }
            })?;

            p.expect_consume(TokenKind::ParenClose)?;

            let result = p.with_context(Context::ReturnType, |p| {
                p.check(TokenKind::Arrow)
                    .then(|| {
                        p.consume()?;
                        Ok(p.parse_type()?)
                    })
                    .transpose()
            })?;

            let body = p.parse_body(Self::parse_stmt)?;
            let last = body.span;

            Ok(Node {
                node: decl::Function {
                    symbol: symbol,
                    params,
                    result: result.map(IntoNode::into_node).map(Into::into),
                    body,
                },
                span: span![first.begin, last.end],
            })
        })
    }

    fn parse_decl_use(&mut self) -> Result<Node<decl::Use>> {
        todo!()
    }

    fn parse_decl_type(&mut self) -> Result<Node<decl::Type>> {
        self.with_context(Context::DeclType, |p| {
            let begin = p.expect_consume(TokenKind::KwType)?;
            let symbol = p.expect_consume(TokenKind::Identifier)?;
            p.expect_consume(TokenKind::OpEq)?;

            let ty = p.parse_type()?;
            let last = ty.span;

            Ok(Node {
                node: decl::Type {
                    symbol,
                    ty: ty.into(),
                },
                span: span![begin.span.begin, last.end],
            })
        })
    }

    fn parse_decl_const(&mut self) -> Result<Node<decl::Const>> {
        self.with_context(Context::DeclConst, |p| {
            let begin = p.expect_consume(TokenKind::KwConst)?;
            let symbol = p.expect_consume(TokenKind::Identifier)?;
            p.expect_consume(TokenKind::OpEq)?;

            let expr = p.parse_expr()?;
            let last = expr.span;

            Ok(Node {
                node: decl::Const {
                    symbol: symbol,
                    expr: expr.into(),
                },
                span: span![begin.span.begin, last.end],
            })
        })
    }

    fn parse_decl_struct(&mut self) -> Result<Node<decl::Struct>> {
        self.with_context(Context::DeclStruct, |p| {
            let first = p.expect_consume(TokenKind::KwStruct)?;
            let symbol = p.expect_consume(TokenKind::Identifier)?;
            let body = p.parse_body(Self::parse_decl)?;
            let last = body.span;

            Ok(Node {
                node: decl::Struct { symbol, body },
                span: span![first.span.begin, last.end],
            })
        })
    }

    fn parse_decl_import(&mut self) -> Result<Node<decl::Import>> {
        self.with_context(Context::DeclImport, |p| {
            let first = p.expect_consume(TokenKind::KwImport)?;
            let mut path = vec![p.expect_consume(TokenKind::Identifier)?];
            while p.check(TokenKind::Period) {
                p.consume()?;
                let token = p.expect_consume(TokenKind::Identifier)?;
                path.push(token);
            }

            let alias = p
                .check(TokenKind::KwAs)
                .then(|| {
                    p.consume()?;
                    Ok(p.expect_consume(TokenKind::Identifier)?)
                })
                .transpose()?;

            let span = span![
                first.span.begin,
                alias
                    .unwrap_or(
                        path.last()
                            .unwrap_or_else(|| bug!("misparsed import path"))
                            .clone()
                    )
                    .span
                    .end
            ];

            Ok(Node {
                node: decl::Import { path, alias },
                span,
            })
        })
    }

    fn parse_decl(&mut self) -> Result<Node<Decl>> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwVar => self.parse_decl_variable().map(IntoNode::into_node),
                TokenKind::KwFn => self.parse_decl_function().map(IntoNode::into_node),
                TokenKind::KwUse => self.parse_decl_use().map(IntoNode::into_node),
                TokenKind::KwType => self.parse_decl_type().map(IntoNode::into_node),
                TokenKind::KwConst => self.parse_decl_const().map(IntoNode::into_node),
                TokenKind::KwStruct => self.parse_decl_struct().map(IntoNode::into_node),
                TokenKind::KwImport => self.parse_decl_import().map(IntoNode::into_node),
                _ => self.error(ErrorKind::UnexpectedToken {
                    expected: vec![],
                    got: token,
                }),
            }
        } else {
            self.error(ErrorKind::UnexpectedEndOfFile)
        }
    }

    fn parse_stmt(&mut self) -> Result<Node<Stmt>> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwBreak
                | TokenKind::KwContinue
                | TokenKind::KwReturn
                | TokenKind::KwRaise
                | TokenKind::KwWhile
                | TokenKind::KwFor
                | TokenKind::KwIf => self.parse_control().map(|node| node.map(Stmt::Control)),
                TokenKind::KwVar
                | TokenKind::KwFn
                | TokenKind::KwUse
                | TokenKind::KwType
                | TokenKind::KwConst
                | TokenKind::KwStruct
                | TokenKind::KwImport => self.parse_decl().map(|node| node.map(Stmt::Decl)),
                _ if self.is_expr_start() => {
                    let expr = self.parse_expr()?;
                    match self.peek().map(|t| t.kind) {
                        Ok(TokenKind::OpEq)
                        | Ok(TokenKind::OpPlusEq)
                        | Ok(TokenKind::OpMinusEq)
                        | Ok(TokenKind::OpStarEq)
                        | Ok(TokenKind::OpSlashEq)
                        | Ok(TokenKind::OpStarStarEq)
                        | Ok(TokenKind::OpPercentEq)
                        | Ok(TokenKind::OpAmpEq)
                        | Ok(TokenKind::OpPipeEq) => {
                            let op = self.consume()?;
                            let rhs = self.parse_expr()?;
                            let first = expr.span;
                            let last = rhs.span;
                            Ok(Node {
                                node: Stmt::Control(
                                    control::Assign {
                                        op: op,
                                        lhs: expr.into(),
                                        rhs: rhs.into(),
                                    }
                                    .into(),
                                ),
                                span: span![first.begin, last.end],
                            })
                        }
                        _ => Ok(Node {
                            node: Stmt::Expr(expr.node),
                            span: expr.span,
                        }),
                    }
                }
                _ => self.error(ErrorKind::UnexpectedToken {
                    expected: vec![],
                    got: token,
                }),
            }
        } else {
            self.error(ErrorKind::UnexpectedEndOfFile)
        }
    }

    fn parse(&mut self) -> Result<Vec<Node<Stmt>>> {
        let mut ast = vec![];
        loop {
            if self.check(TokenKind::EndOfFile) {
                break Ok(ast);
            }
            let stmt = self.parse_stmt()?;
            ast.push(stmt);
        }
    }
}

pub fn parse(source: &Source, tokens: &[Token]) -> Result<Vec<Node<Stmt>>> {
    Parser {
        source,
        tokens,
        position: 0,
        contexts: vec![],
    }
    .parse()
}
