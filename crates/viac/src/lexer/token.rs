/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::source::span::Span;
use strum::IntoStaticStr;
use via_proc_macros::PrecData;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Base {
    Binary = 2,
    Decimal = 10,
    Hex = 16,
}

#[derive(PrecData, IntoStaticStr, Debug, Clone, PartialEq)]
#[prec_data(u8)]
pub enum TokenKind {
    EndOfFile,
    Illegal,
    Int {
        base: Base,
    },
    Float,
    String {
        terminated: bool,
    },
    Ident,
    KwVar,
    KwMut,
    KwConst,
    KwFn,
    KwMatch,
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
    KwAsync,
    KwAwait,
    KwSpawn,
    KwYield,
    KwAs,
    KwImport,
    KwModule,
    KwStruct,
    KwSelf,
    KwEnum,
    KwUse,
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

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn length(&self) -> usize {
        self.span.length()
    }
}
