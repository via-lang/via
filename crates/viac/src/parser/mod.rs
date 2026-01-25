/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

mod attr;
pub mod context;
mod control;
mod decl;
pub mod error;
mod expr;
mod macros;
mod stmt;
mod ty;

#[cfg(test)]
mod test;

use crate::ast::node::Node;
use crate::ast::stmt::Stmt;
use crate::lexer::token::Token;
use crate::source::Source;
use context::Context;
use error::Result;
use std::rc::Rc;

pub(super) mod prelude {
    pub use super::context::Context;
    pub use super::error::{Error, ErrorKind, Result};
    pub use crate::ast::extra::{NodeList, Param};
    pub use crate::ast::node::{Ast, Node};
    pub use crate::lexer::token::{
        Token,
        TokenKind::{self, *},
    };
    pub(super) use crate::source::span::span;

    macro_rules! check_token {
        ($this:expr, $kind:pat_param) => {
            $this.peek().is_ok_and(|token| matches!(token.kind, $kind))
        };
        ($this:expr, $kind:expr) => {
            $this.peek().is_ok_and(|token| token.kind == $kind)
        };
        ($this:expr, $kind:pat_param, $ahead:expr) => {
            $this
                .peek_ahead($ahead)
                .is_ok_and(|token| matches!(token.kind, $kind))
        };
        ($this:expr, $kind:expr, $ahead:expr) => {
            $this
                .peek_ahead($ahead)
                .is_ok_and(|token| token.kind == $kind)
        };
    }

    macro_rules! optional_token {
        ($this:expr, $kind:pat_param) => {
            check_token!($this, $kind)
                .then(|| $this.consume().is_ok())
                .unwrap_or(false)
        };
        ($this:expr, $kind:expr) => {
            check_token!($this, $kind)
                .then(|| $this.consume().is_ok())
                .unwrap_or(false)
        };
    }

    macro_rules! expect_token(
        ($this:expr, $kind:pat_param) => {
            match $this.consume()? {
                token if matches!(&token.kind, $kind) => Ok(token),
                token => $this.error::<Token>(ErrorKind::UnexpectedToken {
                    exp: vec![].into(),
                    got: token,
                }),
            }
        };
        ($this:expr, $kind:expr) => {
            match $this.consume()? {
                token if $kind == token.kind => Ok(token),
                token => $this.error::<Token>(ErrorKind::UnexpectedToken {
                    exp: vec![].into(),
                    got: token,
                }),
            }
        }
    );

    pub(super) use check_token;
    pub(super) use expect_token;
    pub(super) use optional_token;
}

use prelude::*;

pub struct Parser {
    src: Rc<Source>,
    toks: Rc<[Token]>,
    pos: usize,
    ctxts: Vec<Context>,
}

impl Parser {
    pub fn new(src: &Rc<Source>, toks: &Rc<[Token]>) -> Self {
        Self {
            src: src.clone(),
            toks: toks.clone(),
            pos: 0,
            ctxts: vec![],
        }
    }

    fn push_context(&mut self, ctx: Context) {
        self.ctxts.push(ctx);
    }

    fn pop_context(&mut self) {
        self.ctxts.pop();
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
            ctxts: self.ctxts.clone(),
        })
    }

    fn peek(&self) -> Result<Token> {
        self.toks.get(self.pos).cloned().ok_or_else(|| {
            self.error::<Token>(ErrorKind::UnexpectedEndOfFile)
                .err()
                .unwrap()
        })
    }

    fn peek_ahead(&self, ahead: u32) -> Result<Token> {
        self.toks
            .get(self.pos + ahead as usize)
            .cloned()
            .ok_or_else(|| {
                self.error::<Token>(ErrorKind::UnexpectedEndOfFile)
                    .err()
                    .unwrap()
            })
    }

    fn consume(&mut self) -> Result<Token> {
        self.peek().map(|token| {
            self.pos += 1;
            token
        })
    }

    pub(super) fn parse_body<F, T>(&mut self, mut parse: F) -> Result<NodeList<T>>
    where
        F: FnMut(&mut Self) -> Result<Node<T>>,
        T: Ast,
    {
        let first = expect_token!(self, BraceOpen)?;
        let mut body = vec![];

        while !check_token!(self, BraceClose) {
            let node = parse(self)?;
            body.push(node);
        }

        let last = expect_token!(self, BraceClose)?;
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
        let first = expect_token!(self, brackets.0)?;
        let mut body = vec![];

        while !check_token!(self, brackets.1) {
            let node = parse(self)?;
            body.push(node);
            if !optional_token!(self, Comma) {
                break;
            }
        }

        let last = expect_token!(self, brackets.1)?;
        Ok(NodeList {
            list: body,
            span: span![first.span.begin, last.span.end],
        })
    }

    pub(super) fn parse_param(&mut self) -> Result<Node<Param>> {
        self.with_context(Context::Param, |p| {
            let name = expect_token!(p, Identifier)?;
            let first = name.span;

            expect_token!(p, Colon)?;

            let ty = p.parse_param_ty()?;
            let last = ty.span;

            Ok(Node {
                node: Param {
                    name,
                    ty: ty.into(),
                },
                span: span![first.begin, last.end],
                attrs: vec![],
            })
        })
    }

    pub(crate) fn parse(&mut self) -> Result<Rc<[Node<Stmt>]>> {
        let mut ast = vec![];
        loop {
            if check_token!(self, EndOfFile) {
                break Ok(Rc::from(ast));
            }
            let stmt = self.parse_stmt()?;
            ast.push(stmt);
        }
    }
}

pub fn parse(src: &Rc<Source>, toks: &Rc<[Token]>) -> Result<Rc<[Node<Stmt>]>> {
    Parser::new(&src, &toks).parse()
}
