use super::Lexer;

impl Lexer {
    pub(crate) fn eat(&mut self, ch: char) -> bool {
        (self.peek() == Some(ch)).then(|| self.bump()).is_some()
    }

    pub(crate) fn eat_str(&mut self, s: &str) -> bool {
        self.starts_with(s).then(|| self.advance(s.len())).is_some()
    }

    pub(crate) fn eat_while(&mut self, mut f: impl FnMut(char) -> bool) {
        while let Some(ch) = self.peek()
            && f(ch)
        {
            self.bump();
        }
    }

    pub(crate) fn eat_until(&mut self, target: char) {
        while self.bump() != Some(target) {}
    }
}
