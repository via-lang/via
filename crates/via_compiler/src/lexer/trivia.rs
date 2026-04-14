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
