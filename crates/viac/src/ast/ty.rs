use crate::source::SourceSpan;

#[derive(Debug)]
pub enum TyKind {
    // ()
    Unit,
    // bool
    Bool,
    // int
    Int,
    // float
    Float,
}

#[derive(Debug)]
pub struct Ty {
    pub kind: TyKind,
    pub span: SourceSpan,
}
