/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::context::Context;
use crate::compiler::lexer::token::{Token, TokenKind};

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedEndOfFile,
    UnexpectedToken {
        expected: Vec<TokenKind>,
        got: Token,
    },
}

impl Into<Error> for ErrorKind {
    fn into(self) -> Error {
        Error {
            contexts: Vec::new(),
            kind: self,
        }
    }
}

#[derive(Debug)]
pub struct Error {
    pub contexts: Vec<Context>,
    pub kind: ErrorKind,
}

pub type Result<T> = std::result::Result<T, Error>;
