/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use unicode_ident::*;

use super::{
    Lexer,
    keyword::KEYWORD_LIST,
    operator::OPERATOR_LIST,
    token::{
        Base, Token,
        TokenKind::{self, *},
    },
};
use crate::source::SourceSpan;

impl Lexer {
    pub(crate) fn read_ident(&mut self) -> Token {
        let start = self.pos;
        self.bump(); // first char
        self.eat_while(|c| c == '_' || is_xid_continue(c));

        let span = SourceSpan::new(start, self.pos);
        let kind = KEYWORD_LIST
            .get(self.src.get_span(&span))
            .cloned()
            .unwrap_or(Ident);

        Token { kind, span }
    }

    pub(crate) fn read_number(&mut self) -> Token {
        let start = self.pos;

        if self.eat_str("0x") {
            self.eat_while(|c| c.is_ascii_hexdigit());
            return Token {
                kind: Int { base: Base::Hex },
                span: SourceSpan::new(start, self.pos),
            };
        }

        if self.eat_str("0b") {
            self.eat_while(|c| c == '0' || c == '1');
            return Token {
                kind: Int { base: Base::Binary },
                span: SourceSpan::new(start, self.pos),
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
                Float
            } else {
                Int {
                    base: Base::Decimal,
                }
            },
            span: SourceSpan::new(start, self.pos),
        }
    }

    pub(crate) fn read_string(&mut self) -> Token {
        let start = self.pos;
        self.bump(); // opening "
        self.eat_while(|c| c != '"');

        let terminated = self.eat('"');

        Token {
            kind: String { terminated },
            span: SourceSpan::new(start, self.pos),
        }
    }

    pub(crate) fn read_operator(&mut self) -> Token {
        let start = self.pos;
        let rest = self.remaining();

        let mut best: Option<(&str, TokenKind)> = None;

        for (lexeme, kind) in OPERATOR_LIST.entries() {
            if rest.starts_with(lexeme) && best.as_ref().is_none_or(|(b, _)| lexeme.len() > b.len())
            {
                best = Some((lexeme, kind.clone()));
            }
        }

        if let Some((lexeme, kind)) = best {
            self.advance(lexeme.len());
            return Token {
                kind,
                span: SourceSpan::new(start, self.pos),
            };
        }

        self.bump();
        Token {
            kind: Illegal,
            span: SourceSpan::new(start, self.pos),
        }
    }
}
