/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::Lexer;

impl Lexer {
    pub(crate) fn skip_trivia(&mut self) {
        loop {
            self.eat_while(|c| c.is_whitespace());

            if self.eat_str("//") {
                self.eat_until('\n');
                continue;
            }

            if self.eat_str("/*") {
                while !self.eof() {
                    if self.eat_str("*/") {
                        break;
                    }
                    self.bump();
                }
                continue;
            }
            break;
        }
    }
}
