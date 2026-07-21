use via_macros::Operation;

#[derive(Operation, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryOp {
    #[from(MINUS)]
    #[trait_info(name = "Neg", method = "neg")]
    Negate,

    #[from(BANG)]
    #[trait_info(name = "Not", method = "not")]
    Not,

    #[from(TILDE)]
    #[trait_info(name = "BitNot", method = "bit_not")]
    BitNot,
}

#[derive(Operation, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    #[from(PLUS)]
    #[trait_info(name = "Add", method = "add")]
    Add,

    #[from(MINUS)]
    #[trait_info(name = "Sub", method = "sub")]
    Sub,

    #[from(STAR)]
    #[trait_info(name = "Mul", method = "mul")]
    Mul,

    #[from(SLASH)]
    #[trait_info(name = "Div", method = "div")]
    Div,

    #[from(STAR_STAR)]
    #[trait_info(name = "Exp", method = "exp")]
    Exp,

    #[from(PERCENT)]
    #[trait_info(name = "Rem", method = "rem")]
    Rem,

    #[from(AMP_AMP)]
    #[trait_info(name = "And", method = "and")]
    And,

    #[from(PIPE_PIPE)]
    #[trait_info(name = "Or", method = "or")]
    Or,

    #[from(EQ_EQ)]
    #[trait_info(name = "PartialEq", method = "eq")]
    Eq,

    #[from(BANG_EQ)]
    #[trait_info(name = "PartialEq", method = "ne")]
    Ne,

    #[from(AMP)]
    #[trait_info(name = "BitAnd", method = "bit_and")]
    BitAnd,

    #[from(PIPE)]
    #[trait_info(name = "BitOr", method = "bit_or")]
    BitOr,

    #[from(CARET)]
    #[trait_info(name = "BitXor", method = "bit_xor")]
    BitXor,

    #[from(LT_LT)]
    #[trait_info(name = "Shl", method = "shl")]
    Shl,

    #[from(GT_GT)]
    #[trait_info(name = "Shr", method = "shr")]
    Shr,
}
