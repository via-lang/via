/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::TokenKind;
use phf::phf_map;

pub const SYMBOLS: phf::Map<&'static str, TokenKind> = phf_map! {
    "(" => TokenKind::ParenOpen,
    ")" => TokenKind::ParenClose,
    "[" => TokenKind::BracketOpen,
    "]" => TokenKind::BracketClose,
    "{" => TokenKind::BraceOpen,
    "}" => TokenKind::BraceClose,
    "." => TokenKind::Period,
    "," => TokenKind::Comma,
    ";" => TokenKind::Semicolon,
    ":" => TokenKind::Colon,
    "::" => TokenKind::ColonColon,
    "->" => TokenKind::Arrow,
    "?" => TokenKind::Question,
    "+" => TokenKind::OpPlus,
    "-" => TokenKind::OpMinus,
    "*" => TokenKind::OpStar,
    "/" => TokenKind::OpSlash,
    "**" => TokenKind::OpStarStar,
    "%" => TokenKind::OpPercent,
    "&" => TokenKind::OpAmp,
    "~" => TokenKind::OpTilde,
    "^" => TokenKind::OpCaret,
    "|" => TokenKind::OpPipe,
    "<<" => TokenKind::OpShl,
    ">>" => TokenKind::OpShr,
    "!" => TokenKind::OpBang,
    "++" => TokenKind::OpPlusPlus,
    "--" => TokenKind::OpMinusMinus,
    "&&" => TokenKind::OpAmpAmp,
    "||" => TokenKind::OpPipePipe,
    "<" => TokenKind::OpLt,
    ">" => TokenKind::OpGt,
    ".." => TokenKind::OpDotDot,
    "=" => TokenKind::OpEq,
    "==" => TokenKind::OpEqEq,
    "+=" => TokenKind::OpPlusEq,
    "*=" => TokenKind::OpStarEq,
    "/=" => TokenKind::OpSlashEq,
    "**=" => TokenKind::OpStarStarEq,
    "%=" => TokenKind::OpPercentEq,
    "&=" => TokenKind::OpAmpEq,
    "^=" => TokenKind::OpCaretEq,
    "|=" => TokenKind::OpPipeEq,
    "<<=" => TokenKind::OpShlEq,
    ">>=" => TokenKind::OpShrEq,
    "!=" => TokenKind::OpBangEq,
    "<=" => TokenKind::OpLtEq,
    ">=" => TokenKind::OpGtEq,
};
