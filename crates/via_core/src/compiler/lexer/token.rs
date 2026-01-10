/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::compiler::source::Span;
use strum::AsRefStr;

#[derive(AsRefStr, Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    EndOfFile,
    Illegal,
    Identifier,

    LitInt,
    LitBint,
    LitXint,
    LitFloat,
    LitString,

    KwVar,
    KwMut,
    KwConst,
    KwFn,
    KwMatch,
    KwWhile,
    KwWhilex,
    KwFor,
    KwIf,
    KwIfx,
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
    OpLt,
    OpGt,
    OpDotDot,
    OpPlusPlus,
    OpMinusMinus,
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
        match &self {
            TokenKind::OpPipePipe => Some(0),
            TokenKind::OpAmpAmp => Some(1),
            TokenKind::OpEqEq
            | TokenKind::OpBangEq
            | TokenKind::OpLt
            | TokenKind::OpLtEq
            | TokenKind::OpGt
            | TokenKind::OpGtEq => Some(2),
            TokenKind::OpAmp => Some(3),
            TokenKind::OpCaret => Some(4),
            TokenKind::OpPipe => Some(5),
            TokenKind::OpShl | TokenKind::OpShr => Some(6),
            TokenKind::OpPlus | TokenKind::OpMinus => Some(7),
            TokenKind::OpStar | TokenKind::OpSlash | TokenKind::OpPercent => Some(8),
            TokenKind::OpStarStar => Some(9),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self {
            kind: kind,
            span: span,
        }
    }

    pub fn length(&self) -> usize {
        self.span.length()
    }
}
