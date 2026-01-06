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
        expr::{Expr, ExprKind, ExprRef},
        place::{self, Place},
        stmt::{Stmt, StmtKind, StmtRef},
        tree::Tree,
        typ::{Type, TypeRef},
        value::{self, Value},
    },
    lexer::token::{Token, TokenKind},
    macros::*,
    parser::error::Error,
    source::*,
};
use bumpalo::Bump;

pub struct Parser<'m> {
    source: &'m Source,
    tree: Tree<'m>,
    tokens: Vec<Token>,
    position: usize,
}

impl Parser<'_> {
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

    fn parse_body(&mut self) -> Result<(Span, Vec<StmtRef>), Error> {
        let begin = self.expect_consume(TokenKind::BraceOpen, "parsing body")?;
        let mut body: Vec<StmtRef> = vec![];

        while !self.check(TokenKind::BraceClose) {
            let stmt = self.parse_stmt_ref()?;
            body.push(stmt);
        }

        let end = self.expect_consume(TokenKind::BraceClose, "terminating body")?;
        Ok((span![begin.span.begin, end.span.end], body))
    }

    fn is_expr_start(&self) -> bool {
        todo!()
    }

    fn parse_expr(&mut self) -> Result<Expr, Error> {
        todo!()
    }

    fn parse_expr_ref(&mut self) -> Result<ExprRef, Error> {
        self.parse_expr().map(|expr| self.tree.expr(expr))
    }

    fn parse_type(&mut self) -> Result<Type, Error> {
        todo!()
    }

    fn parse_type_ref(&mut self) -> Result<TypeRef, Error> {
        self.parse_type().map(|typ| self.tree.typ(typ))
    }

    fn parse_control_break(&mut self) -> Result<(Span, control::Break), Error> {
        let first = self
            .expect_consume(TokenKind::KwBreak, "parsing break statement")?
            .span;

        Ok((first, control::Break {}))
    }

    fn parse_control_continue(&mut self) -> Result<(Span, control::Continue), Error> {
        let first = self
            .expect_consume(TokenKind::KwContinue, "parsing continue statement")?
            .span;

        Ok((first, control::Continue {}))
    }

    fn parse_control_return(&mut self) -> Result<(Span, control::Return), Error> {
        let first = self
            .expect_consume(TokenKind::KwReturn, "parsing return statement")?
            .span;
        let expr = if self.is_expr_start() {
            Some(self.parse_expr_ref()?)
        } else {
            None
        };

        Ok((
            span![
                first.begin,
                match expr {
                    Some(e) => self.tree.to_expr(e).span.end,
                    _ => first.end,
                }
            ],
            control::Return { expr: expr },
        ))
    }

    fn parse_control_raise(&mut self) -> Result<(Span, control::Raise), Error> {
        let first = self
            .expect_consume(TokenKind::KwRaise, "parsing raise statement")?
            .span;
        let expr = self.parse_expr_ref()?;
        let last = self.tree.to_expr(expr).span;

        Ok((span![first.begin, last.end], control::Raise { expr: expr }))
    }

    fn parse_control_while(&mut self) -> Result<(Span, control::While), Error> {
        let first = self
            .expect_consume(TokenKind::KwWhile, "parsing while statement")?
            .span;
        let cond = self.parse_expr_ref()?;
        let body = self.parse_body()?;

        Ok((
            span![first.begin, body.0.end],
            control::While {
                cond: cond,
                body: body.1,
            },
        ))
    }

    fn parse_control_for(&mut self) -> Result<(Span, control::For), Error> {
        let first = self
            .expect_consume(TokenKind::KwWhile, "parsing for statement")?
            .span;
        let init = self.parse_decl_variable()?;
        self.expect_consume(TokenKind::Comma, "terminating for loop initializer")?;

        let cond = self.parse_expr_ref()?;
        self.expect_consume(TokenKind::Comma, "terminating for loop condition")?;

        let action = self.parse_expr_ref()?;
        let body = self.parse_body()?;

        Ok((
            span![first.begin, body.0.end],
            control::For {
                init: init.1,
                cond: cond,
                action: action,
                body: body.1,
            },
        ))
    }

    fn parse_control_foreach(&mut self) -> Result<(Span, control::ForEach), Error> {
        let first = self
            .expect_consume(TokenKind::KwWhile, "parsing for each statement")?
            .span;
        let param = self.expect_consume(TokenKind::Identifier, "parsing for each parameter")?;
        let typ = if self.check(TokenKind::Colon) {
            self.consume()?;
            Some(self.parse_type_ref()?)
        } else {
            None
        };

        self.expect_consume(TokenKind::KwIn, "terminating for each parameter")?;

        let expr = self.parse_expr_ref()?;
        let body = self.parse_body()?;

        Ok((
            span![first.begin, body.0.end],
            control::ForEach {
                param: (param, typ),
                expr: expr,
                body: body.1,
            },
        ))
    }

    fn parse_control(&mut self) -> Result<(Span, Control), Error> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwBreak => self
                    .parse_control_break()
                    .map(|(span, brk)| (span, Control::Break(brk))),
                TokenKind::KwContinue => self
                    .parse_control_continue()
                    .map(|(span, cont)| (span, Control::Continue(cont))),
                TokenKind::KwReturn => self
                    .parse_control_return()
                    .map(|(span, ret)| (span, Control::Return(ret))),
                TokenKind::KwRaise => self
                    .parse_control_raise()
                    .map(|(span, raise)| (span, Control::Raise(raise))),
                TokenKind::KwWhile => self
                    .parse_control_while()
                    .map(|(span, ret)| (span, Control::While(ret))),
                TokenKind::KwFor if self.check_ahead(TokenKind::KwVar, 1) => self
                    .parse_control_foreach()
                    .map(|(span, foreach)| (span, Control::ForEach(foreach))),
                TokenKind::KwFor => self
                    .parse_control_for()
                    .map(|(span, forl)| (span, Control::For(forl))),
                _ => Err(Error::UnexpectedToken {
                    token: token,
                    task: "parsing control statement",
                }),
            }
        } else {
            Err(Error::UnexpectedEndOfFile)
        }
    }

    fn parse_decl_variable(&mut self) -> Result<(Span, decl::Variable), Error> {
        let first = self
            .expect_consume(TokenKind::KwVar, "parsing variable declaration")?
            .span;
        let symbol = self.expect_consume(TokenKind::Identifier, "parsing variableiable name")?;
        let typ: Option<TypeRef> = if self.check(TokenKind::Colon) {
            self.consume()?;
            Some(self.parse_type_ref()?)
        } else {
            None
        };

        self.expect_consume(TokenKind::OpEq, "parsing variable declaration statement")?;

        let expr = self.parse_expr_ref()?;
        let last = self.tree.to_expr(expr).span;

        Ok((
            span![first.begin, last.end],
            decl::Variable {
                symbol: symbol,
                typ: typ,
                expr: expr,
            },
        ))
    }

    fn parse_decl_function(&mut self) -> Result<(Span, decl::Function), Error> {
        let first = self
            .expect_consume(TokenKind::KwFn, "parsing function declaration")?
            .span;
        let symbol = self.expect_consume(TokenKind::Identifier, "parsing function name")?;
        self.expect_consume(TokenKind::ParenOpen, "parsing function parameter list")?;

        let mut params: Vec<(Token, TypeRef)> = vec![];
        while !self.check(TokenKind::ParenClose) {
            if params.len() != 0 {
                self.expect_consume(TokenKind::Comma, "terminating function parameter")?;
            }
            let symbol =
                self.expect_consume(TokenKind::Identifier, "parsing function parameter name")?;
            self.expect_consume(TokenKind::Colon, "parsing function parameter type")?;
            params.push((symbol, self.parse_type_ref()?));
        }

        self.expect_consume(TokenKind::ParenClose, "terminating function parameter list")?;
        self.expect_consume(TokenKind::Arrow, "parsing function return type")?;

        let result = self.parse_type_ref()?;
        let body = self.parse_body()?;

        Ok((
            span![first.begin, body.0.end],
            decl::Function {
                symbol: symbol,
                params: params,
                result: result,
                body: body.1,
            },
        ))
    }

    fn parse_decl_use(&mut self) -> Result<(Span, decl::Use), Error> {
        todo!()
    }

    fn parse_decl_type(&mut self) -> Result<(Span, decl::Type), Error> {
        let begin = self.expect_consume(TokenKind::KwType, "parsing type declaration")?;
        let symbol = self.expect_consume(TokenKind::Identifier, "parsing type name")?;

        self.expect_consume(TokenKind::OpEq, "parsing type declaration")?;

        let typ = self.parse_type_ref()?;
        let end = self.tree.to_type(typ);

        Ok((
            span![begin.span.begin, end.span.end],
            decl::Type {
                symbol: symbol,
                typ: typ,
            },
        ))
    }

    fn parse_decl_const(&mut self) -> Result<(Span, decl::Const), Error> {
        let begin = self.expect_consume(TokenKind::KwConst, "parsing constant declaration")?;
        let symbol = self.expect_consume(TokenKind::Identifier, "parsing constant name")?;

        self.expect_consume(TokenKind::OpEq, "parsing constant declaration")?;

        let expr = self.parse_expr_ref()?;
        let end = self.tree.to_expr(expr);

        Ok((
            span![begin.span.begin, end.span.end],
            decl::Const {
                symbol: symbol,
                expr: expr,
            },
        ))
    }

    fn parse_decl(&mut self) -> Result<(Span, Decl), Error> {
        if let Ok(token) = self.peek() {
            match token.kind {
                TokenKind::KwVar => self
                    .parse_decl_variable()
                    .map(|(span, var)| (span, Decl::Variable(var))),
                TokenKind::KwFn => self
                    .parse_decl_function()
                    .map(|(span, func)| (span, Decl::Function(func))),
                TokenKind::KwUse => self
                    .parse_decl_use()
                    .map(|(span, using)| (span, Decl::Use(using))),
                TokenKind::KwType => self
                    .parse_decl_type()
                    .map(|(span, typ)| (span, Decl::Type(typ))),
                TokenKind::KwConst => self
                    .parse_decl_const()
                    .map(|(span, konst)| (span, Decl::Const(konst))),
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
                | TokenKind::KwFor => self.parse_control().map(|(span, ctrl)| Stmt {
                    span: span,
                    kind: StmtKind::Control(ctrl),
                }),
                TokenKind::KwVar
                | TokenKind::KwFn
                | TokenKind::KwUse
                | TokenKind::KwType
                | TokenKind::KwConst => self.parse_decl().map(|(span, decl)| Stmt {
                    span: span,
                    kind: StmtKind::Decl(decl),
                }),
                _ => Err(Error::UnexpectedToken {
                    token: token,
                    task: "parsing statement",
                }),
            }
        } else {
            Err(Error::UnexpectedEndOfFile)
        }
    }

    fn parse_stmt_ref(&mut self) -> Result<StmtRef, Error> {
        self.parse_stmt().map(|stmt| self.tree.stmt(stmt))
    }

    pub fn parse(&mut self) -> Result<Vec<StmtRef>, Error> {
        let mut ast: Vec<StmtRef> = vec![];
        loop {
            let stmt = self.parse_stmt_ref()?;
            ast.push(stmt);
            if self.check(TokenKind::EndOfFile) {
                break Ok(ast);
            }
        }
    }
}
