/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Cyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use phf::phf_map;

use super::token::TokenKind::{self, *};

pub const OPERATOR_LIST: phf::Map<&'static str, TokenKind> = phf_map! {
    "(" => LParen,
    ")" => RParen,
    "[" => LBracket,
    "]" => RBracket,
    "{" => LBrace,
    "}" => RBrace,
    "." => Dot,
    "," => Comma,
    ";" => Semi,
    ":" => Col,
    "::" => ColCol,
    "->" => Arrow,
    "?" => Quest,
    "+" => Plus,
    "-" => Minus,
    "*" => Star,
    "/" => Slash,
    "**" => StarStar,
    "%" => Percent,
    "&" => Amp,
    "~" => Tilde,
    "^" => Caret,
    "|" => Pipe,
    "<<" => LtLt,
    ">>" => GtGt,
    "#" => Hash,
    "!" => Bang,
    "'" => Quote,
    "&&" => AmpAmp,
    "||" => PipePipe,
    "<" => Lt,
    ">" => Gt,
    ".." => DotDot,
    "=" => Eq,
    "==" => EqEq,
    "+=" => PlusEq,
    "*=" => StarEq,
    "/=" => SlashEq,
    "**=" => StarStarEq,
    "%=" => PercentEq,
    "&=" => AmpEq,
    "^=" => CaretEq,
    "|=" => PipeEq,
    "<<=" => LtLtEq,
    ">>=" => GtGtEq,
    "!=" => BangEq,
    "<=" => LtEq,
    ">=" => GtEq,
};
