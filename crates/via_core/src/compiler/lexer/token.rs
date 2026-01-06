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
    KwType,
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
