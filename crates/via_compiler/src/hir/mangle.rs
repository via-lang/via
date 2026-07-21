use demangle::Demangler;

/// Attempts to demangle a given string.
///
/// # Examples
///
/// ```rs
/// assert_eq!(demangle("_Z3fooIiE"), Ok("foo::<Int>"))
/// ```
pub fn demangle(input: impl AsRef<str>) -> demangle::Result<String> {
    Demangler::new(input.as_ref()).demangle()
}

mod demangle {
    use std::fmt::Write;

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Error {
        /// The input is not ASCII.
        NotAscii,

        /// The input is not mangled - i.e. does not have the discriminant `_Z`.
        NotMangled,

        /// The length of a name does not match the declared length of it, e.g. `4foo`.
        Truncated,

        /// An invalid length prefix given to a name.
        InvalidLength,

        /// Unknown type directive.
        InvalidType,

        /// Unknown command.
        InvalidCommand,

        /// An unterminated block.
        BlockUnclosed,

        /// An empty block.
        BlockEmpty,
    }

    struct Cursor<'a> {
        input: &'a str,
        pos: usize,
    }

    pub struct Demangler<'a> {
        out: String,
        cursor: Cursor<'a>,
    }

    impl<'a> Cursor<'a> {
        fn new(input: &'a str) -> Self {
            Cursor { input, pos: 0 }
        }

        fn rest(&self) -> &'a str {
            &self.input[self.pos..]
        }

        fn peek(&self) -> Option<char> {
            self.rest().chars().next()
        }

        fn eat(&mut self, prefix: &str) -> bool {
            if self.rest().starts_with(prefix) {
                self.pos += prefix.len();
                true
            } else {
                false
            }
        }

        fn eat_n(&mut self, n: usize) -> Option<&'a str> {
            let s = self.rest().get(..n)?;
            self.pos += n;
            Some(s)
        }

        fn eat_digits(&mut self) -> Option<usize> {
            let digit_end = self.rest().find(|c: char| !c.is_ascii_digit())?;
            if digit_end == 0 {
                return None;
            }
            let n = self.rest()[..digit_end].parse().ok()?;
            self.pos += digit_end;
            Some(n)
        }
    }

    impl<'a> Demangler<'a> {
        const MANGLE_START: &'static str = "_Z";
        const CMD_END: char = 'E';
        const CMD_PATH: char = 'P';
        const CMD_GENERIC: char = 'I';

        pub fn new(input: &'a str) -> Self {
            Self {
                out: String::new(),
                cursor: Cursor::new(input),
            }
        }

        pub fn demangle(mut self) -> Result<String> {
            if !self.cursor.input.is_ascii() {
                return Err(Error::NotAscii);
            }

            if !self.cursor.eat(Self::MANGLE_START) {
                return Err(Error::NotMangled);
            }

            self.run_commands_until_end()?;

            if self.cursor.peek().is_some() {
                return Err(Error::BlockUnclosed);
            }

            Ok(self
                .out
                .strip_prefix("::")
                .map(str::to_string)
                .unwrap_or(self.out))
        }

        fn peek_command(&self) -> Option<char> {
            self.cursor.peek().filter(|&c| c != Self::CMD_END)
        }

        fn run_commands_until_end(&mut self) -> Result<()> {
            while let Some(command) = self.peek_command() {
                self.run_command(command)?;
            }
            Ok(())
        }

        fn run_block(
            &mut self,
            tag: char,
            body: impl FnOnce(&mut Self) -> Result<()>,
        ) -> Result<()> {
            if !self.cursor.eat(&tag.to_string()) {
                return Err(Error::InvalidCommand);
            }
            body(self)?;
            if !self.cursor.eat("E") {
                return Err(Error::BlockUnclosed);
            }
            Ok(())
        }

        fn run_command(&mut self, command: char) -> Result<()> {
            match command {
                Self::CMD_PATH => self.run_block(Self::CMD_PATH, Self::path_body),
                Self::CMD_GENERIC => self.run_block(Self::CMD_GENERIC, Self::generic_body),
                '0'..='9' => self.demangle_name(),
                _ => self.demangle_type(),
            }
        }

        fn path_body(&mut self) -> Result<()> {
            self.run_commands_until_end()
        }

        fn generic_body(&mut self) -> Result<()> {
            if self.peek_command().is_none() {
                return Err(Error::BlockEmpty);
            }

            write!(self.out, "::<").unwrap();

            let mut first = true;
            while let Some(command) = self.peek_command() {
                if !first {
                    write!(self.out, ", ").unwrap();
                }
                first = false;
                self.run_command(command)?;
            }

            write!(self.out, ">").unwrap();
            Ok(())
        }

        fn demangle_name(&mut self) -> Result<()> {
            let len = self.cursor.eat_digits().ok_or(Error::InvalidLength)?;
            let name = self.cursor.eat_n(len).ok_or(Error::Truncated)?;

            write!(self.out, "::{name}").unwrap();

            Ok(())
        }

        fn demangle_type(&mut self) -> Result<()> {
            if self.cursor.eat("u") {
                write!(self.out, "()").unwrap();
            } else if self.cursor.eat("b") {
                write!(self.out, "Bool").unwrap();
            } else if self.cursor.eat("i") {
                write!(self.out, "Int").unwrap();
            } else if self.cursor.eat("f") {
                write!(self.out, "Float").unwrap();
            } else if self.cursor.eat("s") {
                write!(self.out, "String").unwrap();
            } else if self.cursor.eat("V") {
                write!(self.out, "[").unwrap();
                self.demangle_type()?;
                write!(self.out, "]").unwrap();
            } else if self.cursor.eat("A") {
                write!(self.out, "[").unwrap();
                self.demangle_type()?;
                write!(self.out, "; ").unwrap();
                self.demangle_type()?;
                write!(self.out, "]").unwrap();
            } else if self.cursor.eat("M") {
                write!(self.out, "# {{").unwrap();
                self.demangle_type()?;
                write!(self.out, ": ").unwrap();
                self.demangle_type()?;
                write!(self.out, "}}").unwrap();
            } else if self.cursor.eat("C") {
                write!(self.out, "{{ ").unwrap();
                if let Some(len) = self.cursor.eat_digits() {
                    write!(self.out, "{len}").unwrap();
                }
                write!(self.out, " }}").unwrap();
            } else {
                return Err(Error::InvalidType);
            };

            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::{demangle, demangle::*};

    #[test]
    fn not_mangled_is_an_error() {
        assert_eq!(demangle("hello"), Err(Error::NotMangled));
    }

    #[test]
    fn not_ascii_is_an_error() {
        assert_eq!(demangle("_Z4😭"), Err(Error::NotAscii));
    }

    #[test]
    fn stray_end_is_an_error() {
        assert_eq!(demangle("_ZE"), Err(Error::BlockUnclosed));
    }

    #[test]
    fn unclosed_path_is_an_error() {
        assert_eq!(demangle("_ZP3foo"), Err(Error::BlockUnclosed));
    }

    #[test]
    fn empty_generic_list_is_an_error() {
        assert_eq!(demangle("_Z3fooIE"), Err(Error::BlockEmpty));
    }

    #[test]
    fn basic() {
        assert_eq!(demangle("_Z3foo"), Ok("foo".to_owned()));
    }

    #[test]
    fn path() {
        assert_eq!(demangle("_ZP3foo3barE"), Ok("foo::bar".to_owned()))
    }

    #[test]
    fn type_unit() {
        assert_eq!(demangle("_Z3fooIuE"), Ok("foo::<()>".to_owned()));
    }

    #[test]
    fn type_bool() {
        assert_eq!(demangle("_Z3fooIbE"), Ok("foo::<Bool>".to_owned()));
    }

    #[test]
    fn type_int() {
        assert_eq!(demangle("_Z3fooIiE"), Ok("foo::<Int>".to_owned()));
    }

    #[test]
    fn type_float() {
        assert_eq!(demangle("_Z3fooIfE"), Ok("foo::<Float>".to_owned()));
    }

    #[test]
    fn type_string() {
        assert_eq!(demangle("_Z3fooIsE"), Ok("foo::<String>".to_owned()));
    }

    #[test]
    fn type_vector() {
        assert_eq!(demangle("_Z3fooIViE"), Ok("foo::<[Int]>".to_owned()));
    }

    #[test]
    fn type_vector_recursive() {
        assert_eq!(demangle("_Z3fooIVViE"), Ok("foo::<[[Int]]>".to_owned()));
    }

    #[test]
    fn type_array() {
        assert_eq!(
            demangle("_Z3fooIAiC5E"),
            Ok("foo::<[Int; { 5 }]>".to_owned())
        );
    }

    #[test]
    fn type_map() {
        assert_eq!(
            demangle("_Z3fooIMisE"),
            Ok("foo::<# {Int: String}>".to_owned())
        )
    }
}
