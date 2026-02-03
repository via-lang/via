/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    Hir,
    block::BlockId,
    builder::IrBuilder,
    error::{Error, Result},
    instr::{Instr, ValueId},
};
use crate::{
    ast::{expr::Expr, value::Value},
    sema::value::ConstValue,
    source::SourceSpan,
};

impl<'a> IrBuilder<'a> {
    pub(super) fn lower_expr(
        &mut self,
        hir: &mut Hir,
        block: BlockId,
        expr: &'a Expr,
        out: Option<ValueId>,
    ) -> Result<BlockId> {
        let src = self.source.clone();

        let require_out = || match out {
            Some(out) => Ok(out),
            None => Err(Error::ExprIgnored {
                src,
                span: expr.span().to_miette_span(),
            }),
        };

        let instr = match expr {
            Expr::Value(v) => match v {
                Value::None(_) => Instr::Const {
                    value: ConstValue::None,
                    out: require_out()?,
                },
                Value::True(_) => Instr::Const {
                    value: ConstValue::Bool(true),
                    out: require_out()?,
                },
                Value::False(_) => Instr::Const {
                    value: ConstValue::Bool(false),
                    out: require_out()?,
                },
                Value::Integer(int) => Instr::Const {
                    value: ConstValue::Int(int.value),
                    out: require_out()?,
                },
                Value::Float(fp) => Instr::Const {
                    value: ConstValue::Float(fp.value),
                    out: require_out()?,
                },
                Value::String(str) => Instr::Const {
                    value: ConstValue::String(
                        self.source
                            .get_span(SourceSpan::new(str.span.begin + 1, str.span.end - 1))
                            .to_owned(),
                    ),
                    out: require_out()?,
                },
                Value::Range(range) => {
                    let [lhs, rhs] = hir.temp_id.bump::<2>().map(Into::into);

                    self.lower_expr(hir, block, self.ast.get(range.lhs), Some(lhs))?;
                    self.lower_expr(hir, block, self.ast.get(range.rhs), Some(rhs))?;

                    Instr::Range {
                        inclusive: range.inclusive,
                        lhs,
                        rhs,
                        out: require_out()?,
                    }
                }
                Value::Tuple(tuple) => {
                    let values = tuple
                        .exprs
                        .iter()
                        .map(|expr| {
                            let [out] = hir.temp_id.bump::<1>().map(Into::into);
                            let expr = self.ast.get(*expr);
                            self.lower_expr(hir, block, expr, Some(out))?;
                            Ok(out)
                        })
                        .collect::<Result<_>>()?;

                    Instr::Tuple {
                        values,
                        out: require_out()?,
                    }
                }
                Value::Array(tuple) => {
                    let values = tuple
                        .exprs
                        .iter()
                        .map(|expr| {
                            let [out] = hir.temp_id.bump::<1>().map(Into::into);
                            let expr = self.ast.get(*expr);
                            self.lower_expr(hir, block, expr, Some(out))?;
                            Ok(out)
                        })
                        .collect::<Result<_>>()?;

                    Instr::Array {
                        values,
                        out: require_out()?,
                    }
                }
                _ => todo!(),
            },
            Expr::Place(_) => todo!(),
        };

        self.push(hir, block, instr);
        Ok(block)
    }
}
