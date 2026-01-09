/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

mod error;

use crate::compiler::{
    ast::{
        control::{self, Control},
        decl::{self, Decl},
        expr::Expr,
        place,
        stmt::Stmt,
        typ::{self, Type},
        value,
    },
    lexer::token::{Token, TokenKind},
    parser::error::Error,
    source::*,
};
use crate::support::macros::bug;

pub struct Parser<'m> {
    tokens: &'m Vec<Token>,
    position: usize,
}

impl<'m> Parser<'m> {
    pub fn new(tokens: &'m Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Result<Token, Error> {
        if let Some(tok) = self.tokens[self.position..].iter().next() {
            Ok(tok.clone())
        } else {
            Err(Error::UnexpectedEndOfFile)
        }
    }

    fn peek_ahead(&self, ahead: u32) -> Result<Token, Error> {
        if let Some(tok) = self.tokens[self.position..].iter().nth(ahead as usize) {
            Ok(tok.clone())
        } else {
            Err(Error::UnexpectedEndOfFile)
        }
    }

    fn consume(&mut self) -> Result<Token, Error> {
        let tok = self.peek()?;
        self.position += 1;
        Ok(tok)
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.peek().is_ok_and(|tok| tok.kind == kind)
    }

    fn check_ahead(&self, kind: TokenKind, ahead: u32) -> bool {
        self.peek_ahead(ahead).is_ok_and(|tok| tok.kind == kind)
    }

    #[allow(dead_code)]
    fn expect(&self, kind: TokenKind, task: &'static str) -> Result<Token, Error> {
        let tok = self.peek()?;
        if tok.kind == kind {
            Ok(tok)
        } else {
            Err(Error::UnexpectedToken {
                token: tok,
                task: task,
            })
        }
    }

    fn expect_consume(&mut self, kind: TokenKind, task: &'static str) -> Result<Token, Error> {
        let tok = self.consume()?;
        if tok.kind == kind {
            Ok(tok)
        } else {
            Err(Error::UnexpectedToken {
                token: tok,
                task: task,
            })
        }
    }

    fn parse_body_with<F, T>(&mut self, f: F) -> Result<(Span, Vec<T>), Error>
    where
        F: Fn(&mut Self) -> Result<T, Error>,
    {
        let first = self.expect_consume(TokenKind::BraceOpen, "parsing body")?;
        let mut body: Vec<T> = vec![];

        while !self.check(TokenKind::BraceClose) {
            let stmt = f(self)?;
            body.push(stmt);
        }

        let last = self.expect_consume(TokenKind::BraceClose, "terminating body")?;
        Ok((span![first.span.begin, last.span.end], body))
    }

    fn parse_body(&mut self) -> Result<(Span, Vec<Stmt>), Error> {
        self.parse_body_with(Self::parse_stmt)
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

    fn parse_expr_primary(&mut self) -> Result<Expr, Error> {
        let tok = self.peek()?;
        match tok.kind {
            TokenKind::Identifier => {
                self.consume()?;
                Ok(Expr::Place(
                    place::Symbol {
                        span: tok.span,
                        token: tok,
                    }
                    .into(),
                ))
            }
            TokenKind::KwSelf => {
                self.consume()?;
                Ok(Expr::Place(place::This { span: tok.span }.into()))
            }
            TokenKind::KwNone => {
                self.consume()?;
                Ok(Expr::Value(value::None { span: tok.span }.into()))
            }
            TokenKind::KwTrue => {
                self.consume()?;
                Ok(Expr::Value(value::True { span: tok.span }.into()))
            }
            TokenKind::KwFalse => {
                self.consume()?;
                Ok(Expr::Value(value::False { span: tok.span }.into()))
            }
            TokenKind::LitInt | TokenKind::LitFloat | TokenKind::LitString => {
                self.consume()?;
                Ok(Expr::Value(
                    value::Constant {
                        span: tok.span,
                        token: tok,
                    }
                    .into(),
                ))
            }
            TokenKind::ParenOpen => {
                let first = self.consume()?;
                let inner = self.parse_expr()?;
                let last =
                    self.expect_consume(TokenKind::ParenClose, "terminating group expression")?;

                Ok(Expr::Value(
                    value::Group {
                        span: span![first.span.begin, last.span.end],
                        expr: Box::new(inner),
                    }
                    .into(),
                ))
            }
            TokenKind::OpMinus | TokenKind::OpBang | TokenKind::OpAmp | TokenKind::OpTilde => {
                self.consume()?;
                let inner = self.parse_expr()?;
                return Ok(Expr::Value(
                    value::Unary {
                        span: span![tok.span.begin, inner.span().end],
                        op: tok,
                        expr: Box::new(inner),
                    }
                    .into(),
                ));
            }
            _ => Err(Error::UnexpectedToken {
                token: tok,
                task: "parsing primary expression",
            }),
        }
    }

    fn parse_expr_postfix(&mut self) -> Result<Expr, Error> {
        let mut expr = self.parse_expr_primary()?;
        loop {
            if let Ok(tok) = self.peek() {
                match tok.kind {
                    TokenKind::Period => {
                        self.consume()?;
                        let field = self.expect_consume(
                            TokenKind::Identifier,
                            "parsing dynamic access expression field",
                        )?;

                        expr = Expr::Place(
                            place::Dynamic {
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
                        let last = self.expect_consume(
                            TokenKind::BracketClose,
                            "terminating subscript expression",
                        )?;

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

    fn parse_expr_binary(&mut self, min_prec: u8) -> Result<Expr, Error> {
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

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        self.parse_expr_binary(0)
    }

    fn parse_type(&mut self) -> Result<Type, Error> {
        todo!()
    }

    fn parse_control_return(&mut self) -> Result<control::Return, Error> {
        let first = self.expect_consume(TokenKind::KwReturn, "parsing return statement")?;
        let expr = if self.is_expr_start() {
            Some(self.parse_expr()?)
        } else {
            None
        };

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
    }

    fn parse_control_raise(&mut self) -> Result<control::Raise, Error> {
        let first = self
            .expect_consume(TokenKind::KwRaise, "parsing raise statement")?
            .span;
        let expr = self.parse_expr()?;

        Ok(control::Raise {
            span: span![first.begin, expr.span().end],
            expr: Box::new(expr),
        })
    }

    fn parse_control_if(&mut self) -> Result<control::If, Error> {
        let first = self
            .expect_consume(TokenKind::KwIf, "parsing if statement")?
            .span;
        let cond = self.parse_expr()?;
        let body = self.parse_body()?;
        let mut last = body.0;
        let mut elifs: Vec<(Expr, Vec<Stmt>)> = vec![];

        while self.check(TokenKind::KwElse) && self.check_ahead(TokenKind::KwIf, 1) {
            self.consume()?;
            self.consume()?;
            let cond = self.parse_expr()?;
            let body = self.parse_body()?;
            elifs.push((cond, body.1));
            last = body.0;
        }

        let els = if self.check(TokenKind::KwElse) {
            self.consume()?;
            let body = self.parse_body()?;
            last = body.0;
            Some(body.1)
        } else {
            None
        };

        Ok(control::If {
            span: span![first.begin, last.end],
            cond: Box::new(cond),
            body: body.1,
            elifs: elifs,
            els: els,
        })
    }

    fn parse_control_while(&mut self) -> Result<control::While, Error> {
        let first = self
            .expect_consume(TokenKind::KwWhile, "parsing while statement")?
            .span;
        let cond = self.parse_expr()?;
        let body = self.parse_body()?;

        Ok(control::While {
            span: span![first.begin, body.0.end],
            cond: Box::new(cond),
            body: body.1,
        })
    }

    fn parse_control_whilenot(&mut self) -> Result<control::WhileNot, Error> {
        let first = self
            .expect_consume(TokenKind::KwWhilex, "parsing while-not statement")?
            .span;
        let cond = self.parse_expr()?;
        let body = self.parse_body()?;

        Ok(control::WhileNot {
            span: span![first.begin, body.0.end],
            cond: Box::new(cond),
            body: body.1,
        })
    }

    fn parse_control_for(&mut self) -> Result<control::For, Error> {
        let first = self
            .expect_consume(TokenKind::KwWhile, "parsing for statement")?
            .span;
        let _init = self.parse_decl_variable()?;
        self.expect_consume(TokenKind::Comma, "terminating for loop initializer")?;

        let cond = self.parse_expr()?;
        self.expect_consume(TokenKind::Comma, "terminating for loop condition")?;

        let action = self.parse_expr()?;
        let body = self.parse_body()?;

        Ok(control::For {
            span: span![first.begin, body.0.end],
            // init: init.1,
            cond: Box::new(cond),
            action: Box::new(action),
            body: body.1,
        })
    }

    fn parse_control_foreach(&mut self) -> Result<control::ForEach, Error> {
        let first = self
            .expect_consume(TokenKind::KwWhile, "parsing for each statement")?
            .span;
        let param = self.expect_consume(TokenKind::Identifier, "parsing for each parameter")?;
        let typ = if self.check(TokenKind::Colon) {
            self.consume()?;
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect_consume(TokenKind::KwIn, "terminating for each parameter")?;

        let expr = self.parse_expr()?;
        let body = self.parse_body()?;

        Ok(control::ForEach {
            span: span![first.begin, body.0.end],
            param: (param, typ.map(Box::new)),
            expr: Box::new(expr),
            body: body.1,
        })
    }

    fn parse_control(&mut self) -> Result<Control, Error> {
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
                TokenKind::KwWhilex => self.parse_control_whilenot().map(Into::into),
                TokenKind::KwFor if self.check_ahead(TokenKind::KwVar, 1) => {
                    self.parse_control_foreach().map(Into::into)
                }
                TokenKind::KwFor => self.parse_control_for().map(Into::into),
                TokenKind::KwIf => self.parse_control_if().map(Into::into),
                _ => Err(Error::UnexpectedToken {
                    token: token,
                    task: "parsing control statement",
                }),
            }
        } else {
            Err(Error::UnexpectedEndOfFile)
        }
    }

    fn parse_decl_variable(&mut self) -> Result<decl::Variable, Error> {
        let first = self
            .expect_consume(TokenKind::KwVar, "parsing variable declaration")?
            .span;
        let symbol = self.expect_consume(TokenKind::Identifier, "parsing variable name")?;
        let typ: Option<Type> = if self.check(TokenKind::Colon) {
            self.consume()?;
            Some(self.parse_type()?)
        } else {
            None
        };

        self.expect_consume(TokenKind::OpEq, "parsing variable declaration statement")?;

        let expr = self.parse_expr()?;

        Ok(decl::Variable {
            span: span![first.begin, expr.span().end],
            symbol: symbol,
            typ: typ.map(Box::new),
            expr: Box::new(expr),
        })
    }

    fn parse_decl_function(&mut self) -> Result<decl::Function, Error> {
        let first = self
            .expect_consume(TokenKind::KwFn, "parsing function declaration")?
            .span;
        let symbol = self.expect_consume(TokenKind::Identifier, "parsing function name")?;
        self.expect_consume(TokenKind::ParenOpen, "parsing function parameter list")?;

        let mut params: Vec<(Token, Type)> = vec![];
        loop {
            let symbol =
                self.expect_consume(TokenKind::Identifier, "parsing function parameter name")?;
            self.expect_consume(TokenKind::Colon, "parsing function parameter type")?;
            let typ = self.parse_type()?;
            params.push((symbol, typ));

            if self.check(TokenKind::Comma) {
                self.consume()?;
            } else {
                break;
            }
        }

        self.expect_consume(TokenKind::ParenClose, "terminating function parameter list")?;

        let result = if self.check(TokenKind::Arrow) {
            self.consume()?;
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_body()?;

        Ok(decl::Function {
            span: span![first.begin, body.0.end],
            symbol: symbol,
            params: params,
            result: result.map(Box::new),
            body: body.1,
        })
    }

    fn parse_decl_use(&mut self) -> Result<decl::Use, Error> {
        todo!()
    }

    fn parse_decl_type(&mut self) -> Result<decl::Type, Error> {
        let begin = self.expect_consume(TokenKind::KwType, "parsing type declaration")?;
        let symbol = self.expect_consume(TokenKind::Identifier, "parsing type name")?;

        self.expect_consume(TokenKind::OpEq, "parsing type declaration")?;

        let typ = self.parse_type()?;

        Ok(decl::Type {
            span: span![begin.span.begin, typ.span().end],
            symbol: symbol,
            typ: Box::new(typ),
        })
    }

    fn parse_decl_const(&mut self) -> Result<decl::Const, Error> {
        let begin = self.expect_consume(TokenKind::KwConst, "parsing constant declaration")?;
        let symbol = self.expect_consume(TokenKind::Identifier, "parsing constant name")?;

        self.expect_consume(TokenKind::OpEq, "parsing constant declaration")?;

        let expr = self.parse_expr()?;

        Ok(decl::Const {
            span: span![begin.span.begin, expr.span().end],
            symbol: symbol,
            expr: Box::new(expr),
        })
    }

    fn parse_decl_struct(&mut self) -> Result<decl::Struct, Error> {
        let first = self.expect_consume(TokenKind::KwStruct, "parsing struct declaration")?;
        let symbol = self.expect_consume(TokenKind::Identifier, "parsing struct name")?;
        let fields = self.parse_body_with(Self::parse_decl)?;

        Ok(decl::Struct {
            span: span![first.span.begin, fields.0.end],
            symbol,
            fields: fields.1,
        })
    }

    fn parse_decl_import(&mut self) -> Result<decl::Import, Error> {
        let first = self.expect_consume(TokenKind::KwImport, "parsing import statement")?;
        let mut path: Vec<Token> =
            vec![self.expect_consume(TokenKind::Identifier, "parsing import path")?];

        while self.check(TokenKind::Period) {
            self.consume()?;
            let tok = self.expect_consume(TokenKind::Identifier, "parsing import path")?;
            path.push(tok);
        }

        let alias = if self.check(TokenKind::KwAs) {
            self.consume()?;
            Some(self.expect_consume(TokenKind::Identifier, "parsing import alias")?)
        } else {
            None
        };

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
    }

    fn parse_decl(&mut self) -> Result<Decl, Error> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwVar => self.parse_decl_variable().map(Into::into),
                TokenKind::KwFn => self.parse_decl_function().map(Into::into),
                TokenKind::KwUse => self.parse_decl_use().map(Into::into),
                TokenKind::KwType => self.parse_decl_type().map(Into::into),
                TokenKind::KwConst => self.parse_decl_const().map(Into::into),
                TokenKind::KwStruct => self.parse_decl_struct().map(Into::into),
                TokenKind::KwImport => self.parse_decl_import().map(Into::into),
                _ => Err(Error::UnexpectedToken {
                    token: token,
                    task: "parsing declaration statement",
                }),
            }
        } else {
            Err(Error::UnexpectedEndOfFile)
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, Error> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwBreak
                | TokenKind::KwContinue
                | TokenKind::KwReturn
                | TokenKind::KwRaise
                | TokenKind::KwWhile
                | TokenKind::KwWhilex
                | TokenKind::KwFor
                | TokenKind::KwIf => self.parse_control().map(|ctrl| Stmt::Control(ctrl)),
                TokenKind::KwVar
                | TokenKind::KwFn
                | TokenKind::KwUse
                | TokenKind::KwType
                | TokenKind::KwConst
                | TokenKind::KwStruct
                | TokenKind::KwImport => self.parse_decl().map(|decl| Stmt::Decl(decl)),
                _ if self.is_expr_start() => Ok(Stmt::Expr(self.parse_expr()?)),
                _ => Err(Error::UnexpectedToken {
                    token: token,
                    task: "parsing statement",
                }),
            }
        } else {
            Err(Error::UnexpectedEndOfFile)
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
        let mut ast: Vec<Stmt> = vec![];
        while !self.check(TokenKind::EndOfFile) {
            let stmt = self.parse_stmt()?;
            ast.push(stmt);
        }
        Ok(ast)
    }
}
