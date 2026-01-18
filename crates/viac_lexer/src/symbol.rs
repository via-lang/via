/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use crate::token::TokenKind::{self, *};
use phf::phf_map;

pub const SYMBOL_LIST: phf::Map<&'static str, TokenKind> = phf_map! {
    "(" => ParenOpen,
    ")" => ParenClose,
    "[" => BracketOpen,
    "]" => BracketClose,
    "{" => BraceOpen,
    "}" => BraceClose,
    "." => Period,
    "," => Comma,
    ";" => Semicolon,
    ":" => Colon,
    "::" => ColonColon,
    "->" => Arrow,
    "?" => Question,
    "+" => OpPlus,
    "-" => OpMinus,
    "*" => OpStar,
    "/" => OpSlash,
    "**" => OpStarStar,
    "%" => OpPercent,
    "&" => OpAmp,
    "~" => OpTilde,
    "^" => OpCaret,
    "|" => OpPipe,
    "<<" => OpShl,
    ">>" => OpShr,
    "#" => OpHash,
    "!" => OpBang,
    "'" => OpQuote,
    "&&" => OpAmpAmp,
    "||" => OpPipePipe,
    "<" => OpLt,
    ">" => OpGt,
    ".." => OpDotDot,
    "=" => OpEq,
    "==" => OpEqEq,
    "+=" => OpPlusEq,
    "*=" => OpStarEq,
    "/=" => OpSlashEq,
    "**=" => OpStarStarEq,
    "%=" => OpPercentEq,
    "&=" => OpAmpEq,
    "^=" => OpCaretEq,
    "|=" => OpPipeEq,
    "<<=" => OpShlEq,
    ">>=" => OpShrEq,
    "!=" => OpBangEq,
    "<=" => OpLtEq,
    ">=" => OpGtEq,
};
