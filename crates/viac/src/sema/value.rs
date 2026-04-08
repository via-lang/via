use pretty::RcDoc;

#[derive(Debug)]
pub enum ConstValue {
    Unit,
    Bool(bool),
    Int(i64),
    Float(f64),
}

impl ConstValue {
    pub fn to_doc(&self) -> RcDoc {
        match self {
            Self::Unit => RcDoc::text("()"),
            Self::Bool(b) => RcDoc::text(format!("{b}")),
            Self::Int(i) => RcDoc::text(format!("{i}")),
            Self::Float(f) => RcDoc::text(format!("{f}")),
        }
    }
}
