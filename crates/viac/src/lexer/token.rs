use strum::IntoStaticStr;

use crate::source::SourceSpan;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Base {
    Binary = 2,
    Decimal = 10,
    Hex = 16,
}

type RsString = String;

#[derive(via_macros::Token, IntoStaticStr, Debug, Clone, PartialEq)]
#[token_kind(u8)]
pub enum TokenKind {
    EndOfFile,
    Illegal,
    Int {
        value: u128,
        base: Base,
    },
    Float(f64),
    String {
        literal: RsString,
        terminated: bool,
    },
    Ident(RsString),
    KwVar,
    KwLet,
    KwMut,
    KwConst,
    KwFn,
    KwWhile,
    KwFor,
    KwIf,
    KwIn,
    KwElse,
    KwDo,
    KwBreak,
    KwContinue,
    KwReturn,
    KwRaise,
    KwAs,
    KwImport,
    KwType,
    KwTrue,
    KwFalse,
    KwNone,
    KwBool,
    KwInt,
    KwFloat,
    KwString,
    Dot,
    Comma,
    Semi,
    Col,
    ColCol,
    Arrow,
    Quest,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    #[prec(7)]
    Plus,

    #[prec(7)]
    Minus,

    #[prec(8)]
    Star,

    #[prec(8)]
    Slash,

    #[prec(9)]
    StarStar,

    #[prec(8)]
    Percent,

    #[prec(3)]
    Amp,
    Tilde,

    #[prec(4)]
    Caret,

    #[prec(5)]
    Pipe,

    #[prec(6)]
    LtLt,

    #[prec(6)]
    GtGt,
    Hash,
    Bang,
    Quote,

    #[prec(2)]
    Lt,

    #[prec(2)]
    Gt,
    DotDot,

    #[prec(1)]
    AmpAmp,

    #[prec(0)]
    PipePipe,
    Eq,

    #[prec(2)]
    EqEq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    StarStarEq,
    PercentEq,
    AmpEq,
    CaretEq,
    PipeEq,
    LtLtEq,
    GtGtEq,

    #[prec(2)]
    BangEq,

    #[prec(2)]
    LtEq,

    #[prec(2)]
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
