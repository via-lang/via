use crate::source::SourceSpan;

#[derive(Debug)]
pub enum TyKind {
    None,
    Bool,
    Int,
    Float,
}

#[derive(Debug)]
pub struct Ty {
    pub kind: TyKind,
    pub span: SourceSpan,
}
