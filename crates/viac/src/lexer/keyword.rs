/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */
use phf::phf_map;

use super::token::TokenKind::{self, *};

pub const KEYWORD_LIST: phf::Map<&'static str, TokenKind> = phf_map! {
    "var" => KwVar,
    "let" => KwLet,
    "mut" => KwMut,
    "const" => KwConst,
    "fn" => KwFn,
    "while" => KwWhile,
    "for" => KwFor,
    "if" => KwIf,
    "in" => KwIn,
    "as" => KwAs,
    "else" => KwElse,
    "do" => KwDo,
    "break" => KwBreak,
    "continue" => KwContinue,
    "return" => KwReturn,
    "raise" => KwRaise,
    "type" => KwType,
    "none" => KwNone,
    "true" => KwTrue,
    "false" => KwFalse,
    "bool" => KwBool,
    "int" => KwInt,
    "float" => KwFloat,
    "string" => KwString,
};
