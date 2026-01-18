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
use crate::token::{Base, Token, TokenKind::*};
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

    fn remaining(&self) -> &str {
        &self.source.0[self.position as usize..]
    }

    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn peek_ahead(&self, n: usize) -> Option<char> {
        self.remaining().chars().nth(n)
    }

    fn consume(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.position += ch.len_utf8() as u32;
        Some(ch)
    }

    fn consume_while(&mut self, mut f: impl FnMut(&mut Lexer, char) -> bool) {
        while let Some(ch) = self.peek() {
            if !f(self, ch) {
                break;
            }
            self.position += ch.len_utf8() as u32;
        }
    }

    fn read_int(&mut self, base: Base, prefix_len: u32) -> Token {
        let begin = self.position;
        self.position += prefix_len;

        self.consume_while(|_, ch| match base {
            Base::Binary => ch == '0' || ch == '1',
            Base::Decimal => ch.is_ascii_digit(),
            Base::Hex => ch.is_ascii_hexdigit(),
        });

        let span = span![begin, self.position];
        let raw = self.source.slice(span).to_string();
        let digits = &raw[prefix_len as usize..];
        let data = i128::from_str_radix(digits, base as u32).unwrap_or(0);

        Token {
            kind: LitInt(data, base),
            span,
        }
    }

    fn read_decimal_or_float(&mut self) -> Token {
        let begin = self.position;
        let mut is_float = false;

        self.consume_while(|l, ch| match ch {
            '.' if !is_float && l.peek_ahead(1).is_some_and(|c| c.is_ascii_digit()) => {
                is_float = true;
                true
            }
            _ => ch.is_ascii_digit(),
        });

        let span = span![begin, self.position];
        let raw = self.source.slice(span).to_string();

        if is_float {
            let data = raw.parse::<f64>().unwrap_or(0.0);
            Token {
                kind: LitFloat(data),
                span,
            }
        } else {
            let data = raw.parse::<i128>().unwrap_or(0);
            Token {
                kind: LitInt(data, Base::Decimal),
                span,
            }
        }
    }

    fn read_ident(&mut self) -> Token {
        let begin = self.position;
        self.consume(); // first char

        self.consume_while(|_, ch| ch == '_' || is_xid_continue(ch));

        let span = span![begin, self.position];
        let text = self.source.slice(span);

        let kind = KEYWORD_LIST
            .get(&text)
            .cloned()
            .unwrap_or(Identifier(text.to_string()));

        Token { kind, span }
    }

    fn read_string(&mut self) -> Token {
        let begin = self.position;
        self.consume(); // opening "

        let content_start = self.position;

        self.consume_while(|_, ch| ch != '"');

        let value = self.source.0[content_start as usize..self.position as usize].to_string();

        if self.peek() == Some('"') {
            self.consume();
        }

        Token {
            kind: LitString(value),
            span: span![begin, self.position],
        }
    }

    fn read_operator(&mut self) -> Token {
        let begin = self.position;
        let rest = self.remaining();

        let mut best: Option<(&str, _)> = None;

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
            return Token {
                kind,
                span: span![begin, self.position],
            };
        }

        self.consume();
        Token {
            kind: Illegal,
            span: span![begin, self.position],
        }
    }

    fn next_token(&mut self) -> Token {
        self.consume_while(|_, ch| ch.is_whitespace());
        let begin = self.position;

        match self.peek() {
            Some('0') if self.peek_ahead(1) == Some('x') => self.read_int(Base::Hex, 2),
            Some('0') if self.peek_ahead(1) == Some('b') => self.read_int(Base::Binary, 2),
            Some('"') => self.read_string(),
            Some(ch) if ch == '_' || is_xid_start(ch) => self.read_ident(),
            Some(ch) if ch.is_ascii_digit() => self.read_decimal_or_float(),
            Some(_) => self.read_operator(),
            None => Token {
                kind: EndOfFile,
                span: span![begin, begin],
            },
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let eof = tok.kind == EndOfFile;
            tokens.push(tok);
            if eof {
                break;
            }
        }
        tokens
    }
}

pub fn tokenize(source: &Source) -> Vec<Token> {
    Lexer::new(source).tokenize()
}
