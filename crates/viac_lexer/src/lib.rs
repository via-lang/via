/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

mod cursor;
mod helpers;
pub mod keyword;
pub mod operator;
mod read;
pub mod token;
mod trivia;

use std::rc::Rc;
use token::{Token, TokenKind};
use unicode_ident::*;
use viac_source::Source;
use viac_source::span;

pub struct Lexer {
    src: Rc<Source>,
    pos: u32,
}

impl Lexer {
    pub fn new(src: &Rc<Source>) -> Self {
        Self {
            src: src.clone(),
            pos: 0,
        }
    }

    fn next_token(&mut self) -> Token {
        self.skip_trivia();

        let start = self.pos;

        match self.peek() {
            Some('"') => self.read_string(),
            Some(c) if c == '_' || is_xid_start(c) => self.read_ident(),
            Some(c) if c.is_ascii_digit() => self.read_number(),
            Some(_) => self.read_operator(),
            None => Token {
                kind: TokenKind::EndOfFile,
                span: span![start, start],
            },
        }
    }

    fn tokenize(&mut self) -> Rc<[Token]> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let eof = tok.kind == TokenKind::EndOfFile;
            tokens.push(tok);
            if eof {
                break;
            }
        }
        Rc::from(tokens)
    }
}

pub fn tokenize(src: &Rc<Source>) -> Rc<[Token]> {
    Lexer::new(&src).tokenize()
}
