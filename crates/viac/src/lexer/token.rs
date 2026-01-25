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
    LitInt {
        base: Base,
    },
    LitFloat,
    LitString {
        terminated: bool,
    },
    Identifier,
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
    Period,
    Comma,
    Semicolon,
    Colon,
    ColonColon,
    Arrow,
    Question,
    ParenOpen,
    ParenClose,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,

    #[prec(7)]
    OpPlus,

    #[prec(7)]
    OpMinus,

    #[prec(8)]
    OpStar,

    #[prec(8)]
    OpSlash,

    #[prec(9)]
    OpStarStar,

    #[prec(8)]
    OpPercent,

    #[prec(3)]
    OpAmp,
    OpTilde,

    #[prec(4)]
    OpCaret,

    #[prec(5)]
    OpPipe,

    #[prec(6)]
    OpShl,

    #[prec(6)]
    OpShr,
    OpHash,
    OpBang,
    OpQuote,

    #[prec(2)]
    OpLt,

    #[prec(2)]
    OpGt,
    OpDotDot,

    #[prec(1)]
    OpAmpAmp,

    #[prec(0)]
    OpPipePipe,
    OpEq,

    #[prec(2)]
    OpEqEq,
    OpPlusEq,
    OpMinusEq,
    OpStarEq,
    OpSlashEq,
    OpStarStarEq,
    OpPercentEq,
    OpAmpEq,
    OpCaretEq,
    OpPipeEq,
    OpShlEq,
    OpShrEq,

    #[prec(2)]
    OpBangEq,

    #[prec(2)]
    OpLtEq,

    #[prec(2)]
    OpGtEq,
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
