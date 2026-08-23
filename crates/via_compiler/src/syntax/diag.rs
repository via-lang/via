use std::fmt;

use super::SyntaxKind;
use crate::db;

#[derive(Clone)]
pub struct Expected(Vec<SyntaxKind>);

#[derive(Debug, Clone)]
pub enum Diagnostic {
    UnexpectedEof { expected: Expected },
    UnexpectedToken { expected: Expected, got: SyntaxKind },
}

impl Expected {
    pub fn new(list: &[SyntaxKind]) -> Self {
        Self(list.to_vec())
    }
}

impl AsRef<[SyntaxKind]> for Expected {
    fn as_ref(&self) -> &[SyntaxKind] {
        self.0.as_slice()
    }
}

impl fmt::Debug for Expected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.as_slice() {
            [] => write!(f, "nothing"),
            [single] => write!(f, "`{single:?}`"),
            [a, b] => write!(f, "`{a:?}` or `{b:?}`"),
            [all_but_last @ .., last] => {
                let mut list = all_but_last
                    .iter()
                    .map(|k| format!("`{k:?}`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                list.push_str(&format!(", or `{last:?}`")); // Oxford comma
                write!(f, "{list}")
            }
        }
    }
}

impl db::IntoDiagnostic for Diagnostic {
    fn into_diagnostic(self, range: rowan::TextRange) -> db::Diagnostic {
        use {Diagnostic::*, db::Severity::*};

        let (message, severity) = match self {
            UnexpectedEof { expected } => {
                (format!("Expected {expected:?}, reached end-of-file"), Error)
            }
            UnexpectedToken { expected, got } => {
                (format!("Expected {expected:?}, got {got:?}"), Error)
            }
        };

        db::Diagnostic {
            message,
            severity,
            range,
        }
    }
}
