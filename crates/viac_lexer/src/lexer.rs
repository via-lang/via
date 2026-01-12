/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::keyword::KEYWORD_LIST;
use crate::symbol::SYMBOL_LIST;
use crate::token::{Token, TokenKind};
use unicode_ident::*;
use viac_source::source::Source;
use viac_source::span;

pub struct Lexer<'a> {
    source: &'a Source,
    position: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Source) -> Self {
        Self {
            source,
            position: 0,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source.0[(self.position as usize)..].chars().next()
    }

    fn peek_ahead(&self, ahead: u32) -> Option<char> {
        self.source.0[(self.position as usize)..]
            .chars()
            .nth(ahead as usize)
    }

    #[allow(dead_code)]
    fn check(&self, ch: char) -> bool {
        self.peek().is_some_and(|c| c == ch)
    }

    fn check_ahead(&self, ch: char, ahead: u32) -> bool {
        self.peek_ahead(ahead).is_some_and(|c| c == ch)
    }

    fn consume(&mut self) -> Option<char> {
        let chr = self.peek()?;
        self.position += chr.len_utf8() as u32;
        Some(chr)
    }

    fn consume_while(&mut self, mut f: impl FnMut(char) -> bool) {
        while let Some(ch) = self.peek() {
            if !f(ch) {
                break;
            }
            self.position += ch.len_utf8() as u32;
        }
    }

    fn read_number(&mut self) -> Token {
        let begin = self.position;
        let mut kind = TokenKind::LitInt;

        self.consume_while(|ch| match ch {
            '.' if kind != TokenKind::LitFloat => {
                kind = TokenKind::LitFloat;
                true
            }
            _ => ch.is_ascii_digit(),
        });

        Token::new(kind, span![begin, self.position])
    }

    fn read_xint(&mut self) -> Token {
        let begin = self.position;

        self.consume(); // 0
        self.consume(); // x
        self.consume_while(|ch| ch.is_ascii_hexdigit());

        Token::new(TokenKind::LitXint, span![begin, self.position])
    }

    fn read_bint(&mut self) -> Token {
        let begin = self.position;

        self.consume(); // 0
        self.consume(); // b
        self.consume_while(|ch| ch == '0' || ch == '1');

        Token::new(TokenKind::LitBint, span![begin, self.position])
    }

    fn read_ident(&mut self) -> Token {
        let begin = self.position;

        self.consume();
        self.consume_while(|ch| ch == '_' || is_xid_continue(ch));

        let span = span![begin, self.position];
        let lexeme = self.source.slice(span);

        Token::new(
            KEYWORD_LIST
                .get(&lexeme)
                .unwrap_or(&TokenKind::Identifier)
                .clone(),
            span,
        )
    }

    fn read_string(&mut self) -> Token {
        let begin = self.position;

        self.consume(); // opening "
        self.consume_while(|ch| ch != '"');

        if self.peek() == Some('"') {
            self.consume();
        }
        Token::new(TokenKind::LitString, span![begin, self.position])
    }

    fn read_format_string(&mut self) -> Token {
        todo!()
    }

    fn read_binary_string(&mut self) -> Token {
        todo!()
    }

    fn read_raw_string(&mut self) -> Token {
        todo!()
    }

    fn read_operator(&mut self) -> Token {
        let begin = self.position;
        let rest = &self.source.0[begin as usize..];

        let mut best: Option<(&str, TokenKind)> = None;

        for (lexeme, kind) in SYMBOL_LIST.into_iter() {
            if rest.starts_with(*lexeme) {
                match best {
                    Some((best_lexeme, _)) if best_lexeme.len() >= lexeme.len() => {}
                    _ => best = Some((lexeme, kind.clone())),
                }
            }
        }

        if let Some((lexeme, kind)) = best {
            self.position += lexeme.len() as u32;
            return Token::new(kind, span![begin, self.position]);
        }

        self.position += 1;
        Token::new(TokenKind::Illegal, span![begin, self.position])
    }

    fn next_token(&mut self) -> Token {
        self.consume_while(|ch| ch.is_whitespace());

        let begin = self.position;

        match self.peek() {
            Some('0') if self.check_ahead('x', 1) => self.read_xint(),
            Some('0') if self.check_ahead('b', 1) => self.read_bint(),
            Some('f') if self.check_ahead('"', 1) => self.read_format_string(),
            Some('b') if self.check_ahead('"', 1) => self.read_binary_string(),
            Some('r') if self.check_ahead('"', 1) => self.read_raw_string(),
            Some('"') => self.read_string(),
            Some(ch) if is_xid_start(ch) => self.read_ident(),
            Some(ch) if ch.is_ascii_digit() => self.read_number(),
            Some(_) => self.read_operator(),
            None => Token::new(TokenKind::EndOfFile, span![begin, self.position]),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];
        loop {
            let token = self.next_token();
            let kind = token.kind;
            tokens.push(token);
            if kind == TokenKind::EndOfFile {
                break tokens;
            }
        }
    }
}

pub fn tokenize(source: &Source) -> Vec<Token> {
    Lexer {
        source,
        position: 0,
    }
    .tokenize()
}
