use std::fmt;

#[derive(Debug)]
pub enum ConstValue {
    None,
    Bool(bool),
    Int(i64),
    Float(f64),
}

impl fmt::Display for ConstValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Bool(b) => write!(f, "{b}"),
            Self::Int(i) => write!(f, "{i}"),
            Self::Float(fp) => write!(f, "{fp}"),
        }
    }
}
