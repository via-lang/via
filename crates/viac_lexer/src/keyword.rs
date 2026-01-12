/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::token::TokenKind;
use phf::phf_map;

pub const KEYWORD_LIST: phf::Map<&'static str, TokenKind> = phf_map! {
    "var" => TokenKind::KwVar,
    "mut" => TokenKind::KwMut,
    "const" => TokenKind::KwConst,
    "fn" => TokenKind::KwFn,
    "match" => TokenKind::KwMatch,
    "while" => TokenKind::KwWhile,
    "for" => TokenKind::KwFor,
    "if" => TokenKind::KwIf,
    "in" => TokenKind::KwIn,
    "as" => TokenKind::KwAs,
    "else" => TokenKind::KwElse,
    "do" => TokenKind::KwDo,
    "break" => TokenKind::KwBreak,
    "continue" => TokenKind::KwContinue,
    "return" => TokenKind::KwReturn,
    "raise" => TokenKind::KwRaise,
    "spawn" => TokenKind::KwSpawn,
    "yield" => TokenKind::KwYield,
    "import" => TokenKind::KwImport,
    "struct" => TokenKind::KwStruct,
    "self" => TokenKind::KwSelf,
    "enum" => TokenKind::KwEnum,
    "use" => TokenKind::KwUse,
    "type" => TokenKind::KwType,
    "none" => TokenKind::KwNone,
    "true" => TokenKind::KwTrue,
    "false" => TokenKind::KwFalse,
    "bool" => TokenKind::KwBool,
    "int" => TokenKind::KwInt,
    "float" => TokenKind::KwFloat,
    "string" => TokenKind::KwString,
};
