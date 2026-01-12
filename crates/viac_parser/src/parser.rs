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
use viac_ast::body::Body;
use viac_ast::control::{self, Control};
use viac_ast::decl::{self, Decl};
use viac_ast::expr::Expr;
use viac_ast::node::Node;
use viac_ast::param::Param;
use viac_ast::place;
use viac_ast::stmt::Stmt;
use viac_ast::ty::{self, Ty};
use viac_ast::value;
use viac_lexer::token::{Token, TokenKind};
use viac_source::span;
use viac_source::span::Span;

pub struct Parser<'a> {
    tokens: &'a [Token],
    position: usize,
    contexts: Vec<Context>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
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
        self.peek().map(|tok| {
            self.position += 1;
            tok
        })
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().is_ok_and(|tok| tok.kind == kind)
    }

    fn check_ahead(&self, kind: TokenKind, ahead: u32) -> bool {
        self.peek_ahead(ahead).is_ok_and(|tok| tok.kind == kind)
    }

    #[allow(dead_code)]
    fn expect(&self, kind: TokenKind) -> Result<Token> {
        let tok = self.peek()?;
        (tok.kind == kind).then_some(tok).ok_or_else(|| {
            self.error::<Token>(ErrorKind::UnexpectedToken {
                expected: vec![kind],
                got: tok,
            })
            .err()
            .unwrap()
        })
    }

    fn expect_consume(&mut self, kind: TokenKind) -> Result<Token> {
        let tok = self.consume()?;
        (tok.kind == kind).then_some(tok).ok_or_else(|| {
            self.error::<Token>(ErrorKind::UnexpectedToken {
                expected: vec![kind],
                got: tok,
            })
            .err()
            .unwrap()
        })
    }

    fn parse_body<F, T>(&mut self, ctx: Context, mut parse: F) -> Result<Body<T>>
    where
        F: FnMut(&mut Self) -> Result<T>,
        T: Node,
    {
        self.with_context(ctx, |p| {
            let first = p.expect_consume(TokenKind::BraceOpen)?;
            let mut body = Vec::new();

            while !p.check(TokenKind::BraceClose) {
                let node = parse(p)?;
                body.push(node);
            }

            let last = p.expect_consume(TokenKind::BraceClose)?;
            Ok(Body(span![first.span.begin, last.span.end], body))
        })
    }

    fn parse_list<F, T>(&mut self, ctx: Context, mut parse: F) -> Result<(Span, Vec<T>)>
    where
        F: FnMut(&mut Self) -> Result<T>,
    {
        self.with_context(ctx, |p| {
            let first = p.expect_consume(TokenKind::ParenOpen)?;
            let mut body = vec![];

            loop {
                let node = parse(p)?;
                body.push(node);
                if p.check(TokenKind::Comma) {
                    p.consume()?;
                } else {
                    break;
                }
            }

            let last = p.expect_consume(TokenKind::ParenClose)?;
            Ok((span![first.span.begin, last.span.end], body))
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
                | TokenKind::BracketOpen)
        )
    }

    fn parse_expr_primary(&mut self) -> Result<Expr> {
        self.with_context(Context::ExprPrimary, |p| {
            let tok = p.peek()?;
            match tok.kind {
                TokenKind::Identifier => {
                    p.consume()?;
                    Ok(Expr::Place(place::Symbol { token: tok }.into()))
                }
                TokenKind::KwSelf => {
                    p.consume()?;
                    Ok(Expr::Place(place::This { span: tok.span }.into()))
                }
                TokenKind::KwNone => {
                    p.consume()?;
                    Ok(Expr::Value(value::None { span: tok.span }.into()))
                }
                TokenKind::KwTrue => {
                    p.consume()?;
                    Ok(Expr::Value(value::True { span: tok.span }.into()))
                }
                TokenKind::KwFalse => {
                    p.consume()?;
                    Ok(Expr::Value(value::False { span: tok.span }.into()))
                }
                TokenKind::LitInt | TokenKind::LitXint | TokenKind::LitBint => {
                    p.consume()?;
                    Ok(Expr::Value(value::Integer { token: tok }.into()))
                }
                TokenKind::LitFloat => {
                    p.consume()?;
                    Ok(Expr::Value(value::Float { token: tok }.into()))
                }
                TokenKind::LitString => {
                    p.consume()?;
                    Ok(Expr::Value(value::String { token: tok }.into()))
                }
                TokenKind::ParenOpen => {
                    let first = p.consume()?;
                    let expr = p.parse_expr()?;

                    let expr = if p.check(TokenKind::Comma) {
                        p.push_context(Context::ExprTuple);

                        let mut exprs = vec![expr];

                        while p.check(TokenKind::Comma) {
                            p.consume()?;
                            if p.check(TokenKind::ParenClose) {
                                break;
                            }
                            let next = p.parse_expr()?;
                            exprs.push(next);
                        }

                        let last = p.expect_consume(TokenKind::ParenClose)?;

                        Expr::Value(
                            value::Tuple {
                                span: span![first.span.begin, last.span.end],
                                exprs,
                            }
                            .into(),
                        )
                    } else {
                        p.push_context(Context::ExprGroup);

                        let last = p.expect_consume(TokenKind::ParenClose)?;
                        Expr::Value(
                            value::Group {
                                span: span![first.span.begin, last.span.end],
                                expr: Box::new(expr),
                            }
                            .into(),
                        )
                    };
                    p.pop_context();
                    Ok(expr)
                }
                TokenKind::OpMinus | TokenKind::OpBang | TokenKind::OpAmp | TokenKind::OpTilde => {
                    p.consume()?;
                    let inner = p.parse_expr()?;
                    Ok(Expr::Value(
                        value::Unary {
                            span: span![tok.span.begin, inner.span().end],
                            op: tok,
                            expr: Box::new(inner),
                        }
                        .into(),
                    ))
                }
                TokenKind::KwFn => p.with_context(Context::ExprLambda, |p| {
                    p.consume()?;
                    let mut params = vec![];

                    if p.check(TokenKind::ParenOpen) {
                        params = p
                            .parse_list(Context::ParameterList, |p| {
                                let name = p.expect_consume(TokenKind::Identifier)?;
                                p.expect_consume(TokenKind::Colon)?;
                                let ty = p.parse_type()?;
                                Ok(Param(name, Box::new(ty)))
                            })?
                            .1;
                    }

                    let result = p.with_context(Context::ReturnType, |p| {
                        p.check(TokenKind::Arrow)
                            .then(|| {
                                p.consume()?;
                                Ok(Box::new(p.parse_type()?))
                            })
                            .transpose()
                    })?;

                    let body = p.parse_body(Context::Body, Self::parse_stmt)?;

                    Ok(Expr::Value(
                        value::Lambda {
                            span: span![tok.span.begin, body.0.end],
                            params,
                            result,
                            body,
                        }
                        .into(),
                    ))
                }),
                _ => p.error(ErrorKind::UnexpectedToken {
                    expected: vec![TokenKind::Identifier],
                    got: tok,
                }),
            }
        })
    }

    fn parse_expr_postfix(&mut self) -> Result<Expr> {
        let mut expr = self.parse_expr_primary()?;
        loop {
            if let Ok(tok) = self.peek() {
                match tok.kind {
                    TokenKind::Period => {
                        self.consume()?;
                        let field = self.expect_consume(TokenKind::Identifier)?;

                        expr = Expr::Place(
                            place::Dynamic {
                                span: span![tok.span.begin, field.span.end],
                                expr: Box::new(expr),
                                field: field,
                            }
                            .into(),
                        );
                    }
                    TokenKind::ColonColon => {
                        self.consume()?;
                        let field = self.expect_consume(TokenKind::Identifier)?;

                        expr = Expr::Place(
                            place::Static {
                                span: span![tok.span.begin, field.span.end],
                                expr: Box::new(expr),
                                field: field,
                            }
                            .into(),
                        );
                    }
                    TokenKind::BracketOpen => {
                        self.consume()?;
                        let index = self.parse_expr()?;
                        let last = self.expect_consume(TokenKind::BracketClose)?;

                        expr = Expr::Place(
                            place::Subscript {
                                span: span![tok.span.begin, last.span.end],
                                expr: Box::new(expr),
                                index: Box::new(index),
                            }
                            .into(),
                        );
                    }
                    _ => {}
                }
            }
            break;
        }
        Ok(expr)
    }

    fn parse_expr_binary(&mut self, min_prec: u8) -> Result<Expr> {
        let mut lhs = self.parse_expr_postfix()?;
        loop {
            let op = match self.peek() {
                Ok(tok) => tok,
                Err(_) => break,
            };

            let prec = match op.kind.bin_prec() {
                Some(prec) if prec >= min_prec => prec,
                Some(_) | None => break,
            };

            self.consume()?;
            let rhs = self.parse_expr_binary(prec + 1)?;

            lhs = Expr::Value(
                value::Binary {
                    span: span![lhs.span().begin, rhs.span().end],
                    op,
                    lhs: Box::new(lhs),
                    rhs: Box::new(rhs),
                }
                .into(),
            );
        }
        Ok(lhs)
    }

    fn parse_expr(&mut self) -> Result<Expr> {
        self.parse_expr_binary(0)
    }

    fn parse_type(&mut self) -> Result<Ty> {
        let tok = self.peek()?;
        let mut lhs: Ty = match tok.kind {
            TokenKind::KwNone
            | TokenKind::KwBool
            | TokenKind::KwInt
            | TokenKind::KwFloat
            | TokenKind::KwString => ty::Builtin {
                span: tok.span,
                token: self.consume()?,
            }
            .into(),
            TokenKind::BracketOpen => {
                self.consume()?;
                let ty = self.parse_type()?;
                let last = self.expect_consume(TokenKind::BracketClose)?;

                ty::Array {
                    span: span![tok.span.begin, last.span.end],
                    ty: Box::new(ty),
                }
                .into()
            }
            TokenKind::BraceOpen => self.with_context(Context::TypeMap, |p| {
                p.consume()?;
                let key = p.parse_type()?;

                p.expect_consume(TokenKind::Colon)?;

                let value = p.parse_type()?;
                let last = p.expect_consume(TokenKind::BraceClose)?;

                Ok(ty::Map {
                    span: span![tok.span.begin, last.span.end],
                    key: Box::new(key),
                    value: Box::new(value),
                }
                .into())
            })?,
            TokenKind::KwFn => self.with_context(Context::TypeLambda, |p| {
                p.consume()?;
                let params = p.parse_list(Context::ParameterList, Self::parse_type)?;
                p.expect_consume(TokenKind::Arrow)?;
                let result = p.parse_type()?;

                Ok(ty::Function {
                    span: span![tok.span.begin, result.span().end],
                    params: params.1,
                    result: Box::new(result),
                }
                .into())
            })?,
            TokenKind::KwType => self.with_context(Context::TypeId, |p| {
                p.consume()?;
                p.expect_consume(TokenKind::ParenOpen)?;

                let expr = p.parse_expr()?;
                let last = p.expect_consume(TokenKind::ParenClose)?;

                Ok(ty::TypeOf {
                    span: span![tok.span.begin, last.span.end],
                    expr: Box::new(expr),
                }
                .into())
            })?,
            _ => {
                return self.error(ErrorKind::UnexpectedToken {
                    expected: vec![],
                    got: tok,
                });
            }
        };

        loop {
            let tok = self.peek()?;
            let postfix: Ty = match tok.kind {
                TokenKind::Question => {
                    self.consume()?;
                    ty::Optional {
                        span: span![lhs.span().begin, tok.span.end],
                        ty: Box::new(lhs),
                    }
                    .into()
                }
                TokenKind::OpPipe => {
                    self.consume()?;
                    let rhs = self.parse_type()?;
                    ty::Union {
                        span: span![lhs.span().begin, rhs.span().end],
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                    }
                    .into()
                }
                _ => break Ok(lhs),
            };
            lhs = postfix;
        }
    }

    fn parse_control_return(&mut self) -> Result<control::Return> {
        self.with_context(Context::ControlReturn, |p| {
            let first = p.expect_consume(TokenKind::KwReturn)?;
            let expr = p.with_context(Context::ReturnType, |p| {
                Ok(p.is_expr_start().then(|| p.parse_expr()).transpose()?)
            })?;

            Ok(control::Return {
                span: span![
                    first.span.begin,
                    match &expr {
                        Some(e) => e.span().end,
                        _ => first.span.end,
                    }
                ],
                expr: expr.map(Box::new),
            })
        })
    }

    fn parse_control_raise(&mut self) -> Result<control::Raise> {
        self.with_context(Context::ControlRaise, |p| {
            let first = p.expect_consume(TokenKind::KwRaise)?.span;
            let expr = p.parse_expr()?;

            Ok(control::Raise {
                span: span![first.begin, expr.span().end],
                expr: Box::new(expr),
            })
        })
    }

    fn parse_control_if(&mut self) -> Result<control::If> {
        self.with_context(Context::ControlIf, |p| {
            let first = p.expect_consume(TokenKind::KwIf)?.span;
            let cond = p.parse_expr()?;
            let body = p.parse_body(Context::Body, Self::parse_stmt)?;
            let mut last = body.0;
            let mut elseif = vec![];

            p.with_context(Context::ControlElseIf, |p| {
                while p.check(TokenKind::KwElse) && p.check_ahead(TokenKind::KwIf, 1) {
                    p.consume()?;
                    p.consume()?;
                    let cond = p.parse_expr()?;
                    let body = p.parse_body(Context::Body, Self::parse_stmt)?;
                    last = body.0;
                    elseif.push((cond, body));
                }
                Ok(())
            })?;

            let else_body = p.with_context(Context::ControlElse, |p| {
                Ok(p.check(TokenKind::KwElse)
                    .then(|| {
                        p.consume()?;
                        let body = p.parse_body(Context::Body, Self::parse_stmt)?;
                        last = body.0;
                        Ok(body)
                    })
                    .transpose()?)
            })?;

            Ok(control::If {
                span: span![first.begin, last.end],
                cond: Box::new(cond),
                body: body,
                elifs: elseif,
                els: else_body,
            })
        })
    }

    fn parse_control_while(&mut self) -> Result<control::While> {
        self.with_context(Context::ControlWhile, |p| {
            let first = p.expect_consume(TokenKind::KwWhile)?.span;
            let cond = p.parse_expr()?;
            let body = p.parse_body(Context::Body, Self::parse_stmt)?;

            Ok(control::While {
                span: span![first.begin, body.0.end],
                cond: Box::new(cond),
                body,
            })
        })
    }

    fn parse_control_for(&mut self) -> Result<control::For> {
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
            let body = p.parse_body(Context::Body, Self::parse_stmt)?;

            Ok(control::For {
                span: span![first.begin, body.0.end],
                param: (param, ty.map(Box::new)),
                expr: Box::new(expr),
                body,
            })
        })
    }

    fn parse_control(&mut self) -> Result<Control> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwBreak => self
                    .consume()
                    .map(|token| control::Break { span: token.span }.into()),
                TokenKind::KwContinue => self
                    .consume()
                    .map(|token| control::Continue { span: token.span }.into()),
                TokenKind::KwReturn => self.parse_control_return().map(Into::into),
                TokenKind::KwRaise => self.parse_control_raise().map(Into::into),
                TokenKind::KwWhile => self.parse_control_while().map(Into::into),
                TokenKind::KwFor => self.parse_control_for().map(Into::into),
                TokenKind::KwIf => self.parse_control_if().map(Into::into),
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

    fn parse_decl_variable(&mut self) -> Result<decl::Variable> {
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

            Ok(decl::Variable {
                span: span![first.begin, expr.span().end],
                symbol: symbol,
                ty: ty.map(Box::new),
                expr: Box::new(expr),
            })
        })
    }

    fn parse_decl_function(&mut self) -> Result<decl::Function> {
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
                    let param = Param(symbol, Box::new(ty));
                    params.push(param);

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

            let body = p.parse_body(Context::Body, Self::parse_stmt)?;

            Ok(decl::Function {
                span: span![first.begin, body.0.end],
                symbol: symbol,
                params,
                result: result.map(Box::new),
                body,
            })
        })
    }

    fn parse_decl_use(&mut self) -> Result<decl::Use> {
        todo!()
    }

    fn parse_decl_type(&mut self) -> Result<decl::Type> {
        self.with_context(Context::DeclType, |p| {
            let begin = p.expect_consume(TokenKind::KwType)?;
            let symbol = p.expect_consume(TokenKind::Identifier)?;
            p.expect_consume(TokenKind::OpEq)?;

            let ty = p.parse_type()?;

            Ok(decl::Type {
                span: span![begin.span.begin, ty.span().end],
                symbol,
                ty: Box::new(ty),
            })
        })
    }

    fn parse_decl_const(&mut self) -> Result<decl::Const> {
        self.with_context(Context::DeclConst, |p| {
            let begin = p.expect_consume(TokenKind::KwConst)?;
            let symbol = p.expect_consume(TokenKind::Identifier)?;
            p.expect_consume(TokenKind::OpEq)?;

            let expr = p.parse_expr()?;

            Ok(decl::Const {
                span: span![begin.span.begin, expr.span().end],
                symbol: symbol,
                expr: Box::new(expr),
            })
        })
    }

    fn parse_decl_struct(&mut self) -> Result<decl::Struct> {
        self.with_context(Context::DeclStruct, |p| {
            let first = p.expect_consume(TokenKind::KwStruct)?;

            let symbol = p.expect_consume(TokenKind::Identifier)?;
            let body = p.parse_body(Context::Body, Self::parse_decl)?;

            Ok(decl::Struct {
                span: span![first.span.begin, body.0.end],
                symbol,
                body,
            })
        })
    }

    fn parse_decl_import(&mut self) -> Result<decl::Import> {
        self.with_context(Context::DeclImport, |p| {
            let first = p.expect_consume(TokenKind::KwImport)?;
            let mut path = vec![p.expect_consume(TokenKind::Identifier)?];
            while p.check(TokenKind::Period) {
                p.consume()?;
                let tok = p.expect_consume(TokenKind::Identifier)?;
                path.push(tok);
            }

            let alias = p
                .check(TokenKind::KwAs)
                .then(|| {
                    p.consume()?;
                    Ok(p.expect_consume(TokenKind::Identifier)?)
                })
                .transpose()?;

            Ok(decl::Import {
                span: span![
                    first.span.begin,
                    alias
                        .unwrap_or(
                            path.last()
                                .unwrap_or_else(|| bug!("misparsed import path"))
                                .clone()
                        )
                        .span
                        .end
                ],
                path,
                alias,
            })
        })
    }

    fn parse_decl(&mut self) -> Result<Decl> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwVar => self.parse_decl_variable().map(Into::into),
                TokenKind::KwFn => self.parse_decl_function().map(Into::into),
                TokenKind::KwUse => self.parse_decl_use().map(Into::into),
                TokenKind::KwType => self.parse_decl_type().map(Into::into),
                TokenKind::KwConst => self.parse_decl_const().map(Into::into),
                TokenKind::KwStruct => self.parse_decl_struct().map(Into::into),
                TokenKind::KwImport => self.parse_decl_import().map(Into::into),
                _ => self.error(ErrorKind::UnexpectedToken {
                    expected: vec![],
                    got: token,
                }),
            }
        } else {
            self.error(ErrorKind::UnexpectedEndOfFile)
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwBreak
                | TokenKind::KwContinue
                | TokenKind::KwReturn
                | TokenKind::KwRaise
                | TokenKind::KwWhile
                | TokenKind::KwFor
                | TokenKind::KwIf => self.parse_control().map(Stmt::Control),
                TokenKind::KwVar
                | TokenKind::KwFn
                | TokenKind::KwUse
                | TokenKind::KwType
                | TokenKind::KwConst
                | TokenKind::KwStruct
                | TokenKind::KwImport => self.parse_decl().map(Stmt::Decl),
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
                            Ok(Stmt::Control(
                                control::Assign {
                                    op: op,
                                    lhs: Box::new(expr),
                                    rhs: Box::new(rhs),
                                }
                                .into(),
                            ))
                        }
                        _ => Ok(Stmt::Expr(expr)),
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

    fn parse(&mut self) -> Result<Vec<Stmt>> {
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

pub fn parse(tokens: &[Token]) -> Result<Vec<Stmt>> {
    Parser {
        tokens,
        position: 0,
        contexts: vec![],
    }
    .parse()
}
