use crate::{
    clinic::{Diagnostic, Severity},
    source::SourceSpan,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnexpectedEndOfFile(SourceSpan),
    UnexpectedToken(SourceSpan),
    UnterminatedStringLiteral {
        literal: SourceSpan,
        quote: SourceSpan,
    },
}

impl Diagnostic for Error {
    fn severity(&self) -> Severity {
        Severity::Error
    }
}
