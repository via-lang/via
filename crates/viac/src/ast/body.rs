/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{expr::ExprId, stmt::StmtId};
use crate::source::SourceSpan;

#[derive(Debug, Clone)]
pub struct Body {
    pub inner: Box<[StmtId]>,
    pub span: SourceSpan,
    pub tail: Option<ExprId>,
}
