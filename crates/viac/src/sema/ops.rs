#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryOp {
    Negate,
    LogNot,
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    LogAnd,
    LogOr,
    BitAnd,
    BitOr,
    BitShl,
    BitShr,
}
