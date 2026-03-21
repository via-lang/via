/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod error;
mod expr;
mod macros;
mod stmt;
mod ty;

pub(super) mod prelude {
    pub(super) use super::{
        Parser,
        error::{Error, Result},
        macros::*,
    };
    pub(super) use crate::{ast::Tree, lexer::token::TokenKind::*, source::SourceSpan};
}

use prelude::*;

use crate::lexer::token::Token;

pub struct Parser<'a> {
    toks: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(toks: &'a [Token]) -> Self {
        Self { toks, pos: 0 }
    }

    fn eof(&self) -> Token {
        self.toks
            .last()
            .expect("all token trees must have EOF sentinel token")
            .clone()
    }

    fn peek(&self) -> Result<Token> {
        self.toks
            .get(self.pos)
            .cloned()
            .ok_or(Error::UnexpectedEndOfFile(self.eof().span))
    }

    #[allow(dead_code)]
    fn peek_ahead(&self, ahead: u32) -> Result<Token> {
        self.toks
            .get(self.pos + ahead as usize)
            .cloned()
            .ok_or(Error::UnexpectedEndOfFile(self.eof().span))
    }

    fn consume(&mut self) -> Result<Token> {
        self.peek().inspect(|_| self.pos += 1)
    }

    pub fn parse(&mut self) -> Result<Tree> {
        let mut tree = Tree::default();

        while !check!(self, EndOfFile) {
            let stmt = self.parse_stmt(&mut tree)?;
            let stmt = tree.alloc_stmt(stmt);
            tree.roots.push(stmt);
        }

        Ok(tree)
    }
}
