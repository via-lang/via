/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

mod attr;
mod control;
mod decl;
mod expr;
mod stmt;
mod ty;

pub mod prelude {
    pub use crate::context::Context;
    pub use crate::error::{Error, ErrorKind, Result};
    pub use viac_ast::extra::{Body, NodeList, Param};
    pub use viac_ast::node::{Ast, Node};
    pub use viac_lexer::token::{
        Token,
        TokenKind::{self, *},
    };
    pub use viac_source::source::Source;
    pub use viac_source::span;
}

use prelude::*;
use viac_ast::stmt::Stmt;

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

    pub(super) fn parse_body<F, T>(&mut self, mut parse: F) -> Result<NodeList<T>>
    where
        F: FnMut(&mut Self) -> Result<Node<T>>,
        T: Ast,
    {
        let first = self.expect_consume(BraceOpen)?;
        let mut body = vec![];

        while !self.check(BraceClose) {
            let node = parse(self)?;
            body.push(node);
        }

        let last = self.expect_consume(BraceClose)?;
        Ok(NodeList {
            list: body,
            span: span![first.span.begin, last.span.end],
        })
    }

    pub(super) fn parse_list<F, T>(
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
            if !self.optional(Comma) {
                break;
            }
        }

        let last = self.expect_consume(brackets.1)?;
        Ok(NodeList {
            list: body,
            span: span![first.span.begin, last.span.end],
        })
    }

    pub(super) fn parse_param(&mut self) -> Result<Node<Param>> {
        self.with_context(Context::Param, |p| {
            let name = p.expect_consume(Identifier)?;
            p.expect_consume(Colon)?;
            let ty = p.parse_param_ty()?;
            let last = ty.span;

            Ok(Node {
                node: Param {
                    name,
                    ty: ty.into(),
                },
                span: span![name.span.begin, last.end],
            })
        })
    }

    fn parse(&mut self) -> Result<Vec<Node<Stmt>>> {
        let mut ast = vec![];
        loop {
            if self.check(EndOfFile) {
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
