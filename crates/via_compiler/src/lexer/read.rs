use unicode_ident::*;

use super::{
    Lexer,
    token::{
        Base, Token,
        TokenKind::{self, *},
    },
};
use crate::source::SourceSpan;

impl Lexer {
    pub(crate) fn read_ident(&mut self) -> Token {
        let start = self.pos;
        self.bump();
        self.eat_while(|c| c == '_' || is_xid_continue(c));

        let span = SourceSpan::new(start, self.pos);
        let text = self.src.read_span(&span);
        let kind = TokenKind::from_keyword(text).unwrap_or(Ident(text.to_owned()));

        Token { kind, span }
    }

    pub(crate) fn read_number(&mut self) -> Token {
        let start = self.pos;

        let base = if self.eat_str("0x") {
            self.eat_while(|c| c.is_ascii_hexdigit());
            Base::Hex
        } else if self.eat_str("0b") {
            self.eat_while(|c| c == '0' || c == '1');
            Base::Binary
        } else {
            self.eat_while(|c| c.is_ascii_digit());
            Base::Decimal
        };

        let is_float = if base == Base::Decimal {
            match (self.peek(), self.peek_n(1)) {
                (Some('.'), Some(c)) if c.is_ascii_digit() => {
                    self.bump(); // consume '.'
                    self.eat_while(|c| c.is_ascii_digit());
                    true
                }
                _ => false,
            }
        } else {
            false
        };

        let span = SourceSpan::new(start, self.pos);
        let text = self.src.read_span(&span);

        let kind = if is_float {
            text.parse::<f64>().map(NumLit).unwrap_or(Illegal)
        } else {
            text.parse::<u128>()
                .map(|value| IntLit { value, base })
                .unwrap_or(Illegal)
        };

        Token { kind, span }
    }

    pub(crate) fn read_string(&mut self) -> Token {
        let start = self.pos;
        self.bump(); // opening "

        let inner = self.pos;
        self.eat_while(|c| c != '"');

        let span = SourceSpan::new(inner, self.pos);
        let literal = self.src.read_span(&span).to_string();

        let terminated = self.eat('"');

        Token {
            kind: StrLit {
                literal,
                terminated,
            },
            span: SourceSpan::new(start, self.pos),
        }
    }

    pub(crate) fn read_operator(&mut self) -> Token {
        let start = self.pos;
        let rest = self.remaining();

        for len in (1..=rest.len()).rev() {
            if let Some(kind) = TokenKind::from_operator(&rest[..len]) {
                self.advance(len);
                return Token {
                    kind,
                    span: SourceSpan::new(start, self.pos),
                };
            }
        }

        self.bump();
        Token {
            kind: Illegal,
            span: SourceSpan::new(start, self.pos),
        }
    }
}
