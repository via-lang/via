/* ================================================ **
**           The via Programming Language           **
** ------------------------------------------------ **
**        Copyright (C) XnLogicaL 2024-2026         **
**           Licensed under GNU GPL v3.0            **
** ------------------------------------------------ **
**         https://github.com/via-lang/via          **
** ================================================ */

use super::{
    block::{Block, BlockItem},
    builder::IrBuilder,
    env::Env,
    error::{Error, Result},
    instr::{Instr, ValueId},
};
use crate::{
    ast::{
        expr::{Expr, ExprId},
        value::Value,
    },
    sema::value::ConstValue,
    source::SourceSpan,
};

impl IrBuilder<'_> {
    pub(super) fn lower_expr(
        &mut self,
        env: &mut Env,
        block: &mut Block,
        expr: ExprId,
        out: Option<ValueId>,
    ) -> Result<()> {
        let expr = self.ast.get(expr);
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
                    let [lhs, rhs] = env.value_id.next::<2>();

                    self.lower_expr(env, block, range.lhs, Some(lhs));
                    self.lower_expr(env, block, range.rhs, Some(rhs));

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
                            let [out] = env.value_id.next::<1>();
                            self.lower_expr(env, block, *expr, Some(out));
                            out
                        })
                        .collect();
                    Instr::Tuple {
                        values,
                        out: require_out()?,
                    }
                }
                Value::Array(array) => {
                    let values = array
                        .exprs
                        .iter()
                        .map(|expr| {
                            let [out] = env.value_id.next::<1>();
                            self.lower_expr(env, block, *expr, Some(out));
                            out
                        })
                        .collect();
                    Instr::Array {
                        values,
                        out: require_out()?,
                    }
                }
                _ => todo!(),
            },
            Expr::Place(_) => todo!(),
        };
        block.items.push(BlockItem::Instr(instr));
        Ok(())
    }
}
