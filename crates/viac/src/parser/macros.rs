macro_rules! yes_or_no {
    ($vis:vis enum $name:ident) => {
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
            token => Err(Error::UnexpectedToken(token.span)),
        }
    };
    ($this:expr, $kind:expr) => {
        match $this.consume()? {
            token if $kind == token.kind => Ok(token),
            token => Err(Error::UnexpectedToken(token.span)),
        }
    };
}

pub(super) use check;
pub(super) use expect_one;
pub(super) use optional;
pub(super) use yes_or_no;
