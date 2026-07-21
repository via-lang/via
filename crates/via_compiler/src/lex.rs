use std::sync::Arc;

use unicode_ident::*;

use crate::{
    db::{Db, SourceProgram},
    syntax::SyntaxKind::{self, *},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub kind: SyntaxKind,
    pub len: usize,
}

#[salsa::tracked(debug)]
pub struct TokenStream<'db> {
    #[tracked]
    #[returns(ref)]
    pub tokens: Vec<Token>,
}

struct Lexer<'src> {
    src: &'src str,
    position: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(db: &'src dyn Db, program: SourceProgram) -> Self {
        Self {
            src: program.contents(db),
            position: 0,
        }
    }

    fn remaining(&self) -> Option<&str> {
        self.src.get(self.position as _..)
    }

    fn peek(&self) -> Option<char> {
        self.remaining()?.chars().next()
    }

    fn peek_n(&self, n: u32) -> Option<char> {
        self.remaining()?.chars().nth(n as usize)
    }

    fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;

        self.position += ch.len_utf8();

        Some(ch)
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn read_tab(&mut self) -> Token {
        let start = self.position;
        self.bump();

        while let Some('\t') = self.peek() {
            self.bump();
        }

        Token {
            kind: TAB,
            len: self.position - start,
        }
    }

    fn read_whitespace(&mut self) -> Token {
        let start = self.position;
        self.bump();

        while let Some(' ') = self.peek() {
            self.bump();
        }

        Token {
            kind: WHITESPACE,
            len: self.position - start,
        }
    }

    fn read_comment(&mut self) -> Token {
        let start = self.position;
        self.bump(); // first '/'
        self.bump(); // second '/'

        while let Some(c) = self.peek() {
            if c == '\n' || c == '\r' {
                break;
            }
            self.bump();
        }

        Token {
            kind: COMMENT,
            len: self.position - start,
        }
    }

    fn read_multiline_comment(&mut self) -> Token {
        let start = self.position;
        self.bump(); // Consume '/'
        self.bump(); // Consume '*'

        let mut depth = 1;
        let mut terminated = false;

        while let Some(c) = self.peek() {
            if c == '/' && self.peek_n(1) == Some('*') {
                self.bump(); // '/'
                self.bump(); // '*'
                depth += 1;
            } else if c == '*' && self.peek_n(1) == Some('/') {
                self.bump(); // '*'
                self.bump(); // '/'
                depth -= 1;
                if depth == 0 {
                    terminated = true;
                    break;
                }
            } else {
                self.bump();
            }
        }

        Token {
            kind: if terminated { COMMENT } else { ILLEGAL },
            len: self.position - start,
        }
    }

    fn read_ident(&mut self) -> Token {
        let start = self.position;
        self.bump(); // Consume the xid_start or '_'

        while let Some(c) = self.peek() {
            if is_xid_continue(c) {
                self.bump();
            } else {
                break;
            }
        }

        let len = self.position - start;
        let text = &self.src[start..self.position];

        let kind = SyntaxKind::from_keyword(text).unwrap_or(IDENT);

        Token { kind, len }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        self.bump(); // Consume first digit

        let mut has_dot = false;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.bump();
            } else if c == '.' && !has_dot {
                // Lookahead to prevent consuming range operators like `..` or `..=` as floats
                if let Some(next) = self.peek_n(1)
                    && next == '.'
                {
                    // It's a range sequence (..), stop lexing as float
                    break;
                }

                has_dot = true;

                self.bump();
            } else if c == '_' {
                self.bump(); // Allow numeric separators: 1_000_000
            } else {
                break;
            }
        }

        Token {
            kind: if has_dot { LIT_FLOAT } else { LIT_INT },
            len: self.position - start,
        }
    }

    fn read_string(&mut self) -> Token {
        let start = self.position;
        self.bump(); // Open quote

        let mut escaped = false;
        let mut terminated = false;

        while let Some(c) = self.bump() {
            if escaped {
                escaped = false;
            } else if c == '\\' {
                escaped = true;
            } else if c == '"' {
                terminated = true;
                break;
            }
        }

        Token {
            kind: if terminated { LIT_STRING } else { ILLEGAL },
            len: self.position - start,
        }
    }

    fn read_operator(&mut self) -> Token {
        let start = self.position;
        let rem = self.remaining().unwrap_or("");

        // Check longest multi-character punctuations down to single characters
        let (kind, len) = if rem.starts_with("..=") {
            (DOT_DOT_EQ, 3)
        } else if rem.starts_with("**=") {
            (STAR_STAR_EQ, 3)
        } else if rem.starts_with("<<=") {
            (LT_LT_EQ, 3)
        } else if rem.starts_with(">>=") {
            (GT_GT_EQ, 3)
        } else if rem.starts_with("::") {
            (COLON_COLON, 2)
        } else if rem.starts_with("->") {
            (ARROW, 2)
        } else if rem.starts_with("..") {
            (DOT_DOT, 2)
        } else if rem.starts_with("&&") {
            (AMP_AMP, 2)
        } else if rem.starts_with("||") {
            (PIPE_PIPE, 2)
        } else if rem.starts_with("==") {
            (EQ_EQ, 2)
        } else if rem.starts_with("+=") {
            (PLUS_EQ, 2)
        } else if rem.starts_with("-=") {
            (MINUS_EQ, 2)
        } else if rem.starts_with("*=") {
            (STAR_EQ, 2)
        } else if rem.starts_with("/=") {
            (SLASH_EQ, 2)
        } else if rem.starts_with("**") {
            (STAR_STAR, 2)
        } else if rem.starts_with("%=") {
            (PERCENT_EQ, 2)
        } else if rem.starts_with("&=") {
            (AMP_EQ, 2)
        } else if rem.starts_with("^=") {
            (CARET_EQ, 2)
        } else if rem.starts_with("|=") {
            (PIPE_EQ, 2)
        } else if rem.starts_with("<<") {
            (LT_LT, 2)
        } else if rem.starts_with(">>") {
            (GT_GT, 2)
        } else if rem.starts_with("!=") {
            (BANG_EQ, 2)
        } else if rem.starts_with("<=") {
            (LT_EQ, 2)
        } else if rem.starts_with(">=") {
            (GT_EQ, 2)
        } else if rem.starts_with(".") {
            (DOT, 1)
        } else if rem.starts_with(",") {
            (COMMA, 1)
        } else if rem.starts_with(";") {
            (SEMI, 1)
        } else if rem.starts_with(":") {
            (COLON, 1)
        } else if rem.starts_with("?") {
            (QUESTION, 1)
        } else if rem.starts_with("(") {
            (L_PAREN, 1)
        } else if rem.starts_with(")") {
            (R_PAREN, 1)
        } else if rem.starts_with("[") {
            (L_BRACKET, 1)
        } else if rem.starts_with("]") {
            (R_BRACKET, 1)
        } else if rem.starts_with("{") {
            (L_BRACE, 1)
        } else if rem.starts_with("}") {
            (R_BRACE, 1)
        } else if rem.starts_with("+") {
            (PLUS, 1)
        } else if rem.starts_with("-") {
            (MINUS, 1)
        } else if rem.starts_with("*") {
            (STAR, 1)
        } else if rem.starts_with("/") {
            (SLASH, 1)
        } else if rem.starts_with("%") {
            (PERCENT, 1)
        } else if rem.starts_with("&") {
            (AMP, 1)
        } else if rem.starts_with("~") {
            (TILDE, 1)
        } else if rem.starts_with("^") {
            (CARET, 1)
        } else if rem.starts_with("|") {
            (PIPE, 1)
        } else if rem.starts_with("#") {
            (HASH, 1)
        } else if rem.starts_with("!") {
            (BANG, 1)
        } else if rem.starts_with("\"") {
            (QUOTE, 1)
        } else if rem.starts_with("<") {
            (LT, 1)
        } else if rem.starts_with(">") {
            (GT, 1)
        } else if rem.starts_with("=") {
            (EQ, 1)
        } else {
            self.bump();
            return Token {
                kind: ILLEGAL,
                len: self.position - start,
            };
        };

        self.advance(len);
        Token {
            kind,
            len: self.position - start,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let char = self.peek()?;
        let token = match char {
            // Trivia/layout chunks
            '\t' => self.read_tab(),
            ' ' | '\n' => self.read_whitespace(),

            // Comments
            '/' if self.peek_n(1) == Some('/') => self.read_comment(),
            '/' if self.peek_n(1) == Some('*') => self.read_multiline_comment(),

            // String constants
            '"' => self.read_string(),

            // Identifiers and Keywords
            c if c == '_' || is_xid_start(c) => self.read_ident(),

            // Number literals (Ints, Floats, Hex, etc.)
            c if c.is_ascii_digit() => self.read_number(),

            // If it isn't any of the above, it's either an operator, punctuation,
            // or a completely unrecognized/ILLEGAL character sequence.
            _ => self.read_operator(),
        };

        Some(token)
    }
}

#[salsa::tracked]
pub fn tokenize_program<'db>(db: &'db dyn Db, program: SourceProgram) -> Arc<TokenStream<'db>> {
    let mut lexer = Lexer::new(db, program);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next_token() {
        tokens.push(token);
    }

    Arc::new(TokenStream::new(db, tokens))
}
