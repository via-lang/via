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
    env::Env,
    instr::{Instr, ValueId},
    place::ReadKind,
};
use crate::{
    ast::{expr::Expr, value::Value},
    lexer::token::TokenKind,
    sema::value::ConstValue,
    source::SourceSpan,
};

impl<'a> IrBuilder<'a> {
    pub(super) fn lower_expr(
        &mut self,
        hir: &mut Hir,
        env: &mut Env,
        block: BlockId,
        expr: &'a Expr,
        out: impl Into<ValueId>,
        read_kind: ReadKind,
    ) {
        let out = out.into();

        match expr {
            Expr::Value(v) => match v {
                Value::None(_) => self.push(
                    hir,
                    block,
                    Instr::Const {
                        value: ConstValue::None,
                        out,
                    },
                ),
                Value::True(_) => self.push(
                    hir,
                    block,
                    Instr::Const {
                        value: ConstValue::Bool(true),
                        out,
                    },
                ),
                Value::False(_) => self.push(
                    hir,
                    block,
                    Instr::Const {
                        value: ConstValue::Bool(false),
                        out,
                    },
                ),
                Value::Integer(int) => self.push(
                    hir,
                    block,
                    Instr::Const {
                        value: ConstValue::Int(int.value),
                        out,
                    },
                ),
                Value::Float(fp) => self.push(
                    hir,
                    block,
                    Instr::Const {
                        value: ConstValue::Float(fp.value),
                        out,
                    },
                ),
                Value::String(str) => self.push(
                    hir,
                    block,
                    Instr::Const {
                        value: ConstValue::String(
                            self.source
                                .get_span(&SourceSpan::new(str.span.begin + 1, str.span.end - 1))
                                .to_owned(),
                        ),
                        out,
                    },
                ),
                Value::Range(range) => {
                    let [lhs, rhs] = env.temp_id.bump::<2>();

                    self.lower_expr(
                        hir,
                        env,
                        block,
                        self.ast.get(range.lhs),
                        lhs,
                        ReadKind::Move,
                    );

                    self.lower_expr(
                        hir,
                        env,
                        block,
                        self.ast.get(range.rhs),
                        rhs,
                        ReadKind::Move,
                    );

                    self.push(
                        hir,
                        block,
                        Instr::Range {
                            inclusive: range.inclusive,
                            lhs,
                            rhs,
                            out,
                        },
                    )
                }
                Value::Tuple(tuple) => {
                    let values = tuple
                        .exprs
                        .iter()
                        .map(|expr| {
                            let expr = self.ast.get(*expr);
                            let [out] = env.temp_id.bump::<1>();
                            self.lower_expr(hir, env, block, expr, out, ReadKind::Move);
                            out
                        })
                        .collect::<_>();

                    self.push(hir, block, Instr::Tuple { values, out })
                }
                Value::Array(tuple) => {
                    let values = tuple
                        .exprs
                        .iter()
                        .map(|expr| {
                            let expr = self.ast.get(*expr);
                            let [out] = env.temp_id.bump::<1>();
                            self.lower_expr(hir, env, block, expr, out, ReadKind::Move);
                            out
                        })
                        .collect::<_>();

                    self.push(hir, block, Instr::Array { values, out })
                }
                Value::Map(_map) => todo!(),
                Value::Lambda(lambda) => {
                    let mut inner_env = Env::new();
                    let mut current = self.block(hir);

                    self.push(
                        hir,
                        block,
                        Instr::Closure {
                            block: current,
                            upvals: vec![],
                            out,
                        },
                    );

                    for stmt in &lambda.body.inner {
                        let stmt = self.ast.get(*stmt);
                        current = self.lower_stmt(hir, &mut inner_env, current, stmt);
                    }
                }
                Value::Unary(unary) => {
                    let [value] = env.temp_id.bump::<1>();

                    self.lower_expr(
                        hir,
                        env,
                        block,
                        self.ast.get(unary.expr),
                        value,
                        ReadKind::Move,
                    );

                    self.push(
                        hir,
                        block,
                        match unary.op.kind {
                            TokenKind::Minus => Instr::Negate { value, out },
                            _ => unreachable!(),
                        },
                    );
                }
                Value::Call(call) => {
                    let [callee] = env.temp_id.bump::<1>();
                    let args = call
                        .args
                        .iter()
                        .map(|expr| {
                            let expr = self.ast.get(*expr);
                            let [out] = env.temp_id.bump::<1>();
                            self.lower_expr(hir, env, block, expr, out, ReadKind::Move);
                            out
                        })
                        .collect::<_>();

                    self.lower_expr(
                        hir,
                        env,
                        block,
                        self.ast.get(call.callee),
                        callee,
                        ReadKind::Move,
                    );

                    self.push(
                        hir,
                        block,
                        Instr::Call {
                            callee,
                            args,
                            out: Some(out),
                        },
                    );
                }
                Value::Binary(bin) => {
                    let [lhs, rhs] = env.temp_id.bump::<2>();

                    self.lower_expr(hir, env, block, self.ast.get(bin.lhs), lhs, ReadKind::Move);
                    self.lower_expr(hir, env, block, self.ast.get(bin.rhs), rhs, ReadKind::Move);
                    self.push(
                        hir,
                        block,
                        match bin.op.kind {
                            TokenKind::Plus => Instr::Add { lhs, rhs, out },
                            TokenKind::Minus => Instr::Sub { lhs, rhs, out },
                            TokenKind::Star => Instr::Mul { lhs, rhs, out },
                            TokenKind::Slash => Instr::Div { lhs, rhs, out },
                            TokenKind::StarStar => Instr::Pow { lhs, rhs, out },
                            TokenKind::Percent => Instr::Mod { lhs, rhs, out },
                            _ => todo!(),
                        },
                    );
                }
                Value::Copy(copy) => self.lower_expr(
                    hir,
                    env,
                    block,
                    self.ast.get(copy.expr),
                    out,
                    ReadKind::Copy,
                ),
                _ => todo!(),
            },
            Expr::Place(place) => self.lower_place(hir, env, block, place, out, read_kind),
        }
    }
}
