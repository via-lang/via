/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

mod body;
pub mod context;
pub mod error;
mod expr;
mod macros;
mod param;
mod stmt;
mod ty;

pub(super) mod prelude {
    pub(super) use super::{
        Parser,
        context::Context,
        error::{Error, Result},
        macros::*,
    };
    pub(super) use crate::{
        ast::Tree,
        lexer::token::TokenKind::{self, *},
        source::{SourceBuf, SourceSpan},
    };
}

use context::Context;
use prelude::*;

use crate::{
    ast::{Id, Tree},
    lexer::token::Token,
    module::compiler::{Compiler, state::Lexed},
    parser::body::Allow,
};

pub struct Parser<'a> {
    src: SourceBuf,
    toks: &'a [Token],
    pos: usize,
    ctxts: Vec<Context>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &SourceBuf, toks: &'a [Token]) -> Self {
        Self {
            src: src.clone(),
            toks,
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

    fn peek(&self) -> Result<Token> {
        self.toks
            .get(self.pos)
            .cloned()
            .ok_or(Error::UnexpectedEndOfFile {})
    }

    #[allow(dead_code)]
    fn peek_ahead(&self, ahead: u32) -> Result<Token> {
        self.toks
            .get(self.pos + ahead as usize)
            .cloned()
            .ok_or(Error::UnexpectedEndOfFile {})
    }

    fn consume(&mut self) -> Result<Token> {
        self.peek().inspect(|_| self.pos += 1)
    }

    pub(super) fn parse_list<F, I>(
        &mut self,
        tree: &mut Tree,
        mut parse: F,
        brackets: (TokenKind, TokenKind),
    ) -> Result<(Vec<I>, SourceSpan)>
    where
        F: FnMut(&mut Self, &mut Tree) -> Result<I::Node>,
        I: Id,
    {
        let first = expect_one!(self, brackets.0)?;
        let mut inner = vec![];

        while !check!(self, brackets.1) {
            let node = parse(self, tree)?;
            let id = tree.insert(node);
            inner.push(id);

            if !optional!(self, Comma) {
                break;
            }
        }

        let last = expect_one!(self, brackets.1)?;
        Ok((inner, SourceSpan::merge(first.span, last.span)))
    }

    pub(crate) fn parse(&mut self) -> Result<Tree> {
        let mut tree = Tree::default();
        loop {
            if check!(self, EndOfFile) {
                break Ok(tree);
            }
            let stmt = self.parse_stmt(&mut tree, Allow::all())?;
            let id = tree.insert(stmt);
            tree.inner.push(id);
        }
    }
}

pub fn parse(c: &Compiler<Lexed>) -> Result<Tree> {
    Parser::new(c.source(), &c.stage().tt).parse()
}
