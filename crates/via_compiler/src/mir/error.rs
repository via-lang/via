use crate::source::SourceSpan;

#[derive(Debug)]
pub enum Error {
    UndefinedSymbol { span: SourceSpan, symbol: String },
    ExprIgnored(SourceSpan),
    UnreachableStatement(SourceSpan),
    RogueControlStatement(SourceSpan),
}
