/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::Lexer;

impl Lexer {
    #[inline]
    pub(crate) fn remaining(&self) -> &str {
        &self.src.text[self.pos as usize..]
    }

    #[inline]
    pub(crate) fn eof(&self) -> bool {
        self.pos as usize >= self.src.text.len()
    }

    #[inline]
    pub(crate) fn peek(&self) -> Option<char> {
        self.remaining().chars().next()
    }

    #[inline]
    pub(crate) fn peek_n(&self, n: u32) -> Option<char> {
        self.remaining().chars().nth(n as usize)
    }

    #[inline]
    pub(crate) fn starts_with(&self, s: &str) -> bool {
        self.remaining().starts_with(s)
    }

    #[inline]
    pub(crate) fn bump(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8() as u32;
        Some(ch)
    }

    #[inline]
    pub(crate) fn advance(&mut self, n: u32) {
        self.pos += n;
    }
}
