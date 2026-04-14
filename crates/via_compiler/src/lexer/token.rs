use strum::IntoStaticStr;

use crate::source::SourceSpan;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Base {
    Binary = 2,
    Decimal = 10,
    Hex = 16,
}

#[derive(via_macros::Token, IntoStaticStr, Debug, Clone, PartialEq)]
#[token_kind(u8)]
pub enum TokenKind {
    EndOfFile,
    Illegal,

    IntLit {
        value: u128,
        base: Base,
    },

    NumLit(f64),

    StrLit {
        literal: String,
        terminated: bool,
    },

    Ident(String),

    #[keyword("_")]
    KwPlaceholder,

    #[keyword("var")]
    KwVar,

    #[keyword("let")]
    KwLet,

    #[keyword("mut")]
    KwMut,

    #[keyword("const")]
    KwConst,

    #[keyword("fn")]
    KwFn,

    #[keyword("for")]
    KwFor,

    #[keyword("if")]
    KwIf,

    #[keyword("in")]
    KwIn,

    #[keyword("else")]
    KwElse,

    #[keyword("do")]
    KwDo,

    #[keyword("break")]
    KwBreak,

    #[keyword("continue")]
    KwContinue,

    #[keyword("return")]
    KwReturn,

    #[keyword("raise")]
    KwRaise,

    #[keyword("as")]
    KwAs,

    #[keyword("import")]
    KwImport,

    #[keyword("type")]
    KwType,

    #[keyword("true")]
    KwTrue,

    #[keyword("false")]
    KwFalse,

    #[operator(".")]
    Dot,

    #[operator(",")]
    Comma,

    #[operator(";")]
    Semi,

    #[operator(":")]
    Colon,

    #[operator("::")]
    ColonColon,

    #[operator("->")]
    Arrow,

    #[operator("?")]
    Question,

    #[operator("(")]
    LParen,

    #[operator(")")]
    RParen,

    #[operator("[")]
    LBracket,

    #[operator("]")]
    RBracket,

    #[operator("{")]
    LBrace,

    #[operator("}")]
    RBrace,

    #[prec(7)]
    #[operator("+")]
    Plus,

    #[prec(7)]
    #[operator("-")]
    Minus,

    #[prec(8)]
    #[operator("*")]
    Star,

    #[prec(8)]
    #[operator("/")]
    Slash,

    #[prec(9)]
    #[operator("**")]
    StarStar,

    #[prec(8)]
    #[operator("%")]
    Percent,

    #[prec(3)]
    #[operator("&")]
    Amp,

    #[operator("~")]
    Tilde,

    #[prec(4)]
    #[operator("^")]
    Caret,

    #[prec(5)]
    #[operator("|")]
    Pipe,

    #[prec(6)]
    #[operator("<<")]
    LtLt,

    #[prec(6)]
    #[operator(">>")]
    GtGt,

    #[operator("#")]
    Hash,

    #[operator("!")]
    Bang,

    #[operator("\"")]
    Quote,

    #[prec(2)]
    #[operator("<")]
    Lt,

    #[prec(2)]
    #[operator(">")]
    Gt,

    #[operator("..")]
    DotDot,

    #[operator("..=")]
    DotDotEq,

    #[prec(1)]
    #[operator("&&")]
    AmpAmp,

    #[prec(0)]
    #[operator("||")]
    PipePipe,

    #[operator("=")]
    Eq,

    #[prec(2)]
    #[operator("==")]
    EqEq,

    #[operator("+=")]
    PlusEq,

    #[operator("-=")]
    MinusEq,

    #[operator("*=")]
    StarEq,

    #[operator("/=")]
    SlashEq,

    #[operator("**=")]
    StarStarEq,

    #[operator("%=")]
    PercentEq,

    #[operator("&=")]
    AmpEq,

    #[operator("^=")]
    CaretEq,

    #[operator("|=")]
    PipeEq,

    #[operator("<<=")]
    LtLtEq,

    #[operator(">>=")]
    GtGtEq,

    #[prec(2)]
    #[operator("!=")]
    BangEq,

    #[prec(2)]
    #[operator("<=")]
    LtEq,

    #[prec(2)]
    #[operator(">=")]
    GtEq,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}
