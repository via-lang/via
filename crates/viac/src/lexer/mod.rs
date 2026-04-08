mod cursor;
mod helpers;
mod read;
mod token;
mod trivia;

use unicode_ident::*;

pub use token::*;

use crate::source::{SourceBuf, SourceSpan};

pub struct Lexer {
    src: SourceBuf,
    pos: u32,
}

impl Lexer {
    pub fn new(src: &SourceBuf) -> Self {
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
                span: SourceSpan::new(start, start),
            },
        }
    }

    pub(crate) fn tokenize(&mut self) -> Box<[Token]> {
        let mut tokens = Vec::new();
        loop {
            let tok = self.next_token();
            let eof = tok.kind == TokenKind::EndOfFile;
            tokens.push(tok);
            if eof {
                break;
            }
        }
        Box::from(tokens)
    }
}
