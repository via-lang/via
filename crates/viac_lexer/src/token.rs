/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use viac_source::span::Span;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Base {
    Binary = 2,
    Decimal = 10,
    Hex = 16,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    EndOfFile,
    Illegal,

    LitInt(i128, Base),
    LitFloat(f64),
    LitString(String),
    Identifier(String),

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

    OpPlus,
    OpMinus,
    OpStar,
    OpSlash,
    OpStarStar,
    OpPercent,
    OpAmp,
    OpTilde,
    OpCaret,
    OpPipe,
    OpShl,
    OpShr,
    OpHash,
    OpBang,
    OpQuote,
    OpLt,
    OpGt,
    OpDotDot,
    OpAmpAmp,
    OpPipePipe,
    OpEq,
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
    OpBangEq,
    OpLtEq,
    OpGtEq,
}

impl TokenKind {
    pub fn bin_prec(&self) -> Option<u8> {
        use TokenKind::*;
        match &self {
            OpPipePipe => Some(0),
            OpAmpAmp => Some(1),
            OpEqEq | OpBangEq | OpLt | OpLtEq | OpGt | OpGtEq => Some(2),
            OpAmp => Some(3),
            OpCaret => Some(4),
            OpPipe => Some(5),
            OpShl | OpShr => Some(6),
            OpPlus | OpMinus => Some(7),
            OpStar | OpSlash | OpPercent => Some(8),
            OpStarStar => Some(9),
            _ => None,
        }
    }
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
