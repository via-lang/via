/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

macro_rules! yes_or_no {
    ($vis:vis $name:ident) => {
        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
        $vis enum $name {
            Yes,
            No,
        }

        impl From<bool> for $name {
            fn from(value: bool) -> Self {
                match value {
                    true => $name::Yes,
                    false => $name::No,
                }
            }
        }

        impl From<$name> for bool {
            fn from(value: $name) -> Self {
                value == $name::Yes
            }
        }
    };
}

macro_rules! check {
    ($this:expr => $kind:pat_param) => {
        $this.peek().is_ok_and(|token| matches!(token.kind, $kind))
    };
    ($this:expr, $kind:expr) => {
        $this.peek().is_ok_and(|token| token.kind == $kind)
    };
    ($this:expr => $kind:pat_param, $ahead:expr) => {
        $this
            .peek_ahead($ahead)
            .is_ok_and(|token| matches!(token.kind, $kind))
    };
    ($this:expr, $kind:expr, $ahead:expr) => {
        $this
            .peek_ahead($ahead)
            .is_ok_and(|token| token.kind == $kind)
    };
}

macro_rules! optional {
    ($this:expr => $kind:pat_param) => {
        check!($this, $kind)
            .then(|| $this.consume().is_ok())
            .unwrap_or(false)
    };
    ($this:expr, $kind:expr) => {
        check!($this, $kind)
            .then(|| $this.consume().is_ok())
            .unwrap_or(false)
    };
}

macro_rules! expect_one {
    ($this:expr => $kind:pat_param) => {
        match $this.consume()? {
            token if matches!(&token.kind, $kind) => Ok(token),
            token => Err(Error::UnexpectedToken {
                span: token.span.into(),
                expected: vec![].into(),
                got: $this.src.get_span(&token.span).to_owned(),
            }),
        }
    };
    ($this:expr, $kind:expr) => {
        match $this.consume()? {
            token if $kind == token.kind => Ok(token),
            token => Err(Error::UnexpectedToken {
                span: token.span.into(),
                expected: vec![].into(),
                got: $this.src.get_span(&token.span).to_owned(),
            }),
        }
    };
}

pub(super) use check;
pub(super) use expect_one;
pub(super) use optional;
pub(super) use yes_or_no;
