/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

pub mod keyword;
pub mod symbol;
pub mod token;

use keyword::KEYWORD_LIST;
use std::rc::Rc;
use symbol::SYMBOL_LIST;
use token::{Base, Token, TokenKind::*};
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

    fn remaining(&self) -> &str {
        &self.src.text[self.pos as usize..]
    }

    fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    fn peek_ahead(&self, n: usize) -> Option<char> {
        self.remaining().chars().nth(n)
    }

    fn consume(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8() as u32;
        Some(ch)
    }

    fn consume_while(&mut self, mut f: impl FnMut(&mut Lexer, char) -> bool) {
        while let Some(ch) = self.peek() {
            if !f(self, ch) {
                break;
            }
            self.pos += ch.len_utf8() as u32;
        }
    }

    fn read_int(&mut self, base: Base, prefix_len: u32) -> Token {
        let begin = self.pos;
        self.pos += prefix_len;

        self.consume_while(|_, ch| match base {
            Base::Binary => ch == '0' || ch == '1',
            Base::Decimal => ch.is_ascii_digit(),
            Base::Hex => ch.is_ascii_hexdigit(),
        });

        Token {
            kind: LitInt { base },
            span: span![begin, self.pos],
        }
    }

    fn read_decimal_or_float(&mut self) -> Token {
        let begin = self.pos;
        let mut is_float = false;

        self.consume_while(|l, ch| match ch {
            '.' if !is_float && l.peek_ahead(1).is_some_and(|c| c.is_ascii_digit()) => {
                is_float = true;
                true
            }
            _ => ch.is_ascii_digit(),
        });

        let span = span![begin, self.pos];
        if is_float {
            Token {
                kind: LitFloat,
                span,
            }
        } else {
            Token {
                kind: LitInt {
                    base: Base::Decimal,
                },
                span,
            }
        }
    }

    fn read_ident(&mut self) -> Token {
        let begin = self.pos;
        self.consume(); // first char

        self.consume_while(|_, ch| ch == '_' || is_xid_continue(ch));

        let span = span![begin, self.pos];
        let text = self.src.slice(span);
        let kind = KEYWORD_LIST.get(&text).cloned().unwrap_or(Identifier);
        Token { kind, span }
    }

    fn read_string(&mut self) -> Token {
        let begin = self.pos;
        self.consume(); // opening "
        self.consume_while(|_, ch| ch != '"');

        let terminated = if let Some('"') = self.peek() {
            self.consume();
            true
        } else {
            false
        };

        Token {
            kind: LitString { terminated },
            span: span![begin, self.pos],
        }
    }

    fn read_operator(&mut self) -> Token {
        let begin = self.pos;
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
            self.pos += lexeme.len() as u32;
            return Token {
                kind,
                span: span![begin, self.pos],
            };
        }

        self.consume();
        Token {
            kind: Illegal,
            span: span![begin, self.pos],
        }
    }

    fn next_token(&mut self) -> Token {
        self.consume_while(|_, ch| ch.is_whitespace());
        let begin = self.pos;

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

    pub(crate) fn tokenize(&mut self) -> Rc<[Token]> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let eof = tok.kind == EndOfFile;
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
