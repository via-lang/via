/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Lexer;
use super::keyword::KEYWORD_LIST;
use super::operator::OPERATOR_LIST;
use super::token::{
    Base, Token,
    TokenKind::{self, *},
};
use crate::source::span::span;
use unicode_ident::*;

impl Lexer {
    pub(crate) fn read_ident(&mut self) -> Token {
        let start = self.pos;
        self.bump(); // first char

        self.eat_while(|c| c == '_' || is_xid_continue(c));

        let span = span![start, self.pos];
        let text = self.src.slice(span);
        let kind = KEYWORD_LIST.get(&text).cloned().unwrap_or(Identifier);
        Token { kind, span }
    }

    pub(crate) fn read_number(&mut self) -> Token {
        let start = self.pos;

        if self.eat_str("0x") {
            self.eat_while(|c| c.is_ascii_hexdigit());
            return Token {
                kind: LitInt { base: Base::Hex },
                span: span![start, self.pos],
            };
        }

        if self.eat_str("0b") {
            self.eat_while(|c| c == '0' || c == '1');
            return Token {
                kind: LitInt { base: Base::Binary },
                span: span![start, self.pos],
            };
        }

        self.eat_while(|c| c.is_ascii_digit());

        let is_float = match (self.peek(), self.peek_n(1)) {
            (Some('.'), Some(c)) if c.is_ascii_digit() => {
                self.bump();
                self.eat_while(|c| c.is_ascii_digit());
                true
            }
            _ => false,
        };

        Token {
            kind: if is_float {
                LitFloat
            } else {
                LitInt {
                    base: Base::Decimal,
                }
            },
            span: span![start, self.pos],
        }
    }

    pub(crate) fn read_string(&mut self) -> Token {
        let start = self.pos;
        self.bump(); // opening "

        self.eat_while(|c| c != '"');

        let terminated = self.eat('"');

        Token {
            kind: LitString { terminated },
            span: span![start, self.pos],
        }
    }

    pub(crate) fn read_operator(&mut self) -> Token {
        let start = self.pos;
        let rest = self.remaining();

        let mut best: Option<(&str, TokenKind)> = None;

        for (lexeme, kind) in OPERATOR_LIST.entries() {
            if rest.starts_with(lexeme) {
                if best.as_ref().map_or(true, |(b, _)| lexeme.len() > b.len()) {
                    best = Some((lexeme, kind.clone()));
                }
            }
        }

        if let Some((lexeme, kind)) = best {
            self.advance(lexeme.len() as u32);
            return Token {
                kind,
                span: span![start, self.pos],
            };
        }

        self.bump();
        Token {
            kind: Illegal,
            span: span![start, self.pos],
        }
    }
}
